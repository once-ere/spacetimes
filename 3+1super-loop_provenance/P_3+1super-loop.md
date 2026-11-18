---
title: "Provenance P_3+1super-loop: the Wolfram Language notebook 3+1super-loop.nb, Part I (the symbols) and Part II (the encyclopedia of Test 2): the library and item [1]"
subtitle: "PROJECT spacetimes, prepared for Patrick Lee Nash, Ph.D."
date: "18 November 2026"
---

# 0. What this document is

This is the provenance of the notebook `3+1super-loop.nb`, which stands in the top folder of the repository of PROJECT spacetimes. A **provenance** is the record of where every part of a piece of work comes from: what was asked, what was done, how it was done, what was computed, which errors were met and how they were removed, and which files hold the evidence. This document describes **Part I** of the notebook, the Wolfram Language form of every symbol of the project (sections 5 to 9), and the finished chapters of **Part II**, the encyclopedia of the programs of Test 2 (sections 10 to 12): chapter II.1, the library of the programs, and chapter II.2, item [1] of Test 2. The remaining items of Part II are added to the notebook in later stages; their provenance will be added to this document at the same time.

The document is written for a physics student who has not seen the work before. Every term and every symbol is defined in words in section 2 before it is used. Nothing in this document depends on any other document: every equation, symbol and definition that it uses is written here.

**Status (18 November 2026).** The notebook was evaluated in Wolfram 15.0.1 (Mac, Apple silicon), in the front end of the application Wolfram.app, by the menu command Evaluation > Evaluate Notebook, in a fresh kernel: 87 input cells, 43 output cells, 37 printed test lines, **0 messages** (no error or warning of any kind), **37 of 37 tests passed** (16 of Part I, 21 of Part II), evaluation time 125 seconds. Two properties of this version are new: every time integration is made by IDA, an integrator of the suite SUNDIALS (section 11.5; no integrator with a fixed step written by hand is used any more, error E20), and every formula that the notebook shows is converted from its TeX by the notebook's own reader of formulas (section 6.2; the TeX importer of the Wolfram Language, which shows 28 of the formulas of the notebook wrongly, is not used any more, error E22).

# 1. The task

The author asked for one stand-alone Wolfram Language notebook, named `3+1super-loop.nb`, extremely well organized, clearly written and thoughtfully commented for a student who is learning the work, with provenance, using only the Wolfram Language, and evaluated in Mathematica (on his own computer, in `/Applications/Wolfram.app`). In his words, restated here:

1. First, read and understand the author's note on how a symbolized variable is created by a program (the technique of section 5).
2. Then port every unique symbol of the Test 2 sub-project, that is, every symbol of the symbol registry of Test 2 (the registry is described in section 7), to the notebook, using only the Wolfram Language.
3. Create non-trivial verification tests that include sums, arithmetic and algebra, and execute them in Mathematica. If errors occur, find out why (also with the help of the internet), fix them, and document the fixes with provenance.
4. Then append to the notebook, as an "encyclopedia", the entire code of Test 2 with all its tests and verifications, ported to the Wolfram Language (Part II; done in this stage: the library and item [1]; the items [2] to [11] follow).
5. Evaluate the notebook after every addition, fix every error at once, describe and discuss the fixes in the provenance, and push every finished item to the repository.
6. (Instruction of 18 November 2026.) Do not integrate in time with the Runge-Kutta method of the fourth order, or with any other integrator of a fixed step written by hand. Use the integrators of the suite SUNDIALS instead: CVODE in the Rust and Python programs, and IDA in the Wolfram Language. Redo every calculation that was made with the former method, and record the error (errors E20 and E21 of section 14; the terms are defined in section 2.7, the method in section 11.5).

Section 15 lists the decisions that were taken where the task left a choice.

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
- **`$PreRead`**: a global variable; if it holds a function, the kernel applies it to the boxes of every input before parsing. It is used in section 14 to record exactly which boxes the front end sends.
- **`SyntaxForm`**: an option of `TagBox` that tells the front end how the box takes part in the grammar of the surrounding input; `SyntaxForm -> "symbol"` makes the front end treat the whole box as one operand, like a symbol.
- **`FrontEndTokenExecute[nb, "EvaluateNotebook"]`**: a command, sent from the kernel to the front end, that performs the menu command Evaluate Notebook for the notebook `nb`.
- ``FrontEnd`UndocumentedTestFEParserPacket[string, True]``: an internal command of the front end (not part of the documented language) that reads a string exactly as typed input is read when a cell is evaluated, and returns the boxes.
- **wolframscript**: the program that runs a Wolfram Language script (a `.wls` file) in a kernel without the window of the application. **`UsingFrontEnd[expr]`** evaluates `expr` with a front end that has no window.
- **Evaluator** (of a notebook): the kernel to which a notebook sends its input; a front end can run several kernels with different names.
- **`.` (`Dot`)**: the product of matrices and vectors, `a . b`. In the grammar of the Wolfram Language the operator `.` binds more strongly than the product written by placing two expressions side by side (`Times`): `x y . z` is read as `x (y . z)`. `Dot` of a number and a matrix is not defined and stays unevaluated (error E15).
- **`KroneckerProduct`**, **`ConjugateTranspose`**, **`ArrayFlatten`**: the Kronecker product of matrices; the complex conjugate of the transpose (the dagger); a matrix assembled from blocks.
- **`BitXor`**, **`BitShiftRight`**, **`Mod`**: the exclusive or of the bits of two integers; the shift of the bits to the right (division by a power of 2, rounded down); the remainder of a division.
- **`MatrixExp`**, **`Eigenvalues`**, **`Norm[m, "Frobenius"]`**: the matrix exponential; the eigenvalues; the square root of the sum of the squared moduli of all entries of a matrix (the Frobenius norm).
- **`SparseArray`**: a matrix stored by its nonzero entries. **`Signature`**: the sign of a permutation, 0 if two entries are equal (`Signature[{i, j, k}]` is the Levi-Civita symbol).
- **`Format`**, **`MessageName[s, "usage"]`** (written `s::usage`): a rule for printing an expression; the usage message that `?s` prints.
- **`ListLinePlot`**, **`ListPlot`**, **`ListLogLogPlot`**, **`Show`**, **`Graphics`**, **`Text`**, **`Line`**, **`LineLegend`**, **`Labeled`**: plots of lists of points (joined, as points, on logarithmic axes); the combination of plots; graphics, text and lines placed at coordinates; a legend; a caption.
- **`SingleLetterItalics`**: an option of the front end; in `TraditionalForm`, the form in which the labels of graphics are typeset, a name of one letter is printed in italics unless the option is `False`.
- **`ImportString[text, "TeX"]`**: the TeX importer of the Wolfram Language, which turns TeX into a notebook with boxes. The builder used it for the formulas in text cells until it was found to show some formulas wrongly (errors E4, E17 and E22); the notebook now converts every formula with its own reader (section 6.2), and the importer is used only by the comparison of that section.
- **`FractionBox[a, b]`**, **`SqrtBox[a]`**, **`RadicalBox[a, n]`**, **`GridBox[rows]`**, **`UnderscriptBox`**, **`UnderoverscriptBox`**, **`FormBox[boxes, TraditionalForm]`**: the boxes of a fraction, of a square root, of a root of the degree `n`, of a table of boxes (a matrix), of a box with another box below it, or below and above it (the limits of a sum); and the statement that boxes are to be shown in `TraditionalForm`.
- **Inline cell**, **`TextData`**, **`RawBoxes`**: the content of a text cell is `TextData[{...}]`, a list of strings, of strings with a style, and of inline cells; an inline cell is a cell that stands inside the text of another cell, for example `Cell[BoxData[boxes], "InlineFormula"]`, the cell of a formula. A cell of the style `DisplayFormula` holds one formula on lines of its own. `RawBoxes[boxes]` is an expression that is displayed as the given boxes.
- **`Pane[expr, width]`**: a region of the given width in which `expr` is displayed; a text inside it is broken into lines of this width.

- **The space of a product**, **`\[InvisibleSpace]`**: the front end shows two neighbouring items of a `RowBox` that are not separated by an operator, such as the number 2 and the letter $x$, with a thin space between them, as it shows a product. `\[InvisibleSpace]` is a character of the Wolfram Language that has no width; if it stands between the two items, the front end adds no space (measured, error E28).
- **`NDSolve`**, **`NDSolveValue`**: the functions that solve differential equations numerically; `NDSolveValue[equations, y, {t, t0, t1}, options]` returns the solution `y`. Options used here: `Method -> "IDA"` chooses the integrator IDA (section 2.7); `PrecisionGoal -> a` and `AccuracyGoal -> b` set the relative and the absolute tolerance of the local error to $10^{-a}$ and $10^{-b}$ (section 11.5); `MaxSteps` limits the number of internal steps; `EvaluationMonitor :> expr` evaluates `expr` every time the equations are evaluated (used to count the evaluations).
- **`InterpolatingFunction`**: the object in which `NDSolve` returns a solution: the values of the solution and of its derivative at the times of a grid, and a rule of interpolation between them. For a solution `f`: `f["Grid"]` is the list of the grid times, `f["ValuesOnGrid"]` the values there, `f[t]` the interpolated value at the time `t`, and `Derivative[1][f]` the derivative.
- **`ListLogPlot`**, **`Epilog`**, **`FrameTicks`**: a plot with a logarithmic vertical axis (the vertical positions of graphics added to it are natural logarithms); graphics drawn over a plot; the tick marks of a framed plot.

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
| $Q_{\rm LL}$ | the electric charge of the model; its eigenvalues are 0 on the quaternion part $\mathbb C\otimes\mathbb H$ of $\mathbb B_{\rm LL}$ and $\pm\tfrac12$ on the spinor charges (in this document only an example of a formula with the fraction one half, section 6.2) |
| $\Gamma_{\rm E}$ | Euler's gamma function; the upright E is a label, not an index (an example in error E23) |
| $\hbar$ | the reduced Planck constant; the registry uses units in which $\hbar$ and the speed of light are 1 |
| $x5$, $x6$, $x7$ | the names of three space coordinates of the eight-dimensional spacetime of the main project; in the super-loop revision they do not occur, and the registry only mentions them in the meaning of $D$ (each is one name, a letter followed by a number, not a product; error E28) |
| $\ell^{\rm K}_{\rm LL}$ | the width of a domain wall (kink) of the pair field (an example of an upright label K) |
| $\mathsf K^3_{\rm LL}$ | the third of the pair matrices $\mathsf K^\iota_{\rm LL}$ of section 2.5 (an example of a sans-serif letter K) |
| $a_{\rm LL}$, $a_{{\rm LL},p}$, $b_{{\rm LL},p}$ | symbols of the demonstrations of section I.2 of the notebook (not part of the registry) |

## 2.5 Symbols of Part II, item [1]

Every symbol in this table is a registered symbol of the project (Part I of the notebook symbolizes all of them; the registry is described in section 7). The meanings are those of the registry, written out here in full, with the formulas used by the programs. The name in the last column is the function or variable of Part II of the notebook that computes the object (code font). Indices: $p,p'=0,1,2,3$ (space-time directions), $j,j',j''=1,2,3$ (space directions), $\iota,\iota',\iota''=1,2,3$ (isospin directions), $k=1,\dots,8$ (the components of the spinor field), $n$ (the number of a mode).

| symbol | meaning | in Part II |
|---|---|---|
| $\sigma^{\rm LL}_0$, $\sigma^{\rm LL}_j$ | the $2\times2$ unit matrix and the Pauli matrices of section 2.2 | `t2sigma[0]` ... `t2sigma[3]` |
| $\tau^{\rm LL}_\iota$ | the same Pauli matrices acting on the isospin index (isospin up, down) | `t2sigma[i]` |
| $\mathsf E_{\rm LL}$ | the real antisymmetric $2\times2$ matrix $i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$ | `t2eps2` |
| $\mathsf 1_2$, $\mathsf 1_4$, $\mathsf 1_8$, $\mathsf 1_{16}$ | the unit matrices of the dimensions 2, 4, 8, 16 | `IdentityMatrix` |
| $\epsilon^{\rm LL}_{\iota\iota'\iota''}$ | the Levi-Civita symbol of section 2.2, also for isospin indices | `t2eps3` |
| $\eta^{\rm LL}_{pp'}$ | the Minkowski metric of section 2.2 | `t2eta[p]` (its diagonal) |
| $\mathbb B_{\rm LL}$; $\mathbf e_0,\dots,\mathbf e_7$ | the bi-octonions, the complex combinations of the eight units $\mathbf e_0=1,\mathbf e_1,\dots,\mathbf e_7$ | |
| $\mathbf a$, $\mathbf b$, $\mathbf c$, $\mathbf d$; $\mathbf x$, $\mathbf y$ | bi-quaternions, the complex combinations of $\mathbf e_0,\dots,\mathbf e_3$ (a bi-quaternion is also a bi-octonion, so the letters $\mathbf x$, $\mathbf y$ of bi-octonions are used for bi-quaternions as well); a bi-octonion is written $\mathbf a+\mathbf b\,\mathbf e_4$ (Cayley-Dickson form) | lists of 4 complex numbers |
| $\xi^{\rm LL}_p(\mathbf b)$ | the complex coefficient of $\mathbf e_p$ in $\mathbf b$: $\mathbf b=\sum_{p=0}^3\xi^{\rm LL}_p(\mathbf b)\mathbf e_p$ | the entries of the list |
| $\bar{\mathbf b}$, $\mathbf b^*$, $\mathbf b^\dagger$ | the conjugations: $\bar{\mathbf b}$ changes the signs of the coefficients of $\mathbf e_1,\mathbf e_2,\mathbf e_3$; $\mathbf b^*$ takes the complex conjugate of every coefficient; $\mathbf b^\dagger=(\bar{\mathbf b})^*$ | `t2bqConj`, `t2bqStar`, `t2bqDag` |
| $M_{\rm LL}(\mathbf b)$ | the $2\times2$ matrix of a bi-quaternion, $M_{\rm LL}(\mathbf b)=\xi^{\rm LL}_0(\mathbf b)\sigma^{\rm LL}_0-i\sum_j\xi^{\rm LL}_j(\mathbf b)\sigma^{\rm LL}_j$, so that $M_{\rm LL}(\mathbf e_0)=\sigma^{\rm LL}_0$, $M_{\rm LL}(\mathbf e_j)=-i\sigma^{\rm LL}_j$; it is multiplicative, $M_{\rm LL}(\mathbf b\mathbf b')=M_{\rm LL}(\mathbf b)M_{\rm LL}(\mathbf b')$, with $M_{\rm LL}(\mathbf b^\dagger)=M_{\rm LL}(\mathbf b)^\dagger$ and $\det M_{\rm LL}(\mathbf b)=N_{\rm LL}(\mathbf b)$ | `t2mq`; inverse `t2bqFrom`; product `t2bqMul` |
| $N_{\rm LL}(\mathbf x)$ | the complex norm, $N_{\rm LL}(\mathbf x)=\sum_p\xi^{\rm LL}_p(\mathbf x)^2$ | |
| $R^{\rm LL}_{\mathbf x}$ | the right multiplication $\mathbf y\mapsto\mathbf y\mathbf x$ | |
| $\mathbb R^{1,3}_{\rm LL}$; $x^p_{\rm LL}$; $x_{\rm LL}$; $\partial_p$ | Minkowski space of $3+1$ dimensions, the Hermitian bi-quaternions $x^0_{\rm LL}\mathbf e_0+i\sum_jx^j_{\rm LL}\mathbf e_j$ with real coordinates $x^p_{\rm LL}$; a point $x_{\rm LL}=(x^0_{\rm LL},\dots,x^3_{\rm LL})$; the partial derivative with respect to $x^p_{\rm LL}$. The coordinates of a Hermitian bi-quaternion $\mathbf x$ are $x^p_{\rm LL}=\tfrac12{\rm tr}\big(\sigma^{\rm LL}_pM_{\rm LL}(\mathbf x)\big)$ | `t2coords` |
| $\mathbf f_p$ | a frame: four Hermitian bi-quaternions $\mathbf f_0,\dots,\mathbf f_3$ (with $N_{\rm LL}(\mathbf f_p,\mathbf f_{p'})=\eta^{\rm LL}_{pp'}$ for an orthonormal frame); the canonical frame is $\mathbf f_0=\mathbf e_0$, $\mathbf f_j=i\,\mathbf e_j$, with $M_{\rm LL}(\mathbf f_p)=\sigma^{\rm LL}_p$, $M_{\rm LL}(\bar{\mathbf f}_0)=\sigma^{\rm LL}_0$, $M_{\rm LL}(\bar{\mathbf f}_j)=-\sigma^{\rm LL}_j$ | `t2canonicalFrame` |
| $\gamma^p_{\rm LL}[\mathbf f]$ | the Dirac matrices of a frame, $\gamma^p_{\rm LL}[\mathbf f]=\begin{pmatrix}0&M_{\rm LL}(\mathbf f_p)\\M_{\rm LL}(\bar{\mathbf f}_p)&0\end{pmatrix}$ ($4\times4$, blocks of $2\times2$) | `t2gammasFromFrame` |
| $\gamma^p_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$, $\alpha^j_{\rm LL}$, $\beta_{\rm LL}$ | the Dirac matrices of the canonical frame (chiral basis), $\gamma^0_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_0\\\sigma^{\rm LL}_0&0\end{pmatrix}$, $\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix}$; the chirality matrix $\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$; the charge-conjugation matrix $C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$; $\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}$; $\beta_{\rm LL}=\gamma^0_{\rm LL}$ | `t2gamma[p]`, `t2gammaCh`, `t2cconj`, `t2alpha[j]`, `t2beta` |
| $\mathfrak l_{\rm LL}$; $J^j_{\rm LL}$, $K^j_{\rm LL}$; $\mathbf u$, $\mathbf u_1,\dots,\mathbf u_5$ | the Lorentz part of the Malcev algebra of the super-loop (the imaginary bi-quaternions); its rotation and boost generators $J^j_{\rm LL}=\mathbf e_j/2$, $K^j_{\rm LL}=i\,\mathbf e_j/2$; elements of $\mathfrak l_{\rm LL}$ (Lorentz generators; the registry lists $\mathbf u$, $\mathbf u'$, $\mathbf u_1$, $\mathbf u_2$, and GS-15 numbers five random generators in the same way) | `t2jgen[j]`, `t2kgen[j]` |
| $\mathsf s_{\rm LL}(\mathbf u)$ | a Lorentz generator acting on the Dirac spinor, $\mathsf s_{\rm LL}(\mathbf u)=\mathrm{diag}\big(-M_{\rm LL}(\mathbf u)^\dagger,M_{\rm LL}(\mathbf u)\big)$ | `t2spinGen` |
| $\mathbf g$; $\tilde{\mathbf g}$ | a unit bi-quaternion ($N_{\rm LL}(\mathbf g)=1$), an element of $SL(2,\mathbb C)$; $\tilde{\mathbf g}=(\mathbf g^\dagger)^{-1}=\mathbf g^*$, with $M_{\rm LL}(\tilde{\mathbf g})=(M_{\rm LL}(\mathbf g)^\dagger)^{-1}$ | `t2randSL2C` (a random one) |
| $\mathsf S_{\rm LL}(\mathbf g)$ | the spinor matrix of the Lorentz transformation $\mathbf g$, $\mathrm{diag}\big(M_{\rm LL}(\tilde{\mathbf g}),M_{\rm LL}(\mathbf g)\big)$ | `t2spinRep` |
| $\mathsf L_{\rm LL}(\mathbf g)$ | the real $4\times4$ Lorentz matrix of $\mathbf x\mapsto\mathbf g\mathbf x\mathbf g^\dagger$: $[\mathsf L_{\rm LL}(\mathbf g)]_{pp'}=\tfrac12{\rm Re}\,{\rm tr}\big(\sigma^{\rm LL}_pM_{\rm LL}(\mathbf g)\sigma^{\rm LL}_{p'}M_{\rm LL}(\mathbf g)^\dagger\big)$ | `t2lmat` |
| $\mathcal V_{\rm LL}(\mathbf u)$ | the vector action of a Lorentz generator, $\mathbf x\mapsto\mathbf u\mathbf x+\mathbf x\mathbf u^\dagger$, a real $4\times4$ matrix with the entries $\mathcal V_{\rm LL}(\mathbf u)^p{}_{p'}$ | `t2vecAction` |
| $\Psi_{\rm LL}$; $\Psi_{{\rm LL},k}$ | the Dirac isodoublet of the spinor charges, 8 components: components 1 and 2 are the first Dirac component with isospin up and down, 3 and 4 the second, 5 and 6 the third, 7 and 8 the fourth; a $4\times4$ Dirac matrix acts as its Kronecker product with $\sigma^{\rm LL}_0$ on the right, a $2\times2$ isospin matrix as the Kronecker product of $\mathsf 1_4$ with it | `t2lift4`, `t2lift2` |
| $T^\iota_{\rm LL}$ | the isospin generators on the eight components, $\mathsf 1_4\otimes\tau^{\rm LL}_\iota/2$ | `t2tiso[i]` |
| $\mathsf K^\iota_{\rm LL}$, $\mathsf K'^\iota_{\rm LL}$ | the $8\times8$ pair matrices of the spin-zero channels: scalar $\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$, pseudoscalar $\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ | `t2kpair[i]`, `t2kpairPs[i]` |
| $\vec d_{\rm LL}$, $d^\iota_{\rm LL}$; $\Delta_{\rm LL}$ | the order parameter, a complex isospin vector $(d^1_{\rm LL},d^2_{\rm LL},d^3_{\rm LL})$; the gap $\Delta_{\rm LL}=\lvert\vec d_{\rm LL}\rvert$ of a unitary state | |
| $\mathsf P_{\rm LL}$ | the pairing block $\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger$ | `t2pairPotential` |
| $\mathsf U_{\rm LL}$; $\theta^\iota_{\rm LL}$; $\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})$; $\mathbf v$ | an isospin rotation, a matrix of $SU(2)$, $\mathsf U_{\rm LL}=\exp\big(i\sum_\iota\theta^\iota_{\rm LL}\tau^{\rm LL}_\iota/2\big)$ with the three angles $\theta^\iota_{\rm LL}$; the rotation of an isospin vector, $\mathsf U_{\rm LL}\tau^{\rm LL}_\iota\mathsf U_{\rm LL}^\dagger=\sum_{\iota'}[\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})]_{\iota'\iota}\tau^{\rm LL}_{\iota'}$; the unit real quaternion with $\mathsf U_{\rm LL}=M_{\rm LL}(\mathbf v)$ | `t2su2`, `t2adjoint` |
| $\theta^{\rm F}_{\rm LL}$ | the phase of a transformation of the fermion number, $\Psi_{\rm LL}\mapsto\exp(i\theta^{\rm F}_{\rm LL})\Psi_{\rm LL}$ | |
| $\mathcal U_{\rm LL}$ | a local gauge transformation by $SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$, $\mathcal U_{\rm LL}=\exp(i\theta^{\rm F}_{\rm LL})\,\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf U_{\rm LL}$, with $\mathbf g$, $\mathsf U_{\rm LL}$, $\theta^{\rm F}_{\rm LL}$ functions of $x_{\rm LL}$ | `caluAt` |
| $A^\iota_{{\rm LL},p}$, $A^{\rm F}_{{\rm LL},p}$; $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ | the gauge potentials of the isospin group and of the fermion number (real fields); their couplings | `aAt`, `afAt`; `gv`, `gf` |
| $\mathsf a_{{\rm LL},p}$; $\vec A^{\rm v}_{{\rm LL},p}$; $\mathbf t^{\rm A}_{{\rm LL},p}$ | the isospin potential as a Hermitian $2\times2$ matrix, $\sum_\iota A^\iota_{{\rm LL},p}\tau^{\rm LL}_\iota/2$; as a vector $(A^1_{{\rm LL},p},A^2_{{\rm LL},p},A^3_{{\rm LL},p})$; as a real imaginary quaternion, $\sum_\iota A^\iota_{{\rm LL},p}\mathbf e_\iota/2$, with $M_{\rm LL}(\mathbf t^{\rm A}_{{\rm LL},p})=-i\,\mathsf a_{{\rm LL},p}$ | |
| $\mathbf u^\omega_{{\rm LL},p}$; $\omega^j_{{\rm LL},p}$, $\varpi^j_{{\rm LL},p}$ | the Lorentz connection, $\mathbf u^\omega_{{\rm LL},p}=\sum_j\big(\omega^j_{{\rm LL},p}J^j_{\rm LL}+\varpi^j_{{\rm LL},p}K^j_{\rm LL}\big)$, with real rotation and boost components; $\mathbf u^{\omega*}_{{\rm LL},p}$ is its complex conjugate | `lorentzElem` |
| $\Gamma^{\rm LL}_p$ | the spin connection, $\Gamma^{\rm LL}_p=\mathsf s_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})\otimes\sigma^{\rm LL}_0$ | `gammaAt` (its $4\times4$ part) |
| $\mathsf A_{{\rm LL},p}$ | the gauge matrix of the internal symmetries, $g^{\rm v}_{\rm LL}\sum_\iota A^\iota_{{\rm LL},p}T^\iota_{\rm LL}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}\mathsf 1_8$ (Hermitian) | `gauge8` |
| $\nabla^{\rm LL}_p$, $\nabla^{{\rm LL}*}_p$ | the gauge-covariant derivative $\partial_p+\Gamma^{\rm LL}_p-i\mathsf A_{{\rm LL},p}$, and that of a field that transforms like $\Psi^*_{\rm LL}$, $\partial_p+(\Gamma^{\rm LL}_p)^*+i(\mathsf A_{{\rm LL},p})^*$ | `nabla` |
| $\mathbf D_{\rm LL}$ | the bi-quaternion of covariant derivatives $\sum_p\mathbf f_p\nabla^{\rm LL}_p$; $\sum_p\gamma^p_{\rm LL}\nabla^{\rm LL}_p=\begin{pmatrix}0&M_{\rm LL}(\mathbf D_{\rm LL})\\M_{\rm LL}(\bar{\mathbf D}_{\rm LL})&0\end{pmatrix}$ | |
| $\mathsf F^\iota_{{\rm LL},pp'}$, $\mathsf F^{\rm F}_{{\rm LL},pp'}$, $\mathsf R^{\rm LL}_{pp'}$ | the field strengths and the curvature: $\mathsf F^\iota_{{\rm LL},pp'}=\partial_pA^\iota_{{\rm LL},p'}-\partial_{p'}A^\iota_{{\rm LL},p}+g^{\rm v}_{\rm LL}\sum_{\iota',\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}A^{\iota'}_{{\rm LL},p}A^{\iota''}_{{\rm LL},p'}$, $\mathsf F^{\rm F}_{{\rm LL},pp'}=\partial_pA^{\rm F}_{{\rm LL},p'}-\partial_{p'}A^{\rm F}_{{\rm LL},p}$, $\mathsf R^{\rm LL}_{pp'}=\partial_p\Gamma^{\rm LL}_{p'}-\partial_{p'}\Gamma^{\rm LL}_p+\Gamma^{\rm LL}_p\Gamma^{\rm LL}_{p'}-\Gamma^{\rm LL}_{p'}\Gamma^{\rm LL}_p$ | |
| $\psi'_{\rm LL}(\mathbf y)$ | the Weyl isodoublet of a spinor charge $\mathbf y\mathbf e_4$, the $2\times2$ matrix $\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\mathbf y)^T$ (rows: Weyl index; columns: isospin up, down) | `t2psiPrime` |
| $\mathbf b^+_{\rm LL}$, $\mathbf b^-_{\rm LL}$; $\Psi_{\rm LL}[\mathbf b^-_{\rm LL},\mathbf b^+_{\rm LL}]$ | the two spinor charges $\mathbf b^\pm_{\rm LL}\mathbf e_4$ of the Dirac isodoublet: its upper block (components 1 to 4, chirality $-1$) is $\psi'_{\rm LL}(\mathbf b^-_{\rm LL})$, its lower block (components 5 to 8, chirality $+1$) is $\psi'_{\rm LL}(\mathbf b^+_{\rm LL})$ | `t2diracFromCharges` |
| $\psi^{\rm p}_{\rm LL}$, $\psi^{\rm h}_{\rm LL}$; $\psi^{\rm p}_{{\rm LL},k}$ | the particle and the hole component of a mode of the mean-field equations, eight complex functions each; $\psi^{\rm p}_{\rm LL}$ transforms like $\Psi_{\rm LL}$, $\psi^{\rm h}_{\rm LL}$ like $\Psi^*_{\rm LL}$; a component | |
| $\vec q_{\rm LL}$, $q_{\rm LL}$, $q^j_{\rm LL}$; $m_{\rm LL}$, $\mu_{\rm LL}$, $G_{\rm LL}$ | a momentum, its length and its components; the mass of the spinor charges, the chemical potential, the coupling of the contact interaction | `q`; `mass`, `mu` |
| $\varepsilon_{\rm LL}$, $E^\pm_{\rm LL}$ | the energy $(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$; the quasi-particle energies of the scalar channel, $E^\pm_{\rm LL}=\big((\varepsilon_{\rm LL}\pm\mu_{\rm LL})^2+\Delta_{\rm LL}^2\big)^{1/2}$ | |
| $\mathsf h_{\rm LL}(\vec q_{\rm LL})$; $\mathsf H_{\rm LL}(\vec q_{\rm LL})$ | the Hamiltonian matrix of a plane wave, $\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$; the $16\times16$ Bogoliubov-de Gennes matrix of section 12.1 without gauge fields | |
| $\mathcal H^{\rm DB}_{\rm LL}$, $h^\nabla_{\rm LL}$; $\mathcal H^{\rm DB}_{\rm LL}(\vec q_{\rm LL})$ | the Dirac-Bogoliubov-de Gennes operator (section 12.1) and the covariant Dirac Hamiltonian $h^\nabla_{\rm LL}=-i\sum_j\alpha^j_{\rm LL}\nabla^{\rm LL}_j+\beta_{\rm LL}m_{\rm LL}$; its $16\times16$ matrix on a plane wave with constant fields | `hq` |
| $E^{\rm qp}_{{\rm LL},n}$ | the energy of a numbered mode, an eigenvalue of $\mathcal H^{\rm DB}_{\rm LL}$ | |
| $\Pi^\iota_{\rm LL}$, $\Pi^\iota_{\rm LL}(x_{\rm LL})$ | the pair operators, $\Psi_{\rm LL}^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}$ (at a point) | `t2anomalous` (lattice) |
| $\ell^{\rm x}_{\rm LL}$, $\ell^{\rm t}_{\rm LL}$, $r^{\rm W}_{\rm LL}$ | the lattice spacing; a time step of the lattice equations (the interval between the output times of an integration by CVODE or by IDA, which choose their own internal steps; in the first version of Test 2 a fixed step); the coefficient of the Wilson term | `"ell"`; the spacing of `touts13`; `"wilson"` |
| $n^{\rm x}_{\rm LL}$, $n^{\rm y}_{\rm LL}$, $n^{\rm z}_{\rm LL}$; $\vec s_{\rm LL}$, $s^j_{\rm LL}$; $\vec\delta^j_{\rm LL}$; $n^{\rm act}_{\rm LL}$ | the numbers of lattice sites in the three directions; a site and its integer coordinates (counted from 0); the step to the next site in the direction $j$; the number of directions with more than one site | `"n"`; `t2siteCoords` |
| $\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})$ | the link matrix from $\vec s_{\rm LL}$ to $\vec s_{\rm LL}+\vec\delta^j_{\rm LL}$, $\exp\big(-i\ell^{\rm x}_{\rm LL}(g^{\rm v}_{\rm LL}\mathsf a_{{\rm LL},j}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\sigma^{\rm LL}_0)\big)$ | `t2link` |
| $\mathcal H^{\rm lat}_{\rm LL}$, $\varepsilon^{\rm lat}_{\rm LL}$, $B^{\rm H}_{\rm LL}$ | the lattice form of $\mathcal H^{\rm DB}_{\rm LL}$ (16 rows per site), the energy of a free particle on the lattice, and the bound on the largest eigenvalue magnitude of $\mathcal H^{\rm lat}_{\rm LL}$ (formula in section 11.3) | `t2apply`, `t2dense`; `t2epsLat`; `t2normBound` |
| $\mathsf h^{\rm lat}_{\rm LL}(\vec q_{\rm LL})$ | the $4\times4$ free lattice operator on a plane wave, $\sum_j\alpha^j_{\rm LL}\sin(q^j_{\rm LL}\ell^{\rm x}_{\rm LL})/\ell^{\rm x}_{\rm LL}+\beta_{\rm LL}\big(m_{\rm LL}+(r^{\rm W}_{\rm LL}/\ell^{\rm x}_{\rm LL})\sum_j(1-\cos(q^j_{\rm LL}\ell^{\rm x}_{\rm LL}))\big)$, sums over the active directions | `t2hLatPlane` |
| $\mathsf G^{\rm site}_{\rm LL}$, $\mathsf X^{\rm ph}_{\rm LL}$ | the gauge transformation of the lattice equations, $\exp(i\theta^{\rm F}_{\rm LL})\mathsf U_{\rm LL}$ at each site on the isospin index of the particle component and its complex conjugate on the hole component; the exchange of the particle and the hole component, $\begin{pmatrix}0&\mathsf 1_8\\\mathsf 1_8&0\end{pmatrix}$ | `gbig`; `swap`, `xm` |
| $x^0_{\rm LL}$ | the time: the component 0 of the coordinates $x^p_{\rm LL}$; $\partial_0$ is the derivative with respect to it | `touts13`, the argument of `wExact13` |
| $w^{\rm lat}_{\rm LL}$, $w^{\rm cmp}_{\rm LL}$; $n^{\rm cpx}_{\rm LL}$ | a Nambu mode on the lattice written as one complex vector (the 16 components of $\psi^{\rm p}_{\rm LL}$ and $\psi^{\rm h}_{\rm LL}$ on every site, the sites one after the other), so that the lattice equations read $i\,\partial_0w^{\rm lat}_{\rm LL}=\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL}$; one complex component of it; the number of its complex components, 16 times the number of sites | `w0IDA13`, the entries of `"states"`; `t2dim[lat]` |
| $r^{\rm tol}_{\rm LL}$, $a^{\rm tol}_{\rm LL}$ | the relative and the absolute tolerance of a time integration (section 2.7): the tolerance of a real unknown $y$ of IDA is $r^{\rm tol}_{\rm LL}\lvert y\rvert+a^{\rm tol}_{\rm LL}$; the original programs give CVODE the tolerance $r^{\rm tol}_{\rm LL}\lvert w^{\rm cmp}_{\rm LL}\rvert+a^{\rm tol}_{\rm LL}$ for the real and for the imaginary part of a complex component | `rtol`, `atol` |
| $h^{\rm IDA}_{\rm LL}$; $N^{\rm st}_{\rm LL}$, $N^{\rm ev}_{\rm LL}$ | the length of an internal step of IDA (chosen by IDA); the number of internal steps of an integration and the number of evaluations of its right side $-i\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL}$ | `"hmax"`, `"stepAt"`; `"steps"`, `"evaluations"` |
| $B^{\rm loc}_{\rm LL}$, $\delta^{\rm loc}_{\rm LL}$ | the local error that the tolerances allow in one internal step, $r^{\rm tol}_{\rm LL}\lVert w^{\rm lat}_{\rm LL}\rVert+a^{\rm tol}_{\rm LL}\sqrt{2n^{\rm cpx}_{\rm LL}}$ for IDA and $\sqrt2\,(r^{\rm tol}_{\rm LL}\lVert w^{\rm lat}_{\rm LL}\rVert+a^{\rm tol}_{\rm LL}\sqrt{n^{\rm cpx}_{\rm LL}})$ for CVODE ($\lVert\cdot\rVert$: the norm of a vector, the square root of the sum of the squared moduli of its components); the exact local error of an internal step (section 2.7) | `bloc13`; `t2localErrorsIDA` |
| $B^{\rm err}_{\rm LL}$; $\delta^{\rm rel}_{\rm LL}$, $w^{\rm ex}_{\rm LL}$ | the accumulated local error that the tolerances allow, $N^{\rm st}_{\rm LL}B^{\rm loc}_{\rm LL}$; the relative deviation of a computed state from the exact solution, $\lVert w^{\rm lat}_{\rm LL}-w^{\rm ex}_{\rm LL}\rVert/\lVert w^{\rm ex}_{\rm LL}\rVert$; the exact solution of the same equations | `berr13`; `dev13`, `maxdev13`; `wExact13` |
| $E^{\rm max}_{\rm LL}$, $r^{\rm der}_{\rm LL}$, $B^{\rm out}_{\rm LL}$ | the largest modulus of the eigenvalues $E^{\rm qp}_{{\rm LL},n}$ of $\mathcal H^{\rm lat}_{\rm LL}$; the norm of the difference, at the end of an internal step of IDA, between the derivative that `NDSolve` stores and $-i\mathcal H^{\rm lat}_{\rm LL}$ applied to the stored state; the bound of the deviation of a state that `NDSolve` returns between the ends of an internal step (section 11.5) | `eMax13`; `"residualAt"`; `bout13` |
| $SL(2,\mathbb C)$, $SU(2)_{\rm v}$, $U(1)^{\rm F}_{\rm LL}$, $\mathfrak{so}(3,1)$ | the Lorentz group (unit bi-quaternions), the isospin group, the group of the fermion number, the Lie algebra of the Lorentz group | |

## 2.6 Abbreviations of this document that are not registered symbols

One letter is used, in a few places only, as an abbreviation; it is defined where it is used and does not coincide with a registered symbol (the registry has no symbol written with this letter alone):

- $\kappa$ (sections 12.2 GS-10 and GS-16, and the notebook text of GS-16): the integer that labels the allowed momenta of a periodic chain: $q^1_{\rm LL}=2\pi\kappa/(n^{\rm x}_{\rm LL}\ell^{\rm x}_{\rm LL})$ for a chain of $n^{\rm x}_{\rm LL}$ sites along $x^1_{\rm LL}$, and $q^3_{\rm LL}=2\pi\kappa/(n^{\rm z}_{\rm LL}\ell^{\rm x}_{\rm LL})$ for a chain along $x^3_{\rm LL}$.
- Variables of the programs are written in code font (for example `kv`, `waves`, `ulist`, `w`, `m`, `hr`, `hi`, `frac`); they are names of the Wolfram Language code, not symbols of the physics.

## 2.7 Terms of the time integration

- **Ordinary differential equation**, **initial value problem**: an equation that gives the derivative of the unknowns with respect to one variable (here the time $x^0_{\rm LL}$) as a function of the unknowns; together with the values of the unknowns at the initial time it fixes the solution. A **differential-algebraic equation** may contain, in addition, equations without derivatives.
- **Integrator**, **internal step**, **output times**: an integrator is a program that computes the solution of an initial value problem step by step in time; an internal step is one such step, whose length the integrator chooses itself; the output times are the times at which the user asks for the solution, and they need not be ends of internal steps.
- **Right side**: the function that gives the derivative, here $-i\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL}$; the number of its evaluations measures the cost of an integration.
- **Local error**, **deviation**: the local error of an internal step is the difference between the computed state at the end of the step and the exact solution that starts from the computed state at the beginning of the step (the error that this one step adds). The deviation (also called the global error) is the difference between the computed state and the exact solution of the initial value problem.
- **Tolerances**, **weighted root-mean-square norm**, **error control**: a relative tolerance $r^{\rm tol}_{\rm LL}$ and an absolute tolerance $a^{\rm tol}_{\rm LL}$ give every real unknown $y$ the tolerance $r^{\rm tol}_{\rm LL}\lvert y\rvert+a^{\rm tol}_{\rm LL}$. The weighted root-mean-square norm of a vector of errors is the square root of the mean of the squares of the errors, each divided by the tolerance of its unknown. An integrator with error control computes, in every step, an estimate of the local error and accepts the step only if this norm of the estimate is at most 1; otherwise it repeats the step with a shorter length.
- **SUNDIALS**: the SUite of Nonlinear and DIfferential/ALgebraic equation Solvers of the Lawrence Livermore National Laboratory. **CVODE** is its integrator of ordinary differential equations; the original programs use it with the **Adams-Moulton formulas** of the orders 1 to 12 (formulas that express the value at the end of a step by the derivatives at the end of this step and of the steps before it). **IDA** is its integrator of differential-algebraic equations; the Wolfram Language contains it, and the notebook uses it.
- **Backward differentiation formula** of the order $k$: a formula that expresses the derivative at the end of a step by the values of the solution at the end of this step and at the $k$ grid times before it. Because the unknown value at the end of the step enters the formula, every step requires the solution of a system of equations, which IDA solves by an iteration, **Newton's method**; the iteration is stopped when its corrections are small compared with the tolerances.
- **Cubic Hermite interpolation**: between two times, the polynomial of the third degree that has given values and given derivatives at the two times.
- **Unitary evolution**: for a Hermitian matrix $\mathcal H^{\rm lat}_{\rm LL}$ the exact solution of the lattice equations over a time $h$ is the initial vector multiplied by the unitary matrix $\exp(-ih\mathcal H^{\rm lat}_{\rm LL})$; it changes the norm of no vector, and therefore not the norm of the difference of two solutions.
- **Runge-Kutta method of the fourth order** (RK4): a classical method that advances the solution by a step of a fixed length from four evaluations of the right side, without an estimate of the error. The first version of Test 2 and of the notebook used it; it is used nowhere any more (error E20).

# 3. Results in brief

| quantity | value |
|---|---|
| entries of the registry | 204, in 9 groups |
| TeX segments (text between two dollar signs) | 452 |
| TeX items (single symbols) | 467 |
| label entries (no TeX) | 2 (entries 86 and 87) |
| TeX items of a symbolized kind | 445 |
| different boxes, i.e. symbolized symbols | 440 |
| tests of Part I | 16 (S-0 to S-15), all passed |
| formulas that the notebook shows, all converted by its own reader of formulas | 1237: 383 in texts, 2 display formulas, 852 in the meanings of the registry; 791 different formulas; no problem reported for any of them |
| formulas that the TeX importer of the Wolfram Language shows wrongly (and the own reader correctly) | 28 of the 791; 24 of them stood, wrongly shown, in the notebook as committed before 18 November 2026 |
| original programs rewritten in Part II so far | the library t2core (rng.rs, c64.rs, cmat.rs, model.rs, lattice.rs; the time integration of cvode.rs is made by IDA) and the program t2_fields (item [1] of Test 2) |
| checks of Part II so far | 21 (T2-SYM, T2-RNG, T2-LINALG, T2-IDA, GS-1 to GS-16, T2-DATA-1), all passed |
| time integration of GS-13 by IDA, $r^{\rm tol}_{\rm LL}=10^{-8},10^{-10},10^{-12}$: largest relative deviation from the exact solution | $7.1\times10^{-6}$, $9.3\times10^{-8}$, $4.4\times10^{-11}$ (CVODE in the original program: $1.2\times10^{-6}$, $1.7\times10^{-7}$, $1.7\times10^{-10}$) |
| components of families typed in the notebook | 11, all atomic symbols (T2-SYM) |
| messages during the evaluation | 0 |
| evaluation time (Wolfram 15.0.1, Mac, Apple silicon) | 125 s |
| printed form of the evaluated notebook | 205 pages |

# 4. How to use the notebook

1. Open `3+1super-loop.nb` in Wolfram (version 15 or later; the notebook was evaluated with 15.0.1).
2. Choose Evaluation > Evaluate Notebook. Every cell is evaluated in order. Nothing outside the notebook is read: the registry, the converter and all tests are inside it.
3. Every test prints one line that begins with PASS (green) or FAIL (red), its name, the statement that it checks, and the numbers that decide it. Section I.9 of the notebook lists the tests of Part I in a table, and its last cell gives `{"tests" -> 16, "passed" -> 16}`; the subsection "Summary of item [1]" at the end of chapter II.2 lists the 21 checks of Part II. In Part II the numbers of a check are followed, in parentheses, by those of the original report.
4. After the evaluation every registered symbol can be typed in its printed form; `?symbol` prints its meaning.

The notebook as stored in the repository is already evaluated, so its results can be read without evaluating it. The folder `3+1super-loop_provenance/export` holds the printed form of the evaluated notebook as a PDF file (205 pages; 22 of them, pages 173 to 194, are the input cell with the numbers of the data files of item [1], decision of section 15).

# 5. The technique: symbolized symbols created by a program

## 5.1 The basic call

The author's note on creating a symbolized variable by a program describes the technique that the notebook uses: the Notation package is loaded, and `Symbolize` is called with the box of the symbol wrapped in `ParsedBoxWrapper`:

```
Needs["Notation`"];
Symbolize[ParsedBoxWrapper[SubscriptBox["a", "LL"]], WorkingForm -> StandardForm];
```

After this call the two-dimensional input $a_{\rm LL}$ is one new symbol (its internal name is `a\[UnderBracket]Subscript\[UnderBracket]LL`, where `\[UnderBracket]` is the separator character that the package puts into names). The notebook checks: `MakeExpression[SubscriptBox["a", "LL"], StandardForm]` gives `HoldComplete[a_LL]` (displayed with the printed form), its head is `Symbol` and it is an atom; the text input `Subscript[a, LL]`, in contrast, is a compound expression with head `Subscript`.

## 5.2 The working form must be given explicitly

Without the option `WorkingForm -> StandardForm` the package asks the front end for the format of the notebook in which the input was typed (`InputNotebook[]`). When a notebook is evaluated by a program, or when no notebook is selected, the answer is `$Failed`, and the package stores its reading rule for the format `$Failed`: the symbol is created but its box is never recognized in input. This was found from the source code of the package (the default value is taken from `AbsoluteCurrentValue[InputNotebook[], {CommonDefaultFormatTypes, "Output"}]`) and is error E1 of section 14.

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

# 6. The converters from TeX to boxes

The notebook contains two readers of TeX, both written in the Wolfram Language: the reader of symbols (section 6.1), which makes the boxes that are symbolized, and the reader of formulas (section 6.2), which makes the boxes of the complete formulas that the notebook shows in its texts. The TeX importer of the Wolfram Language (`ImportString[..., "TeX"]`) is used by neither: it loses the alphabets of styled letters (errors E4 and E17) and shows some formulas wrongly (error E22).

## 6.1 The reader of symbols

The registry stores every symbol in TeX. The reader of exactly this kind of TeX is the function `SL31TeXToBoxes` of the package `` SL31` `` (file `notebook_source/sl31_core.wl`). Its rules:

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

## 6.2 The reader of formulas

**Why it exists.** The texts of the notebook, and the meanings of the registry that section I.8 of the notebook prints, contain complete formulas in TeX, written between dollar signs: 1237 formulas in all, 791 of them different. In the versions of the notebook before 18 November 2026 the TeX importer of the Wolfram Language converted them. The importer breaks a rule of TeX. In TeX the argument of a command, if it is not written in braces, is the single token that follows the command (a token is one command or one other character): `\tfrac12\ell^{\rm x}_{\rm LL}` is the fraction one half, times $\ell^{\rm x}_{\rm LL}$. The importer instead takes everything up to the end of the group as the denominator, and everything before the fraction as part of the numerator. It shows

$$\tfrac12\ell^{\rm x}_{\rm LL}\ \ \text{as}\ \ \frac{1}{2\ell^{\rm x}_{\rm LL}},\qquad Q_{\rm LL}=-\tfrac12\ \ \text{as}\ \ \frac{Q_{\rm LL}=-1}{2},\qquad \cos\big(\tfrac12\ell^{\rm x}_{\rm LL}g^{\rm v}_{\rm LL}\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert\big)\ \ \text{as}\ \ \cos\Big(\frac{1}{2\ell^{\rm x}_{\rm LL}g^{\rm v}_{\rm LL}\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert}\Big),$$

and it shows the command `\hbar` (the reduced Planck constant) as nothing. A formula that is shown wrongly teaches a wrong statement. The importer shows 28 of the 791 formulas of the present notebook wrongly; 24 of them stood, wrongly shown, in the notebook as it was committed before 18 November 2026 (error E22). The notebook therefore converts every formula itself. The reader of formulas is the file `notebook_source/sl31_formula.wl`, which section I.3 of the notebook contains in one input cell (functions `SL31TeXFormula`, `SL31TeXFormulaProblems`, `SL31FormulaCell`, `SL31TextData`, `SL31TextFormulas`, `SL31TextBoxes` and `SL31FormulaReading` of the package `` SL31` ``). The builder uses it for every text cell, and section I.8 of the notebook uses it for every meaning.

**The method** is recursive descent, as in the reader of symbols:

1. The TeX is cut into tokens: a command (`\frac`, `\Delta`), a control symbol (a backslash followed by one character that is not a letter), white space, or one other character. Digits stay single tokens, because an argument without braces is ONE token: `\tfrac12` has the numerator 1 and the denominator 2.
2. A sequence of atoms is read from left to right. An atom is a letter, a digit, the character of a command, a group in braces, a part between matching delimiters, or a command together with its arguments; after the atom come its primes, its superscript and its subscript.
3. Every letter carries its alphabet. Neighbouring digits form one number, and neighbouring letters of one alphabet one string, so that they are set without a space between them, as TeX sets them. A number that directly follows a letter is joined to the letter by the character `\[InvisibleSpace]`, so that the two are set without a space as well: such a pair is the name of one quantity (the coordinates $x5$, $x6$, $x7$ of the main project), not a product (error E28).
4. A sign that stands first, or after an operator, a relation, an opening delimiter or a comma, is a prefix sign and is joined to the item after it. The name of a function and a large operator are separated from a neighbouring ordinary item by a thin space, as in TeX.

The rules in detail:

| TeX | box |
|---|---|
| a letter | `StyleBox["x", "TI"]` (the style of an italic letter); `SL` gives `StyleBox["SL", "TI"]` |
| digits | where a sequence is read, the whole number (`10^{-11}` is the number 10 with a superscript); as an argument without braces, the single digit |
| a letter directly followed by a number, `x5` | `RowBox[{StyleBox["x", "TI"], "\[InvisibleSpace]", "5"}]`: one name, set without a space; a number before a letter, `5x`, stays `RowBox[{"5", StyleBox["x", "TI"]}]`, a product with the thin space of the front end |
| `\frac{a}{b}`, `\tfrac12`, `\tfrac{15}8`, `\tfrac i2` | `FractionBox[a, b]`; each argument is a group in braces or one token |
| `\sqrt{a}`, `\sqrt2`, `\sqrt[n]{a}` | `SqrtBox[a]`, `RadicalBox[a, n]` |
| `x^{..}`, `x_{..}`, primes | `SuperscriptBox`, `SubscriptBox`, `SubsuperscriptBox`; a prime is the superscript `"\[Prime]"` |
| `\sum`, `\prod` with scripts | `UnderoverscriptBox` (the scripts are limits); `\int` keeps its scripts at the side |
| `{\rm text}`, `\mathrm{..}`, `\text{..}` | `StyleBox["text", FontSlant -> "Plain"]` (upright); `\rm` holds to the end of its group |
| `\mathbf b`, `\mathsf N` | `StyleBox["b", FontWeight -> "Bold", FontSlant -> "Plain"]`, `StyleBox["N", FontFamily -> "Helvetica", FontSlant -> "Plain"]` |
| `\mathbb R`, `\mathcal L`, `\mathfrak m` | the double-struck, script and Gothic characters of the Wolfram Language |
| `\vec d`, `\hat x`, `\tilde x`, `\bar x`, `\overline{..}` | `OverscriptBox` with the accent character |
| `( .. )`, `[ .. ]`, `\{ .. \}`, `\lvert .. \rvert`, `\lVert .. \rVert`, `\langle .. \rangle` | a matching pair becomes `RowBox[{open, content, close}]` and takes the scripts that follow it; a delimiter without its partner, as in a half-open interval, is an ordinary character |
| `\begin{pmatrix} a&b\\c&d \end{pmatrix}` | `RowBox[{"(", GridBox[{{a, b}, {c, d}}], ")"}]` |
| `\cos`, `\sin`, `\exp`, ... | the upright name; a thin space separates it from a neighbouring ordinary item |
| `\,` and the other spaces of TeX | thin, medium and thick space characters; the negative thin space gives nothing |
| `\big`, `\Big`, `\left`, `\right` | left out (they only choose the size of a delimiter) |
| `\le`, `\in`, `\to`, `\mapsto`, `\times`, `\otimes`, `\hbar`, ... | the characters of the Wolfram Language |
| a command that the reader does not know | shown as it is written, and reported as a problem |

**Names and products.** TeX sets neighbouring letters and numbers without a space; the front end puts the thin space of a product between two neighbouring items of a row of boxes (section 2.1). The notebook keeps this space between the factors of a product (it shows the product of 2 and $\pi$ with a thin space between the two, as the Wolfram Language shows a product), but a name must not be torn apart: the letters of one alphabet are one string (error E26), and a number that directly follows a letter belongs to the letter (error E28). In the 791 formulas of the notebook the second rule applies to one formula, the three coordinates $x5,x6,x7$ in the meaning of the number of space dimensions $D$.

**Nothing is passed over in silence.** `SL31TeXFormulaProblems[tex]` lists every unknown command, missing argument and unmatched brace of a formula. The builder turns every problem of a text cell into a warning, and a build is accepted only with 0 warnings; test S-15 requires the empty list for every formula of the registry.

**The alphabets and the typeface.** Every letter gets its alphabet by an explicit style, so that it is shown in the same way in a text cell and in an output cell (in an output cell the front end would otherwise set every single letter in italics, section 2.1, `SingleLetterItalics`). A formula is set in the typeface Times, one tenth larger than the text around it (the letters of Times are smaller than those of the typeface of the text cells at the same size). In Times an italic letter of a variable, an upright roman letter of a label and a sans-serif letter (set in Helvetica) differ visibly: the label K of $\ell^{\rm K}_{\rm LL}$, the matrix $\mathsf K^3_{\rm LL}$ and the generator $K^j_{\rm LL}$ are three different letters. This matters for the project: in the formulas of the notebook 16 letters occur both as an upright label and as a sans-serif symbol (A, B, E, F, G, H, K, L, N, Q, R, W, Y, a, h, s), and 13 both as an upright label and as a bold symbol (a, b, c, f, g, h, t, u, v, w, x, y, z). In the earlier versions the builder wrapped the cell that the importer returned into a second cell; the formulas of the text cells were then set in the monospaced typeface of code, in which an upright letter and a sans-serif letter are hard to tell apart.

**Texts.** `SL31TextData[text]` converts a text with formulas between dollar signs, code between backquotes and bold text between double asterisks into the content of a text cell; the patterns are tried from left to right, so that a dollar sign inside code does not begin a formula. `SL31TextBoxes[text, options]` makes an inline text cell of it; section I.8 of the notebook uses it for the meanings, so that a meaning is one paragraph with its formulas, broken into lines by the front end (error E23). A text cell that holds nothing but one formula is written in the source as a cell of the style `DisplayFormula` (error E24).

**The reading on one line.** `SL31FormulaReading[boxes]` writes boxes on one line: a fraction as `frac{a}{b}`, a root as `sqrt{a}`, scripts as `_{..}` and `^{..}`, an accent as `over{a}{accent}`, a matrix as `grid[a&b;c&d]`, a bold or sans-serif letter as `<bf:b>` or `<sf:N>`; spaces, the italic and upright styles and the grouping of rows are left out. Two conversions of a formula show the same formula exactly if their readings are equal.

**The comparison with the importer** tests both programs against each other (script `tools/formula_compare.wls`; the list of the formulas, `notebook_source/formula_inventory.wl`, is written by `tools/formula_inventory.py` from the sources of the notebook and from the registry, with the same rules by which the builder finds the formulas; output `logs/E22_formula_compare_report.txt`). Every one of the 791 different formulas is converted three times: by the own reader, by the importer with the alphabets protected (the way of the builder before, error E17), and by the importer alone (the way of section I.8 before). Six ways of writing in which the importer differs from the own reader without a difference of meaning are made equal first (an absolute value or a norm written as a template, the empty item, the cross for `\times`, another arrow for `\to`, another small circle for `\circ`, the negative thin space). The results:

| quantity | value |
|---|---|
| formulas for which the own reader reports a problem | 0 of 791 |
| fraction commands in the TeX; fraction boxes of the own reader | 37; 37 |
| root commands in the TeX; root boxes of the own reader | 8; 8 |
| formulas with equal readings, own reader and importer with the alphabets protected | 763 of 791 |
| formulas with different readings (a formula that occurs in several places is counted where it occurs first) | 28: 4 in texts, 1 display formula, 23 in meanings |
| of these, a fraction with an argument without braces that the importer reads wrongly | 27 |
| of these, the command `\hbar`, which the importer shows as nothing | 1 |
| formulas with different readings, own reader and importer alone | 344: 101 in texts, 2 display formulas, 241 in meanings |

So the two programs, which share no code, read 763 formulas in the same way, and in each of the 28 others the reading of the own reader is the one that the rule of TeX requires (each was looked at; the report lists them with both readings). The 344 differences from the importer alone are these 28 and 316 formulas in which the importer loses the alphabet of a letter (297 formulas with a bold, sans-serif, double-struck, script or Gothic letter) or writes `\varepsilon` as the character of `\epsilon` (19 formulas; error E17).

The same comparison was made for the notebook as it was committed before the reader of formulas existed (commit 213d205 of the repository; its sources contain 1119 formulas, 741 of them different; list `logs/E22_formula_inventory_committed_notebook.wl`, output `logs/E22_formula_compare_committed_notebook.txt`, report `logs/E22_formula_compare_report_committed_notebook.txt`): the own reader reports no problem, 717 of the 741 formulas have equal readings, and 24 are read wrongly by the importer (2 in texts, 22 in meanings; 23 fractions and the command `\hbar`). These 24 are the formulas that the committed notebook showed wrongly (error E22). The display of the formulas was looked at on a page that shows one text converted by both programs (script `tools/formula_render_test.wls`, pages `logs/E22_formula_render_test.pdf`, picture `logs/E22_formulas_importer_and_own_reader.png`), and on the pages of the evaluated notebook (`logs/E22_notebook_texts_before_and_after_fix.png`, `logs/E23_table_of_symbols_before_and_after_fix.png`).

# 7. The registry inside the notebook

The registry of the symbols of the super-loop revision has 204 entries in nine groups:

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
| 9 Test 2: the gauge-covariant field equations of the spinor field and their solutions | 117 |

The registry is stored in the repository as the file `Revision_SuperLoop/provenance/symbols_ll.csv` (columns group, symbol, names_in_files, meaning, origin). Its last five entries (200 to 204) are the symbols of the time integration (section 2.5); entry 204 was added, and the entries 116, 119, 201, 202 and 203 restated, when the notebook was changed to IDA (section 11.5). The script `tools/registry_to_wl.wls` converts it into the Wolfram Language file `notebook_source/registry_data.wl` (a list `SL31Registry` of 204 associations with the keys "entry", "group", "symbol", "names", "meaning", "meaningEdited", "origin"), and the notebook contains this file in full in one input cell. In 16 entries (2, 36, 58, 86, 115, 123, 139, 148, 169, 174, 176, 177, 178, 179, 192, 199) the meaning was restated (file `notebook_source/meaning_edits.wl`, key "meaningEdited" -> True), because its original wording pointed to equations, theorems, sections or files of other texts; for example, one restated meaning writes out a transformation of the order parameter of the superfluid in full, where the original text only gave the number of the equation that defines it.

Every TeX segment (the text between two dollar signs) is split at the commas that stand outside braces, parentheses and brackets; a list `a, \dots, b` of numbered symbols is expanded. This gives 467 TeX items in 452 segments; the 2 label entries (86: the names of the machine checks and theorems; 87: the hypothesis labels H-grp, H-op, H-Lie, H-gr, H-dyn) are names, not symbols, and are kept as text. The kinds of the 467 items:

| kind | items | symbolized | examples |
|---|---|---|---|
| symbol | 307 | yes | $\Delta^{\rm vac}_{\rm LL}$, $G_{\rm LL}$ |
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

The 445 TeX items of the symbolized kinds give 440 different boxes, so 440 symbolized symbols. Four boxes are shared by several TeX items, always a function symbol written with different arguments; they are ONE symbol, as the head $f$ of $f(x)$ and $f(x,y)$ is one symbol:

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

Every test is recorded by the function `SL31Check[name, statement, condition, details]` in the list `` SL31`Test`$Results `` and prints one line. All 16 tests passed in the evaluation of 18 November 2026; the numbers below are the numbers of that evaluation.

**S-0 (the strings as the front end reads them).** Every string inside every symbolized box (the tags of `TagBox` and the options of `StyleBox` excepted) must be one token for the front end; the test reads each with ``FrontEnd`UndocumentedTestFEParserPacket[string, True]`` and requires that the result is the string itself. Result: 214 different strings, none cut.

**S-1.** Every one of the 467 TeX items is converted into a box structure (no `$Failed`, no `Null`). Result: 467.

**S-2.** Every one of the 440 symbolized boxes is read by `MakeExpression` as one atomic symbol. Result: 440.

**S-14 (the typed symbols).** One input cell contains all 440 symbols, each in its printed form, as a reader would type them. When the cell is evaluated, the front end sends the boxes to the kernel; every one must be one atomic symbol, and it must be the very symbol created from its registry TeX. Result: 440 of 440.

**S-3 (uniqueness).** Different boxes give different symbols, and the boxes stay different after the removal of `StyleBox`, `AdjustmentBox` and `FrameBox` (the comparison that the Notation package makes). Result: 440 boxes, 440 symbols, 440 boxes after the removal.

**S-4 (round trip).** Every symbol printed again gives exactly its box: `ToBoxes[symbol, StandardForm] === box`. Result: 440 of 440.

**S-5 (TeX).** `TeXForm` of every symbol is written from its printed form; the internal name with its separator does not appear. Result: 440 of 440.

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

**S-12 (plain-text names).** No plain-text name of the registry belongs to two different entries. Result: 612 different names, none shared.

**S-13 (the kinds that are not symbolized).** The imaginary unit $i$ is the Wolfram symbol `I`; $\mathbb R$ and $\mathbb C$ are the domains `Reals` and `Complexes`; tr, det, diag, Re, Im, $\otimes$ and ${}^T$ are `Tr`, `Det`, `DiagonalMatrix`, `Re`, `Im`, `KroneckerProduct` and `Transpose`. The test checks eleven statements on the Pauli matrices: $i^2=-1$; $\sqrt2\in\mathbb R$; $i\in\mathbb C$; $i\notin\mathbb R$; ${\rm tr}\,\sigma^{\rm LL}_3=0$; $\det\sigma^{\rm LL}_1=-1$; ${\rm diag}(1,-1)=\sigma^{\rm LL}_3$; ${\rm Re}\,\sigma^{\rm LL}_2=0$; ${\rm Im}\,\sigma^{\rm LL}_2=\begin{pmatrix}0&-1\\1&0\end{pmatrix}$; $(\sigma^{\rm LL}_1\otimes\sigma^{\rm LL}_3)^2=\mathsf 1_4$ ($\mathsf 1_4$: the $4\times4$ unit matrix); $(\sigma^{\rm LL}_2)^T=-\sigma^{\rm LL}_2$. Result: all eleven `True`.

**S-15 (the reader of formulas, section 6.2).** (a) Eighteen formulas whose reading is known are converted, and their readings on one line (`SL31FormulaReading`) are compared with the known readings. Among them are the cases that the TeX importer reads wrongly: `\tfrac12\ell^{\rm x}_{\rm LL}g^{\rm v}_{\rm LL}` must read `frac{1}{2}` followed by the two symbols; `Q_{\rm LL}=-\tfrac12` must read `Q_{LL}=-frac{1}{2}`; `\cos(\tfrac12\vartheta^{\rm B}_{\rm LL})` must have the fraction one half inside the parentheses; `-\tfrac{15}8\int_{q_{\rm LL}<\Lambda_{\rm LL}}d^3q_{\rm LL}` must read `-frac{15}{8}` followed by the integral; `\tfrac i2(y^+_{\rm LL}-y^-_{\rm LL})` must read `frac{i}{2}(..)`; `\hbar` must give the character of the reduced Planck constant. The others test a fraction with braces, roots with and without braces, a number with a script (`5\times10^{-11}`), a group in parentheses with a script, bold and sans-serif letters, an accent, bars of an absolute value with a script, a sum with limits, a matrix, and the three coordinates `x5,x6,x7`. (b) Every formula of the meanings of the registry is converted (620 different formulas): the reader must report no problem; the number of fraction boxes must equal the number of the commands `\frac` and `\tfrac` in the TeX, and the number of root boxes the number of the commands `\sqrt`; no fraction box may have an empty numerator or denominator. (c) A name is not a product: the boxes of `x5` must be the italic letter, the character `\[InvisibleSpace]` and the number, and the boxes of `5x` the number and the letter with nothing between them. Result: 18 of 18 readings equal; 620 formulas, 0 problems; 26 fraction commands and 26 fraction boxes; 7 root commands and 7 root boxes; the two box structures of (c) as required, and 3 names of this kind in the meanings of the registry (the three coordinates). (The formulas of the text cells are converted when the notebook is built; the builder reported 385 formulas and 0 warnings, and the comparison of section 6.2 covers them.)

# 10. Part II: the encyclopedia of the programs of Test 2

## 10.1 What Part II contains

Test 2 of the super-loop revision of the project asks eleven questions, called items [1] to [11]: the gauge-covariant field equations of the self-interacting spinor field $\Psi_{\rm LL}$ on $\mathbb R^{1,3}_{\rm LL}$ and their checks (item [1]); their analysis and analytic solutions (item [2]); a solver of the field equations and a plotter of its solutions, written twice, in Python and in Rust (items [3] to [6]); the construction and the physics of the superfluid boson singlet state, SFBS (items [7] to [11]). The original programs are in the folder `Revision_SuperLoop/test2` of the repository:

- the Rust library `t2core` (files `rng.rs`, `c64.rs`, `cmat.rs`, `model.rs`, `lattice.rs`, `cvode.rs`, `sym.rs`, `lib.rs`) and the Rust programs `t2_fields` (item [1]), `t2_analytic` (item [2]), `t2_solve` (items [3], [5]), `t2_plot` (items [4], [6]) and `t2_sfbs` (items [7] to [11]); with the program `t2_cvcmp`, which compares CVODE with the fixed-step method of the first version, and the small library `t2cvode_c`, through which the Python programs call the same CVODE, 14 012 lines in all;
- the Python programs `t2_solve.py`, `t2_plot.py`, `t2_examples_check.py`, `make_t2_figures.py` and `make_t2_sfbs_figures.py`.

Part II of the notebook rewrites all of them, item by item, in the Wolfram Language, with every check of the original programs. **This stage** contains chapter II.1 (the library `t2core`, with the time integration by IDA in its section II.1.5) and chapter II.2 (item [1], the program `t2_fields`: the checks GS-1 to GS-16, the two figures and the data files of the figures). The items [2] to [11] follow in the next stages; each will be added to this document when it is added to the notebook.

## 10.2 Names, indices and numbers

- **Names.** Every function of Part II begins with `t2` and keeps the name of the original function in the style of the Wolfram Language (for example the Rust function `model::gamma_ch` is `t2gammaCh`). Every name stands for one object only (section 15).
- **Indices.** The indices keep the values of the physics, $p=0,\dots,3$ and $j,\iota=1,2,3$; because the lists of the Wolfram Language begin at position 1, the code writes the entry of $p$ at position $p+1$.
- **Numbers.** The original programs compute in double precision (about 16 decimal digits). Part II does the same. Exact objects (the Dirac matrices, the generators, the pair matrices) are computed exactly, with integers and the imaginary unit.
- **Results compared with the original report.** Every printed check line ends with the numbers of the Wolfram Language and, in parentheses, those of the original report (the file `Revision_SuperLoop/test2/data/t2_fields_checks.txt`, written by the Rust program). Results that are fixed by the random numbers alone (the spectral radius 8.4349 and its bound 17.5474 of GS-10, the smallest Gram eigenvalue 8.000 of GS-15) agree to all printed digits. Results that measure rounding errors (the largest deviation of an exact identity, of the size $10^{-16}$ to $10^{-13}$) agree in their order of magnitude but not in their digits: a sum of floating-point numbers depends in its last bits on the order of the terms, and the matrix products of the Wolfram Language add the terms in a different order than the loops of the Rust code. Such a deviation is the accumulated rounding error of the computation and has no further meaning; what matters is that it stays far below the tolerance of the check. The check GS-13 is different: the notebook integrates in time with another integrator (IDA) than the original program (CVODE), so its numbers are results of their own; both are compared with the same exact solution, and the printed line gives the numbers of both integrators.

## 10.3 Random numbers

The original programs draw their random matrices, fields and momenta from the generator SplitMix64, whose state is one unsigned integer with 64 bits. One step, written with the names of the program (all arithmetic modulo $2^{64}$; `XOR` is the exclusive or of the bits, `>>` the shift of the bits to the right; the constants are hexadecimal):

```
state  = state + 0x9E3779B97F4A7C15
t      = state
t      = (t XOR (t >> 30)) * 0xBF58476D1CE4E5B9
t      = (t XOR (t >> 27)) * 0x94D049BB133111EB
output = t XOR (t >> 31)
```

A uniform number in $[-1,1)$ is `2 (output >> 11)/2^53 - 1`; the division and the subtraction are exact in double precision, so the Wolfram Language (`t2Seed`, `t2NextU64`, `t2Unit`; the integer operations `Mod`, `BitXor`, `BitShiftRight`) produces exactly the numbers of the Rust code. A random complex number is drawn as real part, then imaginary part (`t2UnitC`). Part II uses the seeds of the original programs (2026111707 for `t2_fields`) and draws the numbers in the same order, so that every random object is the same as in the original program. The check T2-RNG compares the first three integers and the first four uniform numbers for the seed 2026111707 with the values of the same algorithm computed independently in Python 3 (script `tools/splitmix64_reference.py`, output `logs/T2-RNG_reference_values.txt`): 16253586484067153034, 11771902715344777314, 8742176756923514478 and 0.7622173776706638, 0.2763122498264816, $-0.05217129678912458$, 0.4038650534416697; all are equal.

## 10.4 Components of families

A component of a registered family, for example $\gamma^0_{\rm LL}$ of $\gamma^p_{\rm LL}$, is a symbol of its own, made from the box of the family with the index replaced by its value (`SL31Instance`, section 7). When an input cell is evaluated, the front end sends the whole cell to the kernel, which reads all of it before it evaluates its first line; a component typed in the cell must therefore be a symbol before the cell is evaluated. The builder collects every component that is typed anywhere in the notebook (every reference `LL["tex", "index" -> "value", ...]` of the source) and writes the list into the first input cell of Part II, which makes the components that Part I has not made, gives them their `TraditionalForm` boxes and usage messages, and checks them (T2-SYM): 11 components are typed in the notebook ($\sigma^{\rm LL}_0,\dots,\sigma^{\rm LL}_3$, $\gamma^0_{\rm LL},\dots,\gamma^3_{\rm LL}$, $\mathsf A_{{\rm LL},j}$, $q^1_{\rm LL}$ and, for the time axis of Figure 2, $x^0_{\rm LL}$), 6 of them are made in this cell, and all 11 are read as atomic symbols whose TeX contains no internal name. Error E18 (section 14) describes what happened before this cell existed.

# 11. Part II, chapter II.1: the library t2core in the Wolfram Language

## 11.1 Complex numbers and matrices (c64.rs, cmat.rs)

The Rust library defines its own complex numbers and dense complex matrices; the Wolfram Language has both built in (section 2.1). Three routines of `cmat.rs` are rewritten because the programs and their checks use them:

- `t2det2`, `t2inv2`: the determinant and the inverse of a $2\times2$ matrix by the explicit formulas: for the matrix `{{m11, m12}, {m21, m22}}` the determinant is `m11 m22 - m12 m21` and the inverse is `{{m22, -m12}, {-m21, m11}}` divided by the determinant. `t2csqrt`: the principal square root of a complex number, the square root of its modulus times the phase factor of half its argument.
- `t2expm`: the matrix exponential by scaling and squaring. The matrix is halved `s` times, until its Frobenius norm is at most $1/4$; the Taylor series of the exponential of the halved matrix `x` (the sum of its powers divided by the factorials) is summed until a term has a Frobenius norm below $10^{-18}$ (at most 29 terms); the sum is then squared `s` times, because the exponential of twice a matrix is the square of the exponential of the matrix.
- `t2eigh`: all eigenvalues and eigenvectors of a complex Hermitian matrix, in two stages. Stage 1, Householder reduction: for every column `k` a reflection (the unit matrix minus twice the projector on a complex unit vector `vv`) is chosen that sets to zero the entries of the column below the subdiagonal; applied from both sides it keeps the eigenvalues, and after all columns the matrix is tridiagonal; the product of the reflections is accumulated in the matrix `q`. The complex subdiagonal entries are then made real and non-negative by a diagonal unitary matrix of phases. Stage 2, the implicit QL algorithm with Wilkinson shifts (`tqli`): plane rotations drive the off-diagonal entries of the real tridiagonal matrix to zero; the diagonal then holds the eigenvalues, and the accumulated rotations, multiplied by `q`, give the eigenvectors. The eigenvalues are sorted in ascending order.

Check **T2-LINALG** (seed 12345): for a random $9\times9$ Hermitian matrix (`z + ConjugateTranspose[z]` with a random complex matrix `z`) the eigenvalues of `t2eigh` differ from those of the built-in `Eigenvalues` by at most $5.33\times10^{-15}$ (tolerance $10^{-12}$), the eigenvectors reproduce the matrix (Frobenius norm of the difference below $10^{-12}$), and for a random $3\times3$ matrix `t2expm` differs from the built-in `MatrixExp` by $1.24\times10^{-14}$ in the Frobenius norm (tolerance $10^{-13}$). The full test of the eigensolver is GS-11.

## 11.2 The super-loop operators as matrices (model.rs)

Every function of `model.rs` is rewritten, in the same order; the formulas are those of section 2.5. In particular: `t2mq` is $M_{\rm LL}$; `t2bqFrom` is its inverse, which gives for a $2\times2$ matrix `m` the coefficients $\xi^{\rm LL}_0=($`m11`$+$`m22`$)/2$, $\xi^{\rm LL}_1=i($`m12`$+$`m21`$)/2$, $\xi^{\rm LL}_2=($`m21`$-$`m12`$)/2$, $\xi^{\rm LL}_3=i($`m11`$-$`m22`$)/2$ (these follow from $M_{\rm LL}(\mathbf b)=\begin{pmatrix}\xi^{\rm LL}_0-i\xi^{\rm LL}_3&-i\xi^{\rm LL}_1-\xi^{\rm LL}_2\\-i\xi^{\rm LL}_1+\xi^{\rm LL}_2&\xi^{\rm LL}_0+i\xi^{\rm LL}_3\end{pmatrix}$); `t2bqMul` multiplies two bi-quaternions through their matrices; `t2bioctMul` multiplies two bi-octonions in Cayley-Dickson form,

$$(\mathbf a+\mathbf b\,\mathbf e_4)(\mathbf c+\mathbf d\,\mathbf e_4)=(\mathbf a\mathbf c-\bar{\mathbf d}\mathbf b)+(\mathbf d\mathbf a+\mathbf b\bar{\mathbf c})\mathbf e_4 ;$$

`t2randSL2C` draws a random element of $SL(2,\mathbb C)$: four random complex entries (in the order `m11`, `m12`, `m21`, `m22`), accepted if the modulus of the determinant exceeds 0.1, then divided by the principal square root of the determinant, so that the determinant is 1; `t2su2` is $\mathsf U_{\rm LL}=\exp\big(i\sum_\iota\theta^\iota_{\rm LL}\tau^{\rm LL}_\iota/2\big)$ (with `t2expm`); `t2adjoint` is $\mathsf R^{\rm v}_{\rm LL}$, with $[\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})]_{\iota'\iota}=\tfrac12{\rm Re}\,{\rm tr}\big(\tau^{\rm LL}_{\iota'}\mathsf U_{\rm LL}\tau^{\rm LL}_\iota\mathsf U_{\rm LL}^\dagger\big)$; `t2eps3` is the Levi-Civita symbol (`Signature`). The notebook shows the matrices $\sigma^{\rm LL}_0,\dots,\sigma^{\rm LL}_3$, $\gamma^0_{\rm LL},\dots,\gamma^3_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$, $\beta_{\rm LL}$ and $\mathsf E_{\rm LL}$, each labelled with its registered symbol.

## 11.3 The lattice operator (lattice.rs)

The solvers discretize space on a lattice of $n^{\rm x}_{\rm LL}\times n^{\rm y}_{\rm LL}\times n^{\rm z}_{\rm LL}$ sites with the spacing $\ell^{\rm x}_{\rm LL}$. A site $\vec s_{\rm LL}$ has the number $s^1_{\rm LL}+n^{\rm x}_{\rm LL}(s^2_{\rm LL}+n^{\rm y}_{\rm LL}s^3_{\rm LL})$ in the code (`t2siteOf`, `t2siteCoords`); a lattice is an association with the keys `"n"`, `"ell"`, `"periodic"` (periodic or Dirichlet boundary in each direction), `"wilson"`, `"mass"`, `"mu"`. On every site a mode has 16 components, $\psi^{\rm p}_{\rm LL}$ and $\psi^{\rm h}_{\rm LL}$. The link matrix (`t2link`) has the closed form

$$\mathsf U^{\rm link}_{{\rm LL},j}=e^{-i\ell^{\rm x}_{\rm LL}g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}}\Big(\cos\big(\tfrac12\ell^{\rm x}_{\rm LL}g^{\rm v}_{\rm LL}\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert\big)\sigma^{\rm LL}_0-i\sin\big(\tfrac12\ell^{\rm x}_{\rm LL}g^{\rm v}_{\rm LL}\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert\big)\frac{\sum_\iota A^\iota_{{\rm LL},j}\tau^{\rm LL}_\iota}{\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert}\Big),$$

because $\big(\sum_\iota A^\iota_{{\rm LL},j}\tau^{\rm LL}_\iota\big)^2=\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert^2\sigma^{\rm LL}_0$. The potential of the time direction at a site (`t2potential`) is the $2\times2$ matrix $g^{\rm v}_{\rm LL}\mathsf a_{{\rm LL},0}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\sigma^{\rm LL}_0$, which acts on the eight components as $\mathsf A_{{\rm LL},0}$. The particle row of $\mathcal H^{\rm lat}_{\rm LL}$ is

$$\sum_j\Big[-\frac{i\alpha^j_{\rm LL}}{2\ell^{\rm x}_{\rm LL}}\big(\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL}+\vec\delta^j_{\rm LL})-\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})^\dagger\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})\big)$$
$$-\frac{r^{\rm W}_{\rm LL}}{2\ell^{\rm x}_{\rm LL}}\beta_{\rm LL}\big(\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL}+\vec\delta^j_{\rm LL})+\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})^\dagger\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})-2\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL})\big)\Big]$$
$$+m_{\rm LL}\beta_{\rm LL}\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL})-\big(\mathsf A_{{\rm LL},0}(\vec s_{\rm LL})+\mu_{\rm LL}\big)\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL})+\mathsf P_{\rm LL}(\vec s_{\rm LL})\psi^{\rm h}_{\rm LL}(\vec s_{\rm LL}),$$

where $\alpha^j_{\rm LL}$ and $\beta_{\rm LL}$ act on the Dirac index and the link matrices on the isospin index; a direction without a neighbour (a Dirichlet boundary, or a direction with one site) contributes zero (`t2neighbour` returns `None` there). The hole row is $\mathsf P_{\rm LL}(\vec s_{\rm LL})^\dagger\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL})$ minus the complex conjugate of the same expression without the pair term, applied to $(\psi^{\rm h}_{\rm LL})^*$ (`t2hMinusV`, `t2apply`). `t2apply` applies $\mathcal H^{\rm lat}_{\rm LL}$ to a vector or, column by column, to a matrix; `t2dense` applies it to the unit matrix and returns the whole matrix. Further functions: `t2normBound`, the bound

$$B^{\rm H}_{\rm LL}=n^{\rm act}_{\rm LL}\frac{1+2r^{\rm W}_{\rm LL}}{\ell^{\rm x}_{\rm LL}}+\lvert m_{\rm LL}\rvert+\lvert\mu_{\rm LL}\rvert+\max_{\vec s_{\rm LL}}\lVert g^{\rm v}_{\rm LL}\mathsf a_{{\rm LL},0}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\sigma^{\rm LL}_0\rVert_{\rm F}+\max_{\vec s_{\rm LL}}\sum_\iota\lvert d^\iota_{\rm LL}(\vec s_{\rm LL})\rvert$$

($\lVert\cdot\rVert_{\rm F}$: the Frobenius norm); `t2epsLat`, the energy $\varepsilon^{\rm lat}_{\rm LL}$ of section 12.2 (GS-16); `t2hLatPlane`, $\mathsf h^{\rm lat}_{\rm LL}(\vec q_{\rm LL})$; `t2anomalous`, the lattice pair densities $\Pi^\iota_{\rm LL}(\vec s_{\rm LL})=\sum_n\psi^{\rm p}_{{\rm LL},n}(\vec s_{\rm LL})^T\mathsf K^\iota_{\rm LL}\psi^{\rm h}_{{\rm LL},n}(\vec s_{\rm LL})^*$ of a set of modes. The first version of the notebook also rewrote the step of the Runge-Kutta method of the first version of the original library (`t2rk4Step`); the function was removed (error E20), and section 11.5 describes the time integration.

## 11.4 The symbolic algebra (sym.rs)

`sym.rs` is a small symbolic algebra of the Rust library (linear combinations of monomials written in TeX, with exact coefficients $a+bi$, $a$ and $b$ rational), used to print matrices and component equations in TeX and to evaluate every printed formula numerically. The Wolfram Language is itself a symbolic algebra with exact Gaussian rationals; the module is therefore not rewritten, and check GS-12 builds and evaluates the same formulas with symbols of the Wolfram Language.

## 11.5 The time integration (cvode.rs): IDA of SUNDIALS through NDSolve

The terms of this section are defined in section 2.7, its symbols in section 2.5.

**The equations and the integrator.** The lattice equations of a Nambu mode are

$$i\,\partial_0w^{\rm lat}_{\rm LL}=\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL},$$

with the mode written as one complex vector $w^{\rm lat}_{\rm LL}$ of $n^{\rm cpx}_{\rm LL}$ components. The original programs integrate them in time with CVODE (SUNDIALS 7.8.0 rewritten in Rust, called through the module `cvode.rs` of `t2core`), with the Adams-Moulton formulas of the orders 1 to 12. A notebook that must be complete in itself cannot call the Rust program. The Wolfram Language contains another integrator of the same suite, IDA, which `NDSolve` uses with the option `Method -> "IDA"`; an ordinary differential equation is the special case of a differential-algebraic equation without algebraic equations. Part II uses IDA for every time integration (decision of section 15). The first version of the notebook used the Runge-Kutta method of the fourth order with a fixed step, written by hand (error E20).

**How IDA integrates.** These facts are taken from the documentation of SUNDIALS and of the Wolfram Language (section 17); those that can be seen from outside were checked (below).

- IDA advances the solution in internal steps with the backward differentiation formulas of the orders 1 to 5 and solves the equations of every step by Newton's method. In Wolfram 15.0.1, ``Options[NDSolve`IDA]`` is `{MaxDifferenceOrder -> 5, ImplicitSolver -> Newton}`.
- IDA chooses the length $h^{\rm IDA}_{\rm LL}$ and the order of every step itself, so that the weighted root-mean-square norm of its estimate of the local error is at most 1; the tolerance of a real unknown $y$ is $r^{\rm tol}_{\rm LL}\lvert y\rvert+a^{\rm tol}_{\rm LL}$.
- After a successful step IDA doubles the length of the step if its estimate allows that, and otherwise keeps the length or shortens it (by a factor between 0.5 and 0.9). The estimate of the error of a formula of the order $k$ changes with the power $k+1$ of the length, and the rule contains a factor of safety 2, so the length is doubled when the estimate is below $2^{-(k+2)}$ of the tolerance: $1/128$ at the order 5.
- `NDSolve` takes the tolerances from its options: $r^{\rm tol}_{\rm LL}$ is 10 to the power minus the setting of `PrecisionGoal`, and $a^{\rm tol}_{\rm LL}$ is 10 to the power minus the setting of `AccuracyGoal`.
- IDA, as `NDSolve` offers it, computes with real machine numbers only: for a complex equation `NDSolve` gives the message `NDSolveValue::mconly` ("only machine real code is available") and no solution. (This is a measurement, `logs/IDA_lattice_probe.txt`; the documentation does not state it.)

**What the tolerances allow.** If the error of every real unknown equals its tolerance, the norm of the error of one step is the norm of the vector of the tolerances. The state has $2n^{\rm cpx}_{\rm LL}$ real unknowns (the real and the imaginary parts of its components), whose moduli form a vector of the norm $\lVert w^{\rm lat}_{\rm LL}\rVert$; by the triangle inequality the norm of the vector of the tolerances is at most

$$B^{\rm loc}_{\rm LL}=r^{\rm tol}_{\rm LL}\lVert w^{\rm lat}_{\rm LL}\rVert+a^{\rm tol}_{\rm LL}\sqrt{2n^{\rm cpx}_{\rm LL}}.$$

(The original programs give CVODE the tolerance $r^{\rm tol}_{\rm LL}\lvert w^{\rm cmp}_{\rm LL}\rvert+a^{\rm tol}_{\rm LL}$ for the real and for the imaginary part of a complex component; the same argument gives there $\sqrt2\,(r^{\rm tol}_{\rm LL}\lVert w^{\rm lat}_{\rm LL}\rVert+a^{\rm tol}_{\rm LL}\sqrt{n^{\rm cpx}_{\rm LL}})$.) IDA tests its estimate of the local error, not the exact local error $\delta^{\rm loc}_{\rm LL}$. The checks T2-IDA and GS-13 therefore measure $\delta^{\rm loc}_{\rm LL}$ for every internal step and compare it with $B^{\rm loc}_{\rm LL}$.

**The deviation at the ends of the internal steps.** The exact evolution is unitary. The deviation at the end of a step is the sum of two vectors: the deviation at the beginning of the step, carried through the step by the exact evolution (which does not change its norm), and the local error of the step. Its norm is therefore at most the norm of the deviation at the beginning plus $\delta^{\rm loc}_{\rm LL}$, and after $N^{\rm st}_{\rm LL}$ steps the deviation is at most the sum of the local errors of the steps. If no local error exceeds $B^{\rm loc}_{\rm LL}$, the deviation at the end of every step is at most

$$B^{\rm err}_{\rm LL}=N^{\rm st}_{\rm LL}B^{\rm loc}_{\rm LL},$$

the accumulated local error that the tolerances allow (in $B^{\rm loc}_{\rm LL}$ the largest norm of a computed state is used).

**The real form** (function `t2evolveIDA`). The state is written as the real vector of the length $2n^{\rm cpx}_{\rm LL}$ made of the real parts `u` and the imaginary parts `v` of its components, and the Hermitian matrix as `hr` $+\,i$ `hi` with the real symmetric matrix `hr` and the real antisymmetric matrix `hi`. Then $-i\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL}$ has the real part `hi.u + hr.v` and the imaginary part `-hr.u + hi.v`: the lattice equations are the real linear system with the matrix `m = {{hi, hr}, {-hr, hi}}` acting on the real vector (`u`, `v`). The matrix `m` is antisymmetric, and the largest length that it gives to a vector of the length 1 is $E^{\rm max}_{\rm LL}$. For the lattice of GS-10 the measurement gave: `m` plus its transpose is exactly zero, the largest length is 8.4349 (the spectral radius of GS-10), and 36 864 of the 589 824 entries of `m` are different from zero (`logs/IDA_lattice_probe.txt`).

**The states between the internal steps.** `NDSolve` returns the solution as an `InterpolatingFunction`. Its grid is the list of the times at the ends of the internal steps; with every grid time it stores the value of the solution and the value of the derivative that IDA computed there, and between two neighbouring grid times it gives every real unknown by cubic Hermite interpolation of these values and derivatives. (This was measured, not assumed: T2-IDA and GS-13 rebuild the polynomial from the stored values and derivatives and find the returned states equal to it within $1.3\times10^{-16}$.) An output time lies in general inside an internal step, so a returned state is an interpolated one. Let `frac`, a number between 0 and 1, be the fraction of the step at which the output time lies. The polynomial is the sum of

- the stored value at the beginning of the step times `2 frac^3 - 3 frac^2 + 1`,
- the stored value at the end times `3 frac^2 - 2 frac^3`,
- $h^{\rm IDA}_{\rm LL}$ times the stored derivative at the beginning times `frac^3 - 2 frac^2 + frac`,
- $h^{\rm IDA}_{\rm LL}$ times the stored derivative at the end times `frac^3 - frac^2`.

The two weights of the values lie between 0 and 1 and their sum is 1. The weights of the derivatives are `frac (1 - frac)^2` and `-frac^2 (1 - frac)`; the sum of their moduli is `frac (1 - frac)`, at most $1/4$. The deviation of a returned state from the exact solution has three sources:

1. **The deviations of the stored values at the two ends.** Compare the polynomial with the cubic Hermite polynomial of the exact solution, whose derivative at an end is `m` applied to its value there. The values differ by the deviations at the two ends, each at most $B^{\rm err}_{\rm LL}$; the derivatives differ, as far as this source goes, by `m` applied to these deviations, of a norm of at most $E^{\rm max}_{\rm LL}B^{\rm err}_{\rm LL}$. With the weights above this source gives at most $\big(1+\tfrac14h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL}\big)B^{\rm err}_{\rm LL}$.
2. **The stored derivatives are not exactly `m` applied to the stored values,** because Newton's method stops at a tolerance. The norm of the difference at a grid time is $r^{\rm der}_{\rm LL}$; it enters with the weights of the derivatives and gives at most $\tfrac14h^{\rm IDA}_{\rm LL}r^{\rm der}_{\rm LL}$. (Measured on the lattice of GS-10: up to $3.0\times10^{-7}$ for $r^{\rm tol}_{\rm LL}=10^{-8}$. The first draft of the code had assumed this difference to be zero; error E21.)
3. **The interpolation of the exact solution itself.** For a function with a continuous fourth derivative, the difference between the function and its cubic Hermite polynomial at a time inside the step is an integral of the fourth derivative over the step, with a weight that is nowhere negative and whose integral is the product of the squares of the distances of that time from the two ends, divided by 24. The product of the two distances is at most a quarter of $(h^{\rm IDA}_{\rm LL})^2$, so the weight integrates to at most $(h^{\rm IDA}_{\rm LL})^4/384$. The fourth derivative of the exact solution is the fourth power of `m` applied to it, a vector of at most $(E^{\rm max}_{\rm LL})^4$ times the norm of the state. This source gives at most $(h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL})^4/384$ times the norm of the state. (Because the weight is nowhere negative, the bound holds for a vector as it does for a single function.)

The sum of the three is the bound of the deviation of a returned state, written for a state of the norm 1:

$$B^{\rm out}_{\rm LL}=\big(1+\tfrac14h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL}\big)B^{\rm err}_{\rm LL}+\tfrac14h^{\rm IDA}_{\rm LL}r^{\rm der}_{\rm LL}+\frac{(h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL})^4}{384}.$$

In GS-13 the largest values of $h^{\rm IDA}_{\rm LL}$ and of $r^{\rm der}_{\rm LL}$ are used, so that the bound holds at every output time.

**The functions.** `t2evolveIDA[hm, w0, touts, rtol, atol, analysis]` integrates the equations with the Hermitian matrix `hm` from the state `w0` and returns an association: `"states"` (the complex states at the output times `touts`), `"steps"` ($N^{\rm st}_{\rm LL}$), `"evaluations"` ($N^{\rm ev}_{\rm LL}$, counted with `EvaluationMonitor`), `"hmax"` (the largest $h^{\rm IDA}_{\rm LL}$), and for every output time after the first `"stepAt"` (the length of the internal step that contains it), `"residualAt"` (the larger $r^{\rm der}_{\rm LL}$ of the two ends of that step) and `"hermiteAt"` (the norm of the difference between the returned state and the cubic Hermite polynomial rebuilt from the stored values and derivatives); `"analysis"` is the result of the optional function `analysis` applied to the `InterpolatingFunction` before it is discarded. `t2localErrorsIDA[eigenvalues, eigenvectors, rtol, atol]` is such a function: it measures $\delta^{\rm loc}_{\rm LL}$ of every internal step. With the eigenvalues $E^{\rm qp}_{{\rm LL},n}$ and the eigenvectors of the Hermitian matrix, the exact solution that starts from a computed state multiplies the coefficient of the eigenvector number $n$ by the phase factor of the angle $-E^{\rm qp}_{{\rm LL},n}h^{\rm IDA}_{\rm LL}$; the function returns the largest ratio of $\delta^{\rm loc}_{\rm LL}$ to $r^{\rm tol}_{\rm LL}$ times the norm of the state at the beginning of the step plus $a^{\rm tol}_{\rm LL}\sqrt{2n^{\rm cpx}_{\rm LL}}$, the largest $\delta^{\rm loc}_{\rm LL}$ and the largest norm of a state.

**Check T2-IDA** has two parts. (a) What the two tolerances mean, on the equation whose solution is the decaying exponential function $\exp(-x^0_{\rm LL})$ (the derivative of the unknown is minus the unknown; initial value 1), integrated from 0 to 25 with $r^{\rm tol}_{\rm LL}=10^{-8}$ and $a^{\rm tol}_{\rm LL}=10^{-10}$. The solution falls from 1 to about $10^{-11}$. While it is much larger than $a^{\rm tol}_{\rm LL}/r^{\rm tol}_{\rm LL}=10^{-2}$ the tolerance of a step is $r^{\rm tol}_{\rm LL}$ times the solution; when it is much smaller the tolerance is $a^{\rm tol}_{\rm LL}$. Required: the exact local error of every internal step is at most the tolerance of the step, and in each of the two regimes (the solution above 0.1; the solution below 0.001) the largest ratio of the local error to the tolerance exceeds $1/128$. The second requirement shows that the tolerances are the stated ones and not smaller ones: IDA would lengthen its steps if its errors stayed below $1/128$ of its tolerance. (b) `t2evolveIDA` and `t2localErrorsIDA` on a Hermitian $2\times2$ matrix, integrated from 0 to 10 with $r^{\rm tol}_{\rm LL}=10^{-10}$ and $a^{\rm tol}_{\rm LL}=10^{-12}$ and compared at 21 output times with the solution by the matrix exponential (`MatrixExp`): every $\delta^{\rm loc}_{\rm LL}$ is at most $B^{\rm loc}_{\rm LL}$, the largest deviation at the output times is at most $B^{\rm out}_{\rm LL}$, and the returned states are the cubic Hermite polynomials of the stored values and derivatives (tolerance $10^{-12}$). Result: (a) 458 internal steps; the largest ratio of a local error to its tolerance is 0.495 (0.495 in the relative regime, 0.315 in the absolute regime); (b) 910 internal steps, 1088 evaluations, largest $\delta^{\rm loc}_{\rm LL}/B^{\rm loc}_{\rm LL}=0.471$, largest deviation $1.6\times10^{-9}\le B^{\rm out}_{\rm LL}=9.4\times10^{-8}$, Hermite polynomials within $1.3\times10^{-16}$.

**Experiments made before the code was written** (scripts in `tools`, outputs in `logs`):

- `tools/gs13_ida_local_errors.wls` (output `logs/IDA_local_errors_decay.txt`): the measurement of part (a) for six pairs of settings of `PrecisionGoal` and `AccuracyGoal` (6 and 10, 8 and 10, 8 and 12, 10 and 8, 6 and 6, 12 and 14). In every case the accepted local errors follow $10^{-a}$ times the solution while the solution is large and $10^{-b}$ when it is small (`a` and `b` the two settings); no accepted error exceeded its tolerance.
- `tools/gs13_ida_tolerances.wls` (output `logs/IDA_tolerance_scaling.txt`): with a negligible absolute tolerance, multiplying the solution of a small linear system by 1024 leaves the times of all 1556 internal steps unchanged (largest difference 0): the relative part of the tolerance is proportional to the unknown. Two further comparisons of this script, which used the settings 15 and `Infinity` to switch one of the two tolerances off, were not conclusive: with these settings IDA took many more steps, and with `PrecisionGoal -> Infinity` the integration stopped at the time 7.97 with the message `NDSolveValue::ndtola`. Such settings are not used in the notebook; the direct measurement of the first script replaced these comparisons.
- `tools/gs13_ida_lattice_probe.wl`, run after the cells of Part II up to GS-11 (output `logs/IDA_lattice_probe.txt`): the integration on the lattice of GS-10 before the check was written. It showed the numbers of GS-13, that the stored derivatives differ from `m` applied to the stored values, that `EvaluationMonitor` counts the evaluations (18 096 for $r^{\rm tol}_{\rm LL}=10^{-10}$), and that a complex system is refused.

# 12. Part II, item [1]: the field equations on the 3+1 manifold (program t2_fields)

## 12.1 The physics of item [1]

The covariant derivative of the spinor field is $\nabla^{\rm LL}_p=\partial_p+\Gamma^{\rm LL}_p-i\mathsf A_{{\rm LL},p}$, with the spin connection $\Gamma^{\rm LL}_p=\mathsf s_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})\otimes\sigma^{\rm LL}_0$ and the gauge matrix $\mathsf A_{{\rm LL},p}$ (section 2.5). The mean-field equations of a mode $(\psi^{\rm p}_{\rm LL},\psi^{\rm h}_{\rm LL})$ are $i\partial_0(\psi^{\rm p}_{\rm LL},\psi^{\rm h}_{\rm LL})=\mathcal H^{\rm DB}_{\rm LL}(\psi^{\rm p}_{\rm LL},\psi^{\rm h}_{\rm LL})$ with the Dirac-Bogoliubov-de Gennes operator

$$\mathcal H^{\rm DB}_{\rm LL}=\begin{pmatrix}h^\nabla_{\rm LL}-i\Gamma^{\rm LL}_0-\mathsf A_{{\rm LL},0}-\mu_{\rm LL}&\mathsf P_{\rm LL}\\\mathsf P_{\rm LL}^\dagger&-\big(h^\nabla_{\rm LL}-i\Gamma^{\rm LL}_0-\mathsf A_{{\rm LL},0}-\mu_{\rm LL}\big)^*\end{pmatrix},$$

with the covariant Dirac Hamiltonian

$$h^\nabla_{\rm LL}=-i\sum_j\alpha^j_{\rm LL}\nabla^{\rm LL}_j+\beta_{\rm LL}m_{\rm LL},$$

and the order parameter is fixed by the self-consistency $d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$. On a plane wave $\exp(i\sum_jq^j_{\rm LL}x^j_{\rm LL})$ with constant fields and without the spin connection, $\mathcal H^{\rm DB}_{\rm LL}$ becomes the $16\times16$ matrix $\mathcal H^{\rm DB}_{\rm LL}(\vec q_{\rm LL})$ (code: `hq`) with the upper left block

$$\sum_j\alpha^j_{\rm LL}\big(q^j_{\rm LL}\mathsf 1_8-\mathsf A_{{\rm LL},j}\big)+\beta_{\rm LL}m_{\rm LL}-\mathsf A_{{\rm LL},0}-\mu_{\rm LL},$$

the off-diagonal blocks $\mathsf P_{\rm LL}$ and $\mathsf P_{\rm LL}^\dagger$, and the lower right block

$$-\Big(-\sum_j\alpha^j_{\rm LL}\big(q^j_{\rm LL}\mathsf 1_8+\mathsf A_{{\rm LL},j}\big)+\beta_{\rm LL}m_{\rm LL}-\mathsf A_{{\rm LL},0}-\mu_{\rm LL}\Big)^*$$

(the hole component carries the momentum $-\vec q_{\rm LL}$ and is complex conjugated); without gauge fields it is the matrix $\mathsf H_{\rm LL}(\vec q_{\rm LL})$ of the registry.

## 12.2 The checks GS-1 to GS-16

The constants of the program: $g^{\rm v}_{\rm LL}=0.7$, $g^{\rm F}_{\rm LL}=0.45$, $m_{\rm LL}=0.4$, $\mu_{\rm LL}=0.3$ (from GS-8 on). "Random" means drawn from the generator of section 10.3 in the order of the program, uniform in $[-1,1)$ unless another range is given. The tolerances are those of the original program; the tolerance of an exact identity is $10^{-12}$.

**GS-1 (the Dirac matrices of the frame).** For the canonical frame: $M_{\rm LL}(\mathbf f_p)=\sigma^{\rm LL}_p$, $M_{\rm LL}(\bar{\mathbf f}_0)=\sigma^{\rm LL}_0$, $M_{\rm LL}(\bar{\mathbf f}_j)=-\sigma^{\rm LL}_j$, $\mathsf E_{\rm LL}=M_{\rm LL}(-\mathbf e_2)$; the Clifford relations $\gamma^p_{\rm LL}\gamma^{p'}_{\rm LL}+\gamma^{p'}_{\rm LL}\gamma^p_{\rm LL}=2\eta^{\rm LL}_{pp'}\mathsf 1_4$; $\gamma^{\rm ch}_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$; $C_{\rm LL}=\mathrm{diag}(\mathsf E_{\rm LL},-\mathsf E_{\rm LL})$; $\alpha^j_{\rm LL}=\mathrm{diag}(-\sigma^{\rm LL}_j,\sigma^{\rm LL}_j)$ ($2\times2$ blocks). Result: the largest deviation of the Clifford relations is 0 (exact integers; original 0).

**GS-2 (the Lorentz generators on the Dirac spinor).** $\mathsf s_{\rm LL}(J^j_{\rm LL})=\mathrm{diag}(-\tfrac i2\sigma^{\rm LL}_j,-\tfrac i2\sigma^{\rm LL}_j)=\tfrac12\gamma^{j'}_{\rm LL}\gamma^{j''}_{\rm LL}$ for cyclic $(j,j',j'')$, $\mathsf s_{\rm LL}(K^j_{\rm LL})=\mathrm{diag}(-\tfrac12\sigma^{\rm LL}_j,\tfrac12\sigma^{\rm LL}_j)=\tfrac12\gamma^0_{\rm LL}\gamma^j_{\rm LL}$; the 27 relations of $\mathfrak{so}(3,1)$,
$$[\mathsf s(J^j),\mathsf s(J^{j'})]=\sum_{j''}\epsilon^{\rm LL}_{jj'j''}\mathsf s(J^{j''}),$$
$$[\mathsf s(J^j),\mathsf s(K^{j'})]=\sum_{j''}\epsilon^{\rm LL}_{jj'j''}\mathsf s(K^{j''}),$$
$$[\mathsf s(K^j),\mathsf s(K^{j'})]=-\sum_{j''}\epsilon^{\rm LL}_{jj'j''}\mathsf s(J^{j''})$$
($\mathsf s$ for $\mathsf s_{\rm LL}$, the subscripts LL of the generators omitted; 9 pairs $(j,j')$, three relations each); and the 24 intertwining relations $[\gamma^p_{\rm LL},\mathsf s_{\rm LL}(\mathbf u)]=\sum_{p'}\mathcal V_{\rm LL}(\mathbf u)^p{}_{p'}\gamma^{p'}_{\rm LL}$ for the six generators and the four values of $p$. Result: all 27 and all 24 hold within $10^{-12}$ (original: 27 and 24).

**GS-3 (the gauge group as a change of representation).** For 200 random $\mathbf g$: $\mathsf S_{\rm LL}(\mathbf g)\mathsf S_{\rm LL}(\mathbf g)^{-1}=\mathsf 1_4$ with $\mathsf S_{\rm LL}(\mathbf g)^{-1}=\mathrm{diag}(M_{\rm LL}(\mathbf g)^\dagger,M_{\rm LL}(\mathbf g)^{-1})$; $\mathsf S_{\rm LL}(\mathbf g)^{-1}\gamma^p_{\rm LL}\mathsf S_{\rm LL}(\mathbf g)=\sum_{p'}[\mathsf L_{\rm LL}(\mathbf g)]_{pp'}\gamma^{p'}_{\rm LL}$; and, for four random Hermitian bi-quaternions $\mathbf f_0,\dots,\mathbf f_3$, each with four random real coordinates (the relation holds for any four Hermitian bi-quaternions, not only for orthonormal frames), $\mathsf S_{\rm LL}(\mathbf g)\gamma^p_{\rm LL}[\mathbf f]\mathsf S_{\rm LL}(\mathbf g)^{-1}=\gamma^p_{\rm LL}[\tilde{\mathbf g}\mathbf f\tilde{\mathbf g}^\dagger]$, where the transformed bi-quaternions are computed from the coordinates of $M_{\rm LL}(\tilde{\mathbf g})M_{\rm LL}(\mathbf f_p)M_{\rm LL}(\tilde{\mathbf g})^\dagger$. Tolerance $10^{-10}$. Result: $5.13\times10^{-14}$ and $7.21\times10^{-14}$ (original $5.8\times10^{-14}$ and $6.6\times10^{-14}$).

**GS-4 (isospin and the pair matrices).** $[T^\iota_{\rm LL},T^{\iota'}_{\rm LL}]=i\sum_{\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}T^{\iota''}_{\rm LL}$; $T^\iota_{\rm LL}$ commutes with $\mathsf s_{\rm LL}(J^j_{\rm LL})\otimes\sigma^{\rm LL}_0$ and $\mathsf s_{\rm LL}(K^j_{\rm LL})\otimes\sigma^{\rm LL}_0$; $(\mathsf K^\iota_{\rm LL})^T=-\mathsf K^\iota_{\rm LL}$; for 100 random $\mathbf g$ and $\mathsf U_{\rm LL}$ (angles $\theta^\iota_{\rm LL}$ uniform in $[-3,3)$): $(\mathsf S_{\rm LL}(\mathbf g)\otimes\sigma^{\rm LL}_0)^T\mathsf K^\iota_{\rm LL}(\mathsf S_{\rm LL}(\mathbf g)\otimes\sigma^{\rm LL}_0)=\mathsf K^\iota_{\rm LL}$ (Lorentz invariance; the transpose appears because the pair operator is $\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}$) and $(\mathsf 1_4\otimes\mathsf U_{\rm LL})^T\mathsf K^\iota_{\rm LL}(\mathsf 1_4\otimes\mathsf U_{\rm LL})=\sum_{\iota'}[\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})]_{\iota\iota'}\mathsf K^{\iota'}_{\rm LL}$ (an isospin vector); and $\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}=0$ for a random commuting (c-number) $\Psi_{\rm LL}$, because the matrix is antisymmetric. Tolerances $10^{-10}$ and $10^{-12}$. Result: $5.59\times10^{-15}$ and $2.99\times10^{-16}$ (original $5.2\times10^{-15}$ and $3.0\times10^{-16}$).

**GS-5 (gauge covariance of the covariant derivative).** Random fields linear in $x_{\rm LL}$: $A^\iota_{{\rm LL},p}$ and $A^{\rm F}_{{\rm LL},p}$ (constant parts in $[-1,1)$, slopes in $[-0.5,0.5)$), $\omega^j_{{\rm LL},p}$ and $\varpi^j_{{\rm LL},p}$ (constant parts in $[-0.5,0.5)$, slopes in $[-0.3,0.3)$); a spinor field made of three plane waves (`waves`: four random numbers and a random 8-component amplitude each); a gauge transformation $\mathcal U_{\rm LL}$ whose $\theta^{\rm F}_{\rm LL}$ and $\theta^\iota_{\rm LL}$ are linear in $x_{\rm LL}$ (coefficients in $[-1,1)$) and whose $\mathbf g$ has $M_{\rm LL}(\mathbf g)=\exp M_{\rm LL}(\mathbf u)$ with a Lorentz generator $\mathbf u$ linear in $x_{\rm LL}$ (coefficients in $[-0.5,0.5)$). The transformed fields are $A^{\rm F}_{{\rm LL},p}+\partial_p\theta^{\rm F}_{\rm LL}/g^{\rm F}_{\rm LL}$, $\mathsf U_{\rm LL}\mathsf a_{{\rm LL},p}\mathsf U_{\rm LL}^{-1}-(i/g^{\rm v}_{\rm LL})(\partial_p\mathsf U_{\rm LL})\mathsf U_{\rm LL}^{-1}$ and $\mathsf S_{\rm LL}(\mathbf g)\mathsf s_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})\mathsf S_{\rm LL}(\mathbf g)^{-1}-(\partial_p\mathsf S_{\rm LL}(\mathbf g))\mathsf S_{\rm LL}(\mathbf g)^{-1}$. At 20 random points and for every $p$, the covariant derivative made of the transformed fields applied to $\mathcal U_{\rm LL}\Psi_{\rm LL}$ is compared with $\mathcal U_{\rm LL}\nabla^{\rm LL}_p\Psi_{\rm LL}$; the derivatives of the gauge functions and of $\mathcal U_{\rm LL}\Psi_{\rm LL}$ are central differences with the step $10^{-5}$, so the comparison is limited by the error of the differences. The relative deviation $\lvert{\rm lhs}-{\rm rhs}\rvert/(1+\lvert{\rm rhs}\rvert)$ must stay below $10^{-7}$, and the transformed isospin potential must be Hermitian within $10^{-9}$. Result: $1.45\times10^{-10}$ (original $1.4\times10^{-10}$).

**GS-6 (the curvature).** At 20 random points and for all 12 ordered pairs $p\neq p'$: $[\nabla^{\rm LL}_p,\nabla^{\rm LL}_{p'}]\Psi_{\rm LL}=\big(\mathsf R^{\rm LL}_{pp'}-ig^{\rm v}_{\rm LL}\sum_\iota\mathsf F^\iota_{{\rm LL},pp'}T^\iota_{\rm LL}-ig^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},pp'}\big)\Psi_{\rm LL}$, with the fields of GS-5; the outer derivative of the left side is a central difference with the step $10^{-4}$, the derivatives of the linear fields on the right side are exact. Relative tolerance $10^{-6}$. Result: $2.14\times10^{-9}$ (original $2.1\times10^{-9}$).

**GS-7 (the super-loop form of the Dirac operator).** For 50 random plane waves with four random numbers `kv` and random constant fields, $\nabla^{\rm LL}_p$ acts as the matrix $i$`kv`$_p\mathsf 1_8-i\mathsf A_{{\rm LL},p}$. Checked: $\sum_p(\gamma^p_{\rm LL}\otimes\sigma^{\rm LL}_0)\nabla^{\rm LL}_p$ has no diagonal blocks and its off-diagonal blocks are $M_{\rm LL}(\mathbf D_{\rm LL})$ and $M_{\rm LL}(\bar{\mathbf D}_{\rm LL})$; $\big(\sum_p\gamma^p_{\rm LL}\nabla^{\rm LL}_p\big)^2=\sum_p\eta^{\rm LL}_{pp}\nabla^{\rm LL}_p\nabla^{\rm LL}_p+\tfrac14\sum_{p,p'}[\gamma^p_{\rm LL},\gamma^{p'}_{\rm LL}][\nabla^{\rm LL}_p,\nabla^{\rm LL}_{p'}]$; and for a free plane wave $M_{\rm LL}(\mathbf D_{\rm LL})M_{\rm LL}(\bar{\mathbf D}_{\rm LL})=-N_{\rm LL}\,\sigma^{\rm LL}_0$, where $N_{\rm LL}$ is the norm of the Hermitian bi-quaternion with the coordinates `kv`, the first of the four numbers squared minus the squares of the other three (the product of the two $2\times2$ matrices is this difference of squares times $\sigma^{\rm LL}_0$, because the Pauli matrices anticommute with each other and square to $\sigma^{\rm LL}_0$). Result: $1.52\times10^{-15}$ (original $1.6\times10^{-15}$).

**GS-8 (the Dirac-Bogoliubov-de Gennes matrix).** For 50 random momenta $\vec q_{\rm LL}\in[-2,2)^3$, random constant fields and random complex $\vec d_{\rm LL}$: $\mathcal H^{\rm DB}_{\rm LL}(\vec q_{\rm LL})$ is Hermitian, and it is particle-hole symmetric, $\mathsf X^{\rm ph}_{\rm LL}\mathcal H^{\rm DB}_{\rm LL}(\vec q_{\rm LL})^*\mathsf X^{\rm ph}_{\rm LL}=-\mathcal H^{\rm DB}_{\rm LL}(-\vec q_{\rm LL})$. For the polar order parameter $\vec d_{\rm LL}=(0,0,\Delta_{\rm LL})$ with $\Delta_{\rm LL}=0.25+0.2\times{}$(a random number in $[-1,1)$), so $\Delta_{\rm LL}\in[0.05,0.45)$, and without gauge fields, the 16 eigenvalues (from `t2eigh`) are $\pm E^+_{\rm LL}$ and $\pm E^-_{\rm LL}$, each four times (30 random momenta). Tolerance $10^{-10}$. Result: 0 and $7.99\times10^{-15}$ (original 0 and $6.7\times10^{-15}$).

**GS-9 (the self-consistency).** For the polar state of GS-8 and 30 random momenta, the sum over the eight eigenvectors with positive energy, $\sum_{n:E^{\rm qp}_{{\rm LL},n}>0}(\psi^{\rm p}_{{\rm LL},n})^T\mathsf K^3_{\rm LL}(\psi^{\rm h}_{{\rm LL},n})^*=-2\Delta_{\rm LL}\big(1/E^+_{\rm LL}+1/E^-_{\rm LL}\big)$. With $d^3_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^3_{\rm LL}\rangle$ and $\langle\Pi^3_{\rm LL}\rangle$ the sum of this expression over the momenta, $\Delta_{\rm LL}=G_{\rm LL}\Delta_{\rm LL}\sum_{\vec q_{\rm LL}}(1/E^+_{\rm LL}+1/E^-_{\rm LL})$, which per volume is the gap equation $1/G_{\rm LL}=\int d^3q_{\rm LL}(2\pi)^{-3}(1/E^+_{\rm LL}+1/E^-_{\rm LL})$. Tolerance $10^{-10}$. Result: $1.34\times10^{-15}$ (original $1.7\times10^{-15}$).

**GS-10 (the lattice operator).** A lattice of $3\times2\times4$ sites, $\ell^{\rm x}_{\rm LL}=0.7$, periodic in the directions 1 and 3, Dirichlet in the direction 2, $r^{\rm W}_{\rm LL}=1$, with random links, potentials and order parameters on every site; the dense matrix of $\mathcal H^{\rm lat}_{\rm LL}$ has $16\cdot24=384$ rows. Checked: Hermiticity; particle-hole symmetry, $\mathsf X(\mathcal H^{\rm lat}_{\rm LL})^*\mathsf X=-\mathcal H^{\rm lat}_{\rm LL}$ with $\mathsf X$ the exchange $\mathsf X^{\rm ph}_{\rm LL}$ at every site; exact gauge covariance under random site transformations $\mathsf G^{\rm site}_{\rm LL}$ with $\exp(i\theta^{\rm F}_{\rm LL})\mathsf U_{\rm LL}$ at each site (angles in $[-3,3)$): links $\mathsf G\,\mathsf U^{\rm link}\,\mathsf G(\vec s_{\rm LL}+\vec\delta^j_{\rm LL})^\dagger$ (the unit matrix in place of the second factor at a Dirichlet boundary), potentials $\mathsf G\,(\cdot)\,\mathsf G^\dagger$, order parameters $\exp(2i\theta^{\rm F}_{\rm LL})\mathsf R^{\rm v}_{\rm LL}(\mathsf U_{\rm LL})\vec d_{\rm LL}$, and the operator transformed by the block-diagonal matrix of $\mathsf 1_4\otimes\mathsf G$ (particles) and its complex conjugate (holes); on a free periodic chain of 12 sites along $x^3_{\rm LL}$ ($\ell^{\rm x}_{\rm LL}=0.5$, $\mu_{\rm LL}=0$) the spectrum is $\pm\varepsilon^{\rm lat}_{\rm LL}$, each eight times, at the momenta $q^3_{\rm LL}=2\pi\kappa/(12\cdot0.5)$, $\kappa=0,\dots,11$; the Wilson term lifts the doubler, $\varepsilon^{\rm lat}_{\rm LL}=m_{\rm LL}+2r^{\rm W}_{\rm LL}/\ell^{\rm x}_{\rm LL}$ at $q^3_{\rm LL}\ell^{\rm x}_{\rm LL}=\pi$; and the spectral radius stays below $B^{\rm H}_{\rm LL}$. Tolerance $10^{-10}$. Result: 0, 0, $1.75\times10^{-13}$, $2.84\times10^{-14}$; spectral radius $8.4349\le B^{\rm H}_{\rm LL}=17.5474$ (original 0, 0, $1.8\times10^{-13}$, $1.4\times10^{-14}$; $8.4349\le17.5474$).

**GS-11 (the eigensolver).** For random Hermitian matrices of the dimensions 7, 64 and 200 (real diagonal and complex off-diagonal entries, drawn row by row), the larger of the relative residual of the reconstruction and the deviation of the eigenvectors from orthonormality stays below $10^{-11}$. Result: $6.49\times10^{-14}$ (original $6.0\times10^{-14}$).

**GS-12 (the printed formulas).** The formulas that the original program prints are built from symbols of the Wolfram Language (ordinary symbols such as `dS[i]` for $d^\iota_{\rm LL}$, with formatting rules so that they print as the registered symbols), shown, and evaluated for numbers assigned to the symbols; each value is compared with the matrix computation: (a) $\mathsf P_{\rm LL}$ with symbolic $d^\iota_{\rm LL}$ (block-diagonal with two equal $4\times4$ blocks; values $\vec d_{\rm LL}=(0.31-0.2i,-0.17+0.4i,0.52+0.11i)$); (b) $\mathsf s_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})$ with symbolic $\omega^j_{{\rm LL},p}$, $\varpi^j_{{\rm LL},p}$, and its blocks $-M_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})^\dagger$ and $M_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})$; (c) the $8\times8$ matrix $\Gamma^{\rm LL}_p$ and the gauge matrix $\mathsf A_{{\rm LL},p}$; (d) the component expansions of $\nabla^{\rm LL}_j\psi^{\rm p}_{\rm LL}$, $\nabla^{{\rm LL}*}_j\psi^{\rm h}_{\rm LL}$ and $\nabla^{\rm LL}_j\Psi_{\rm LL}$ for a plane wave; (e) the sixteen component equations $i\partial_0\psi^{\rm p}_{{\rm LL},k}=\dots$, $i\partial_0\psi^{\rm h}_{{\rm LL},k}=\dots$ of $\mathcal H^{\rm DB}_{\rm LL}$, compared with the $16\times16$ matrix applied to a random vector; (f) $\Pi^\iota_{\rm LL}=\sum_{k<k'}\big((\mathsf K^\iota_{\rm LL})_{kk'}-(\mathsf K^\iota_{\rm LL})_{k'k}\big)\Psi_{{\rm LL},k}\Psi_{{\rm LL},k'}$ (only $k<k'$, because the components anticommute); (g) the eight component equations of the operator equation $i\partial_0\Psi_{\rm LL}=\big(m_{\rm LL}\beta_{\rm LL}-i\Gamma^{\rm LL}_0-\mathsf A_{{\rm LL},0}-\mu_{\rm LL}\big)\Psi_{\rm LL}-i\sum_j\alpha^j_{\rm LL}\nabla^{\rm LL}_j\Psi_{\rm LL}-\tfrac12G_{\rm LL}\sum_\iota(\mathsf K^\iota_{\rm LL})^\dagger\Psi^*_{\rm LL}\Pi^\iota_{\rm LL}$, with independent random values for $\Psi_{\rm LL}$, $\Psi^*_{\rm LL}$, $\Pi^\iota_{\rm LL}$, $\nabla^{\rm LL}_j\Psi_{\rm LL}$ and $G_{\rm LL}=1.3$. The notebook also shows the Weyl blocks $M_{\rm LL}(\mathbf D_{\rm LL})$ and $M_{\rm LL}(\bar{\mathbf D}_{\rm LL})$ as $2\times2$ matrices of operators. Tolerance $10^{-12}$. Result: 0, $3.6\times10^{-16}$, $1.6\times10^{-16}$, $1.1\times10^{-16}$, $2.3\times10^{-16}$ for (a)-(c), (e), (d), (f), (g) (original 0, $4.0\times10^{-16}$, $3.4\times10^{-16}$, 0, $5.0\times10^{-16}$).

**GS-13 (the time integration).** The original program checks its time integration on the lattice of GS-10 ($3\times2\times4$ sites with random links, potentials and pair fields; $n^{\rm cpx}_{\rm LL}=384$). From a random initial state of the norm 1, drawn from a generator of its own (seed 2026111713; the state of the main generator is kept and restored, so that the random numbers of the later checks are not shifted), it integrates the lattice equations with CVODE up to $x^0_{\rm LL}=20$, with $r^{\rm tol}_{\rm LL}=10^{-8},10^{-10},10^{-12}$ and $a^{\rm tol}_{\rm LL}=r^{\rm tol}_{\rm LL}/100$, and compares the states at the 41 output times $x^0_{\rm LL}=0,0.5,\dots,20$ with the exact solution

$$w^{\rm ex}_{\rm LL}(x^0_{\rm LL})=\sum_n\exp\big(-iE^{\rm qp}_{{\rm LL},n}x^0_{\rm LL}\big)\,(\text{eigenvector }n)\,\big((\text{eigenvector }n)^\dagger w^{\rm lat}_{\rm LL}(0)\big)$$

(the eigenvalues $E^{\rm qp}_{{\rm LL},n}$ and the eigenvectors of $\mathcal H^{\rm lat}_{\rm LL}$ from `t2eigh`). The notebook repeats the check with IDA in place of CVODE (functions of section 11.5): the same lattice, the same initial state, the same tolerances, the same output times, the same exact solution. The check has four parts, for each of the three tolerances:

1. the exact local error $\delta^{\rm loc}_{\rm LL}$ of every internal step is at most $B^{\rm loc}_{\rm LL}=r^{\rm tol}_{\rm LL}\lVert w^{\rm lat}_{\rm LL}\rVert+a^{\rm tol}_{\rm LL}\sqrt{768}$, so that the deviation at the end of every internal step is at most $B^{\rm err}_{\rm LL}=N^{\rm st}_{\rm LL}B^{\rm loc}_{\rm LL}$;
2. the states at the output times are the cubic Hermite polynomials of the stored values and derivatives (tolerance $10^{-12}$), and their largest relative deviation $\delta^{\rm rel}_{\rm LL}$ from the exact solution is at most $B^{\rm out}_{\rm LL}$ (the exact state has the norm 1);
3. the largest deviation decreases from $r^{\rm tol}_{\rm LL}=10^{-8}$ to $10^{-10}$ to $10^{-12}$;
4. the two requirements of the original check for $r^{\rm tol}_{\rm LL}=10^{-12}$: the largest deviation stays below $10^{-9}$, and the norm of the state, which the exact evolution conserves, changes by less than $10^{-10}$.

Result (all four parts hold):

| $r^{\rm tol}_{\rm LL}$ | $10^{-8}$ | $10^{-10}$ | $10^{-12}$ |
|---|---|---|---|
| largest $\delta^{\rm loc}_{\rm LL}/B^{\rm loc}_{\rm LL}$ over all internal steps | 0.83 | 0.32 | 0.32 |
| largest $\delta^{\rm rel}_{\rm LL}$ at the output times | $7.1\times10^{-6}$ | $9.3\times10^{-8}$ | $4.4\times10^{-11}$ |
| $B^{\rm out}_{\rm LL}$ | $3.5\times10^{-5}$ | $9.4\times10^{-7}$ | $5.3\times10^{-8}$ |
| $B^{\rm err}_{\rm LL}$ | $3.5\times10^{-5}$ | $9.3\times10^{-7}$ | $5.3\times10^{-8}$ |
| largest change of the norm | $4.1\times10^{-6}$ | $5.4\times10^{-8}$ | $9.7\times10^{-12}$ |
| $N^{\rm st}_{\rm LL}$ | 2705 | 7309 | 41 111 |
| $N^{\rm ev}_{\rm LL}$ | 11 182 | 18 096 | 54 585 |
| largest $h^{\rm IDA}_{\rm LL}$ | $7.5\times10^{-3}$ | $3.2\times10^{-3}$ | $5.8\times10^{-4}$ |
| largest $r^{\rm der}_{\rm LL}$ | $3.0\times10^{-7}$ | $2.3\times10^{-9}$ | $7.7\times10^{-10}$ |
| Hermite polynomials within | $1.1\times10^{-16}$ | $1.1\times10^{-16}$ | $1.1\times10^{-16}$ |
| CVODE (original program): largest $\delta^{\rm rel}_{\rm LL}$ | $1.2\times10^{-6}$ | $1.7\times10^{-7}$ | $1.7\times10^{-10}$ |
| CVODE: $B^{\rm err}_{\rm LL}$ with its tolerances | $2.6\times10^{-5}$ | $6.2\times10^{-7}$ | $6.4\times10^{-9}$ |
| CVODE: largest change of the norm | $5.2\times10^{-7}$ | $9.8\times10^{-8}$ | $5.9\times10^{-11}$ |
| CVODE: $N^{\rm st}_{\rm LL}$ | 1528 | 3661 | 3804 |

with $E^{\rm max}_{\rm LL}=8.4349$ and a computing time of 8.2 seconds for the three integrations and the measurement of all local errors. What the numbers say: both integrators keep the deviation below the accumulated local error that their tolerances allow; the deviations of IDA are larger than those of CVODE for $r^{\rm tol}_{\rm LL}=10^{-8}$ and smaller for the two tighter tolerances; IDA needs more internal steps than CVODE (1.8, 2.0 and 10.8 times as many); its formulas have at most the order 5, those of CVODE the order up to 12. The lengths of the internal steps show the rule of IDA for the change of a step (section 11.5): for $r^{\rm tol}_{\rm LL}=10^{-10}$ the smallest and the largest step are $1.5625\times10^{-6}=10^{-4}/2^6$ and $3.2\times10^{-3}=10^{-4}\cdot2^5$, and for $r^{\rm tol}_{\rm LL}=10^{-12}$ they are $9.765625\times10^{-8}=10^{-4}/2^{10}$ and $5.832\times10^{-4}=10^{-4}\cdot2^3\cdot0.9^3$ (`logs/IDA_lattice_probe.txt`), that is, halvings, doublings and reductions by the factor 0.9 of a first step of the length $10^{-4}$. The check has no free tolerance of its own: its bounds follow from the tolerances of the integration.

**GS-14 (the super-loop form of the field equations).** With random bi-quaternions: (a) $M_{\rm LL}(\mathbf x)\psi'_{\rm LL}(\mathbf y)=\psi'_{\rm LL}(\mathbf y\bar{\mathbf x})$, and the Cayley-Dickson product gives $(\mathbf y\mathbf e_4)\mathbf x=(\mathbf y\bar{\mathbf x})\mathbf e_4$ (50 trials); (b) $\gamma^p_{\rm LL}[\mathbf f]\Psi_{\rm LL}[\mathbf b^-_{\rm LL},\mathbf b^+_{\rm LL}]=\Psi_{\rm LL}[\mathbf b^+_{\rm LL}\bar{\mathbf f}_p,\mathbf b^-_{\rm LL}\mathbf f_p]$ for the canonical frame and 19 sets of random Hermitian bi-quaternions; (c) $\mathcal U_{\rm LL}$ with $\mathsf U_{\rm LL}=M_{\rm LL}(\mathbf v)$ replaces $\mathbf b^+_{\rm LL}$ by $e^{i\theta^{\rm F}_{\rm LL}}\mathbf v\mathbf b^+_{\rm LL}\bar{\mathbf g}$ and $\mathbf b^-_{\rm LL}$ by $e^{i\theta^{\rm F}_{\rm LL}}\mathbf v\mathbf b^-_{\rm LL}\mathbf g^\dagger$; $\mathbf v$ is a real quaternion and $\tilde{\mathbf g}=\mathbf g^*$ (50 trials); (d) $\nabla^{\rm LL}_p$ acts on $\mathbf b^+_{\rm LL}$ as $\partial_p\mathbf b^+_{\rm LL}-\mathbf b^+_{\rm LL}\mathbf u^\omega_{{\rm LL},p}+g^{\rm v}_{\rm LL}\mathbf t^{\rm A}_{{\rm LL},p}\mathbf b^+_{\rm LL}-ig^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}\mathbf b^+_{\rm LL}$ and on $\mathbf b^-_{\rm LL}$ in the same way with $\mathbf u^{\omega*}_{{\rm LL},p}$ (30 trials, four directions each); (e) $\sum_p\gamma^p_{\rm LL}\nabla^{\rm LL}_p\Psi_{\rm LL}=\Psi_{\rm LL}\big[\sum_p(\nabla^{\rm LL}_p\mathbf b^+_{\rm LL})\bar{\mathbf f}_p,\sum_p(\nabla^{\rm LL}_p\mathbf b^-_{\rm LL})\mathbf f_p\big]$, the right multiplications $R^{\rm LL}_{\bar{\mathbf f}_p}$, $R^{\rm LL}_{\mathbf f_p}$ applied to the covariant derivatives of the charges. Tolerances $10^{-12}$ ($10^{-10}$ for (c)). Result: $6.0\times10^{-16}$, $5.9\times10^{-16}$, $3.5\times10^{-15}$, $9.7\times10^{-16}$, $2.0\times10^{-15}$ (original $3.2\times10^{-16}$, $4.1\times10^{-16}$, $3.1\times10^{-15}$, $8.8\times10^{-16}$, $2.1\times10^{-15}$).

**GS-15 (the Lorentz connection is fixed by the frame).** Five random Lorentz generators $\mathbf u_1,\dots,\mathbf u_5$ define $M_{\rm LL}(\mathbf g(x_{\rm LL}))=\exp\big(M_{\rm LL}(\mathbf u_5)+\sum_{p=0}^3x^p_{\rm LL}M_{\rm LL}(\mathbf u_{p+1})\big)$ and the connection $\Gamma_p=-(\partial_p\mathsf S_{\rm LL}(\mathbf g))\mathsf S_{\rm LL}(\mathbf g)^{-1}$ (a $4\times4$ matrix; central differences with the step $10^{-4}$). At 20 random points: $\Gamma_p$ has the form $\mathsf s_{\rm LL}(\mathbf u)$ (no off-diagonal blocks, upper block $=-$(lower block)$^\dagger$, traceless lower block); the Dirac matrices of the transformed frame, $\mathsf S_{\rm LL}(\mathbf g)\gamma^{p'}_{\rm LL}\mathsf S_{\rm LL}(\mathbf g)^{-1}$, are covariantly constant, $\partial_p(\cdot)+[\Gamma_p,\cdot]=0$; the curvature $\partial_p\Gamma_{p'}-\partial_{p'}\Gamma_p+[\Gamma_p,\Gamma_{p'}]$ vanishes. Uniqueness: the real-linear map $\mathbf u\mapsto\big([\mathsf s_{\rm LL}(\mathbf u),\gamma^{p'}_{\rm LL}]\big)_{p'=0,\dots,3}$ has the rank 6 on the six generators $J^j_{\rm LL}$, $K^j_{\rm LL}$: the smallest eigenvalue of the $6\times6$ Gram matrix of the real and imaginary parts of all entries of the commutators exceeds $10^{-6}$. Tolerance $10^{-6}$ (limited by the finite differences). Result: $1.03\times10^{-9}$, $3.55\times10^{-8}$, $7.98\times10^{-9}$; smallest Gram eigenvalue 8.000 (original $1.0\times10^{-9}$, $3.6\times10^{-8}$, $8.0\times10^{-9}$; 8.000). (The first evaluation failed this check because of error E15.)

**GS-16 (the lattice operator on plane waves).** On a periodic chain of 24 sites along $x^1_{\rm LL}$ ($\ell^{\rm x}_{\rm LL}=0.5$, $m_{\rm LL}=0.4$, $\mu_{\rm LL}=0$, no gauge fields, no pairing), $\mathcal H^{\rm lat}_{\rm LL}$, applied by `t2apply` to $\exp(iq^1_{\rm LL}x^1_{\rm LL})$ times each of the 16 unit vectors of a site, gives a plane wave of the same momentum, and the $16\times16$ matrix of the images has the eigenvalues $\pm\varepsilon^{\rm lat}_{\rm LL}$, each eight times, at all 24 momenta $q^1_{\rm LL}=2\pi\kappa/(24\ell^{\rm x}_{\rm LL})$, $\kappa=-11,\dots,12$, for $r^{\rm W}_{\rm LL}=0$ and $1$, where

$$\varepsilon^{\rm lat}_{\rm LL}(\vec q_{\rm LL})=\Big(\sum_j\frac{\sin^2(q^j_{\rm LL}\ell^{\rm x}_{\rm LL})}{(\ell^{\rm x}_{\rm LL})^2}+\Big(m_{\rm LL}+\frac{r^{\rm W}_{\rm LL}}{\ell^{\rm x}_{\rm LL}}\sum_j\big(1-\cos(q^j_{\rm LL}\ell^{\rm x}_{\rm LL})\big)\Big)^2\Big)^{1/2}$$

(sums over the directions with more than one site). Tolerance $10^{-12}$. Result: $2.73\times10^{-14}$ (original $1.4\times10^{-14}$).

**T2-DATA-1 (the data of the figures).** The original program wrote the numbers of its two figures into three files of `Revision_SuperLoop/test2/data`: `t2_dispersion_lattice.csv` (48 rows: $r^{\rm W}_{\rm LL}$, $q^1_{\rm LL}$, the largest eigenvalue of GS-16), `t2_dispersion_curves.csv` (802 rows: $r^{\rm W}_{\rm LL}$, $q^1_{\rm LL}$, $\varepsilon_{\rm LL}$, $\varepsilon^{\rm lat}_{\rm LL}$ on 401 momenta) and `t2_cvode_accuracy.csv` (the 41 rows of its integrations of GS-13 by CVODE: the output time; $\delta^{\rm rel}_{\rm LL}$ for the three tolerances; and, for each tolerance, the length and the order of the last internal step of CVODE before the output time). The first two files print their numbers with 10 decimals, the third with 7 significant digits (its times with 4 decimals). Every number of the three files is copied into the notebook (file `notebook_source/t2_fields_figure_data.wl`, written by `tools/make_fig_data_cell.py` from the files, included in one input cell). The check compares the numbers of Figure 1, entry by entry, with the numbers computed in the notebook; the tolerance is half a unit of the last printed decimal, $5.1\times10^{-11}$. The integrations by CVODE cannot be repeated in the Wolfram Language; for them the check confirms that the file has the output times of GS-13 (tolerance $5.1\times10^{-5}$) and that its numbers show what the original check states: the deviations are below $10^{-13}$ in the first row, stay below $10^{-9}$ for $r^{\rm tol}_{\rm LL}=10^{-12}$, and their largest values decrease with the tolerance. Result: $4.55\times10^{-11}$ and $5.00\times10^{-11}$ for the two files of Figure 1, as expected from the rounding of the files, so the rewritten programs compute the numbers of the original program; the times agree exactly; the largest deviations of CVODE in the file are $1.2\times10^{-6}$, $1.7\times10^{-7}$, $1.7\times10^{-10}$, the numbers of the original report. (The first version of the original program wrote a third file with the modulus of the factor of a Runge-Kutta step; that file and its check went with error E20.)

## 12.3 The figures of item [1]

**Figure 1** (data: `points` and `curves` of GS-16). Panel (a): the energy of a free particle on the lattice as a function of $q^1_{\rm LL}\ell^{\rm x}_{\rm LL}\in[-\pi,\pi]$ for $m_{\rm LL}=0.4$, $\ell^{\rm x}_{\rm LL}=0.5$: the continuum energy $\varepsilon_{\rm LL}$ (grey, dashed), $\varepsilon^{\rm lat}_{\rm LL}$ for $r^{\rm W}_{\rm LL}=0$ (orange) and for $r^{\rm W}_{\rm LL}=1$ (blue), and, as open circles, the largest eigenvalues of the lattice operator at the 24 allowed momenta, which lie on the curves. Without the Wilson term the energy returns to $m_{\rm LL}$ at $q^1_{\rm LL}\ell^{\rm x}_{\rm LL}=\pm\pi$ (a second, spurious particle, the doubler); with $r^{\rm W}_{\rm LL}=1$ the doubler is lifted to $m_{\rm LL}+2r^{\rm W}_{\rm LL}/\ell^{\rm x}_{\rm LL}=4.4$. Panel (b): the relative deviation $(\varepsilon^{\rm lat}_{\rm LL}-\varepsilon_{\rm LL})/\varepsilon_{\rm LL}$ for $\lvert q^1_{\rm LL}\ell^{\rm x}_{\rm LL}\rvert\le\pi/2$: it is of second order in $q^1_{\rm LL}\ell^{\rm x}_{\rm LL}$, negative without the Wilson term and positive (at most 0.07) with it.

**Figure 2** (data: `dev13`, `steps13`, `maxdev13` of GS-13 and the file of the integrations by CVODE). Panel (a): the relative deviation $\delta^{\rm rel}_{\rm LL}$ from the exact solution against the time $x^0_{\rm LL}$, on a logarithmic vertical axis from $10^{-13}$ to $10^{-3}$, for the three tolerances: IDA in the notebook (blue, full lines) and CVODE in the original program (orange, dashed lines). The output time 0, where the deviations are rounding errors of about $10^{-15}$, is left out. Every deviation grows roughly in proportion to the time, as the accumulation of local errors by a unitary evolution lets one expect. The two curves of one tolerance are joined, by two thin grey lines from their ends, to the label of the tolerance, which stands to the right of the curves. Panel (b): accuracy against work, on two logarithmic axes: the largest $\delta^{\rm rel}_{\rm LL}$ against the number $N^{\rm st}_{\rm LL}$ of internal steps, three points for each integrator (IDA: blue, full line, filled circles; CVODE: orange, dashed line, open squares), each labelled with its tolerance. For the same tolerance CVODE needs fewer steps; for $r^{\rm tol}_{\rm LL}=10^{-12}$ IDA reaches a four times smaller deviation with eleven times as many steps. The picture below is the figure as the evaluated notebook shows it.

![](figures/notebook_figure2_gs13.png){width=78%}

**Figure 2 of the notebook (check GS-13), as its evaluated form shows it.** The time integration of the lattice equations by IDA in the notebook and by CVODE in the original program. (a) The relative deviation $\delta^{\rm rel}_{\rm LL}$ from the exact solution against the time $x^0_{\rm LL}$, for the relative tolerances $r^{\rm tol}_{\rm LL}=10^{-8}$, $10^{-10}$ and $10^{-12}$. (b) The largest relative deviation against the number $N^{\rm st}_{\rm LL}$ of internal steps.

**How the figures are drawn.** The figures are made with `ListLinePlot`, `ListPlot`, `ListLogPlot`, `ListLogLogPlot` and `Show` from the computed lists, so every point of a figure is a number of the notebook (and, by T2-DATA-1, of the original data files; the curves of CVODE in Figure 2 are the numbers of the data file). The colors are those of the project's figures: blue `#2a78d6` and orange `#eb6834` for the two series, grey `#8a8984` for the continuum energy, a muted grey `#5f5e5a` for the text of labels and for the thin lines that join curves to their labels. A color belongs to one object in all panels of a figure: in Figure 2 blue is IDA and orange is CVODE, and the two also differ by the kind of line (full, dashed) and of marker (filled circle, open square). The two series colors were checked for readers with color-vision deficiencies: their color difference in the perceptual color space OKLab (times 100) is 33.6 for normal vision and at least 24.7 under simulated protanopia, deuteranopia and tritanopia (8 is the usual threshold of distinctness), and both have a contrast of at least 3:1 against the white page; in addition, every series is named in a legend or in a direct label, so that no information depends on color alone. Labels are placed in empty regions of the plots (checked on the exported page, errors E16 and E25), and all labels use the registered symbols, printed with upright letters as in the cells of the notebook (error E18).

# 13. How the notebook was built and evaluated

**The source.** The notebook is written as a text file in the Wolfram package format, `notebook_source/3+1super-loop.source.wl`: every cell is introduced by a marker line such as `(* ::Section:: *)`, `(* ::Text:: *)` or `(* ::Input:: *)`. Inside an input cell a symbol is written as `LL["tex"]` (for example `LL["G_{\\rm LL}"]`), and a line `(* @@INCLUDE file@@ *)` inserts a file: the packages `sl31_core.wl` (the reader of symbols and the symbolization) and `sl31_formula.wl` (the reader of formulas), the registry `registry_data.wl`, and the list `all_symbols.wl` of all 440 symbols for test S-14 (written by `tools/gen_all_symbols.wls`). A text cell holds its text with formulas in TeX between dollar signs, code between backquotes and bold text between double asterisks; a cell `(* ::DisplayFormula:: *)` holds the TeX of one formula that stands on lines of its own.

**The source of Part II** is kept in separate files, one or two per chapter (`notebook_source/part2_A_core.source.wl`, `part2_B_item1.source.wl`, `part2_C_item1b.source.wl`); a line `(* @@INCLUDECELLS file@@ *)` at the end of the main source inserts such a file with its own cell markers. A line `(* @@INCLUDE t2_fields_figure_data.wl@@ *)` inserts the numbers of the data files of item [1] (made by `tools/make_fig_data_cell.py`), and the line `(* @@COMPONENTS@@ *)` of the first cell of Part II is replaced by the builder with the list of all components of families typed in the notebook (section 10.4).

**The builder** `tools/build_notebook.wls` (run by wolframscript) turns the source into the notebook `notebook_source/3+1super-loop.unevaluated.nb`: the code of every input cell is parsed into boxes by the front end itself (so comments, line breaks and indentation are kept), every `LL["tex"]` is replaced by the two-dimensional box of the symbol, and every text cell receives its inline formulas, its code font and its bold text from the reader of formulas of the notebook itself (`SL31TextData`, section 6.2; until 18 November 2026 from the TeX importer, errors E17 and E22). The builder loads the two packages for this, counts the formulas, and turns every problem that the reader reports for a formula into a warning. The notebook is written by the kernel, not through the front end (error E9). The build of the final notebook reported 11 components of families, 194 cells, 385 formulas in text cells and 0 warnings in 3.6 seconds (`logs/build_notebook.txt`).

**Dry runs.** A build, the copy of the notebook, its evaluation in the front end (about two minutes) and the export of the pages form a long cycle, and an error of the code should be found before it. Before a build, the code is therefore evaluated once by wolframscript, without the notebook: `tools/make_dryrun.py` writes the code of the input cells of Part II into one file (all but the first cell, which holds the check T2-SYM and needs Part I and the front end), with `SL31Check` replaced by a function that prints one line and with every reference `LL["tex"]` replaced by its TeX (in Part II the symbols are only labels); `tools/make_dryrun_part1.py` writes the cells of Part I that concern the reader of formulas (the reader itself, the registry, the examples and test S-15; the package `sl31_core.wl` is loaded before them). The dry runs of the final sources gave 20 of 20 checks of Part II in 18 seconds (`logs/dryrun_part2.txt`) and the passed test S-15 (`logs/dryrun_part1.txt`). The dry runs are a help for the work only: the evaluation of the notebook in the front end stays the evidence for every result of this document.

**The evaluation** is done by the front end of Wolfram.app, exactly as Evaluation > Evaluate Notebook does it (file `tools/guirun.wl`): the notebook is opened (visible), its evaluator is set to a kernel named `SL31Run` that is declared for the running session only (`SetOptions[$FrontEndSession, EvaluatorNames -> ...]`, so the preferences of Wolfram.app are not changed), and `FrontEndTokenExecute[nb, "EvaluateNotebook"]` starts the evaluation. A hidden one-cell notebook, queued after it in the same kernel, writes the test list of the kernel to a file and then quits the kernel, so that every run starts with a fresh kernel. Afterwards the evaluated notebook is saved (with its evaluator reset to the default), exported to PDF, and a log lists every message and every printed line with the label of its input cell (`logs/3+1super-loop.evaluation.log.txt`). The commands of `guirun.wl` are sent to the kernel of the running Wolfram.app session through a small file-request mechanism (`tools/watcher.wl`, `tools/handler.wl`): a scheduled task looks every 3 seconds for request files in a scratch folder.

**To reproduce** without any of these tools: open `notebook_source/3+1super-loop.unevaluated.nb` (or the evaluated `3+1super-loop.nb`) in Wolfram and choose Evaluation > Evaluate Notebook; the result must be the same 37 PASS lines and no message. The commands below are run in the folder `3+1super-loop_provenance`. To rebuild the unevaluated notebook from the source:

```
wolframscript -file tools/build_notebook.wls notebook_source/3+1super-loop.source.wl \
    notebook_source/3+1super-loop.unevaluated.nb notebook_source/sl31_core.wl \
    notebook_source/sl31_formula.wl
```

(the source includes its files relative to its own folder; `registry_data.wl` is made by `tools/registry_to_wl.wls` from the CSV file of the registry, and `all_symbols.wl` by `tools/gen_all_symbols.wls`; the absolute paths inside these two scripts name the scratch folder of the author's Mac and must be adapted elsewhere).

(the files of Part II and `t2_fields_figure_data.wl` stand in the same folder as the source). The data cell is remade from the data files of the original program with

```
python3 tools/make_fig_data_cell.py ../Revision_SuperLoop/test2/data \
    notebook_source/t2_fields_figure_data.wl
```

The comparison of the reader of formulas with the TeX importer (section 6.2) is repeated with

```
python3 tools/formula_inventory.py notebook_source notebook_source/formula_inventory.wl
wolframscript -file tools/formula_compare.wls notebook_source/sl31_core.wl \
    notebook_source/sl31_formula.wl notebook_source/formula_inventory.wl report.txt
```

and the dry runs, in a scratch folder, with

```
mkdir -p /tmp/dryrun && cp notebook_source/sl31_core.wl /tmp/dryrun/
python3 tools/make_dryrun.py /tmp/dryrun/dryrun_part2.wl notebook_source/part2_A_core.source.wl \
    notebook_source/part2_B_item1.source.wl notebook_source/part2_C_item1b.source.wl
python3 tools/make_dryrun_part1.py /tmp/dryrun/dryrun_part1.wl notebook_source
wolframscript -file /tmp/dryrun/dryrun_part2.wl
wolframscript -file /tmp/dryrun/dryrun_part1.wl
```

(the file of the second dry run loads `sl31_core.wl` from the folder in which it stands, hence the copy).

**The pictures of the evidence.** The pictures in `logs` that show pages before and after a fix (errors E22 to E25 and E28), and the picture of Figure 2 in section 12.3, are made by `tools/make_evidence_pictures.py` from the exported pages. The script turns a page into a picture with the program pdftoppm and reads the positions of the words of the page with the program pdftotext (both belong to the Poppler tools); a piece of a page is named in the script by words of the page, and its edges are moved to the nearest empty row of the picture (or, in the table of the symbols, to the nearest horizontal rule), so that no line is cut. The pages before the fixes come from the exported file of the notebook as it was committed before (commit 213d205 of the repository, 181 pages); Figure 2 before its fix is kept as the picture `logs/E25_figure2_before_fix.png`, because the exported pages of that evaluation were replaced by the next one. The pictures are remade with

```
git show 213d205:3+1super-loop_provenance/export/3+1super-loop.pdf > old.pdf
python3 tools/make_evidence_pictures.py --old-pdf old.pdf --new-pdf export/3+1super-loop.pdf \
    --render-test logs/E22_formula_render_test.pdf --fig2-before logs/E25_figure2_before_fix.png \
    --spacing-test logs/E28_letter_digit_spacing_test.pdf --out logs \
    --figure2 figures/notebook_figure2_gs13.png
```

(Python 3 with the packages Pillow and numpy, and the Poppler tools).

**Copies to the author's computer.** The files are written in a workspace and copied into the scratch folder of the author's Mac before each evaluation. A copy can deliver an earlier version of a file whose name was used for a copy shortly before; this happened twice during the first stage of Part II (a test script and the builder arrived in their previous versions) and once more on 18 November 2026, during the experiments with IDA (error E27). Since then every file is copied from a staging file whose name begins with the first eight digits of the MD5 digest of its content, so that a name is never used for two contents, and the digest of every copied file is compared on both sides before a build or an evaluation; for the final evaluation the digests of all sources, of the two packages and of the builder agreed.

# 14. Errors found, their causes and their fixes

Every error below was met while the notebook was prepared and evaluated; each is fixed in the final notebook. E1 to E14 belong to Part I, E15 to E19 to the first stage of Part II, and E20 to E29 to the change of 18 November 2026 (the time integration by IDA, and the reader of formulas; E29 is an error of this document itself).

**E1. The symbolized box was never recognized when the notebook was evaluated by a program.** Cause: the Notation package takes the default of the option `WorkingForm` from `AbsoluteCurrentValue[InputNotebook[], {CommonDefaultFormatTypes, "Output"}]`; without a selected input notebook this is `$Failed`, and the reading rule was stored for the form `$Failed` (found by reading the source code of the package, `Notation.m`, in the installation). Fix: `WorkingForm -> StandardForm` is given explicitly in every call of `Symbolize`. Evidence: the tests S-2 and S-14.

**E2. Results in plain text.** `NotebookEvaluate[nb, InsertResults -> True]`, run by wolframscript, inserted the results as plain-text output form instead of typeset boxes. A first remedy was a custom cell-by-cell evaluator (`tools/run_notebook.wls`); it was replaced by the evaluation in the front end of Wolfram.app (E6).

**E3. Code shown in linear syntax.** Input cells written as plain strings showed the symbols as linear syntax (for example `\!\(\*SubscriptBox[...]\)`). Fix: the builder parses the code into boxes with the front end and inserts the boxes of the symbols.

**E4. The TeX importer loses alphabets.** `ImportString["$...$", "TeX"]` turns `\mathbb`, `\mathsf`, `\mathfrak` and `\mathbf` letters into plain letters, so different symbols would coincide. Fix: the notebook's own converter (section 6.1); the TeX importer was then still used for the formulas in text (it is not used at all since error E22).

**E5. Equal names for styled letters.** The Notation package builds names after removing `StyleBox` and `TagBox`, so $\mathbf e_0$, $\mathsf e_0$ and $e_0$ would become one symbol. Fix: `SymbolizeRootName` with names that record the alphabet (section 8). Evidence: S-3.

**E6. The evaluator run by wolframscript failed.** Two full runs stopped without a result; a diagnostic run showed that a cell with several statements on separate lines was read by `ToExpression` as ONE product of the statements (the output of a 52-statement cell was `Null^52`), and that cells stored as lists of boxes returned unevaluated `HoldComplete[...]` results. In the front end the statements of a cell are separated at its line breaks before they are sent to the kernel. Fix: the evaluation was moved into the front end of Wolfram.app (section 13), which also answered the author's request to execute the notebook in his Wolfram.app; `tools/run_notebook.wls` is kept, marked as superseded.

**E7. Nine symbols were not recognized in typed input.** A test notebook in the front end showed that `Symbolize[ParsedBoxWrapper[SubscriptBox["a", "LL,p"]]]` is not matched by the typed $a_{{\rm LL},p}$: the sum $\sum_{{\rm LL}=1}^2{\rm LL}\,a_{{\rm LL},p}$ gave $a_{1,p}+2\,a_{2,p}$ and `AtomQ` gave `False`, and the saved input cell held `SubscriptBox["a", RowBox[{"LL", ",", "p"}]]` (folder `experiments`). To find every affected symbol, a notebook with one cell for each of the 430 boxes then symbolized was evaluated in the front end with `$PreRead` recording the boxes that the kernel received (`tools/canon_build.wls`, `tools/canon_compare.wls`): 9 differed, all strings of roman text with several tokens (`"int,F"`, `"int,B"`, `"ext,F"`, `"ext,B"`, `"v,Y"`, `"v,4"`, `"s(2)"`, `"s(3)"`, `"s(4)"`), which the front end had cut into `RowBox`es (`logs/E07_front_end_boxes_before_fix.txt`). The front end's own parser confirmed exactly these 9 among the 213 strings then present (`tools/leaf_check.wls`, `logs/E07_leaf_check_front_end_parser.txt`). Cause: before evaluation the front end reads every string of the boxes again as typed text (the reference page of `StripBoxes` states that the front end strips boxes "using a more extensive process" when it sends them to the kernel). Fix: the converter cuts roman text into tokens as the front end does (section 6); after the fix the same comparison found 0 differences (`logs/E07_front_end_boxes_after_fix.txt`). The test S-0 repeats the check in every evaluation.

**E8. Three operators were symbolized as letters.** The tensor product $\otimes$, the direct sum $\oplus$ and the composition $\circ$ are single characters and were classified as letters before the test for operators was reached. In an input cell the front end read them as operators and regrouped the whole line around them. Fix: the test for operators comes first; these three are operators and correspond to Wolfram functions (S-13). The number of symbolized TeX items fell from 430 to 427.

**E9. The cell of test S-14 was not evaluated.** In the first full evaluation the input cell that lists all symbols received an input label but no evaluation: no result, no message, and the next cell received the same input number. The stored cell showed a regrouped line: the partial-derivative sign with a subscript, $\partial_p$, had been read as an operator applied to everything after it (a `RowBox` that begins with a comma followed it). An experiment that split the list into cells of ten symbols (`tools/s14_build.wls`, `logs/E09_s14_chunk_experiment.log.txt`) located the problem in the part with $\otimes$, $\oplus$, $\circ$ and $\partial_p$. Evaluated alone in cells of the form `{b, 1}`, every symbol box reached the kernel unchanged (`tools/canon2_build.wls`, `tools/canon2_compare.wls`, `logs/E09_operand_check_in_kernel_input.txt`), and writing and reading the cells through the front end with `NotebookPut` and `NotebookGet` changed none of them either (`tools/reparse_check.wls`, `logs/E09_reparse_check.txt`); the regrouping had been made when the builder wrote the notebook file through the front end (`Export[file, notebook, "NB"]` inside `UsingFrontEnd`). Fix (three changes, made together, so that the evaluation does not separate their effects): (a) the builder writes the notebook expression with the kernel itself; (b) every symbol whose base is an operator sign, and every group name, ket and expression, is wrapped in `TagBox[box, "SL31Name", SyntaxForm -> "symbol"]`, which tells the front end to treat the box as one operand wherever it stands (the reference page of `SyntaxForm` describes the option; the value `"symbol"` for a box that should act as a standalone thing was recommended by the front-end developer J. Fultz on the MathGroup mailing list in January 2014); (c) a script on a closing parenthesis is attached to the whole group, the form that the front end makes of typed input. After the three changes the built notebook contains no regrouped line, and the test S-14 passes for all symbols (422 at that time, 440 now).

**E10. `Clear::ssym: 42 is not a symbol or a valid string pattern.`** The demonstration of section I.2 cleared the value of $a_{\rm LL}$ with `ReleaseHold[Hold[Clear] @@ s]`; `Hold[Clear][a_LL]` evaluates its argument before `ReleaseHold` acts, so `Clear` received 42. Fix: `Clear @@ s` and `Set @@ Join[s, HoldComplete[42]]` (section 5.4).

**E11. Warnings `Symbolize::bsymbexs`** ("the box structure attempting to be symbolized has a similar or identical symbol already defined"). Cause: TeX items with the same box (the four groups of section 7) were symbolized once each. Fix: the items are gathered by their boxes (`GatherBy`), and each box is symbolized once. The test S-3 was rewritten to compare boxes and symbols (it had compared TeX items with symbols and reported 430 against 425 as a failure).

**E12. TeX of the symbols showed the internal names.** A rule `Format[s, TeXForm] = ...` had no effect, because `TeXForm` converts the `TraditionalForm` boxes of an expression, and the Notation package defines only the `StandardForm` boxes. Fix: `MakeBoxes[s, TraditionalForm] = box` for every symbol. Evidence: S-5 (422 of 422 at that time, 440 of 440 now).

**E13. Plain-text names "Missing[NotAvailable]".** The CSV import of Wolfram 15 gives `Missing["NotAvailable"]` for an empty field; `ToString` made the name "Missing[NotAvailable]" for 14 entries, and the test S-12 reported it as a name shared by 14 entries. Fix: `tools/registry_to_wl.wls` turns every field into a string and an empty field into "" (no name).

**E14. Code spans with a context mark.** In text cells the builder marks code with backquotes; code that itself contains the context mark `` ` `` (for example `` SL31`Test`$Results ``) broke the code span. Fix: such code is marked with double backquotes.

**E15. GS-15 failed in the first evaluation of Part II (32 of 33 checks passed).** The connection was written `conn15[x_, p_] := -(Sp - Sm) (0.5/h15) . Sinv`, where `Sp` and `Sm` stand here for the spinor matrices at the two shifted points and `Sinv` for the inverse spinor matrix at the point. In the grammar of the Wolfram Language the matrix product `.` (`Dot`) binds more strongly than the product written by placing two expressions side by side (`Times`): in the table of the operators in order of decreasing precedence (tutorial "Operator Input Forms") `.` stands above the implicit product. The expression was therefore read as `-(Sp - Sm) * ((0.5/h15) . Sinv)`. `Dot` of a number and a matrix is not defined; it stays unevaluated without a message, so the "connection" was a matrix whose entries contained the unevaluated product, and the comparisons of the check could not give `True`. The evaluation gave no message, so only the check itself revealed the error. Evidence: `tools/e15_dot_precedence.wls` with its output `logs/E15_dot_precedence.txt`: the parsed form of `-(a - b) (s) . c` is `Times[Times[-1, Plus[a, Times[-1, b]]], Dot[s, c]]`; `0.5 . {{1., 2.}, {3., 4.}}` stays unevaluated (head `Dot`); the faulty form applied to numbers gives a matrix of unevaluated products; the corrected form `(-(0.5) (A - B)) . m` gives the expected matrix. Fix: the number multiplies the difference first, inside parentheses, `conn15[x_, p_] := (-(0.5/h15) (Sp - Sm)) . Sinv`, with a comment in the code. After the fix GS-15 passes with the deviations of the original program (section 12.2).

**E16. Labels of the figures covered curves; a label and the summary table were cut at the edge of the page.** Found by looking at the exported PDF (`logs/E16_figures_and_table_before_fix.png`): in Figure 1(a) a label crossed the vertical axis line at $q^1_{\rm LL}=0$, the label of the doubler for $r^{\rm W}_{\rm LL}=0$ lay on the orange curve, and the label $m_{\rm LL}=0.4$ touched the same curve; in Figure 1(b) the vertical axis line crossed the legend; in Figure 2(a) the label of the stability limit was cut at the right edge of the plot and the label of the largest step lay on the curve; and the summary table of item [1] had fixed column widths (81 character widths in all), wider than the page, so it was cut at the right edge. Fix: no axis lines inside the framed plots (only the horizontal line of zero deviation in Figure 1(b)); the labels moved into empty regions and written in the muted grey of the text; the horizontal range of Figure 2(a) extended to 4, so that the label fits to the right of its line; no legend for the single curve of Figure 2(a), which its title names; the summary table without fixed widths, so that the front end wraps the statements at the width of the page, as in the table of section I.9. Evidence after the fix: `logs/E16_figures_and_table_after_fix.png`.

**E17. Formulas in text cells lost the alphabets of their letters.** The Wolfram TeX importer, which the builder used for formulas in text cells at that time (error E4), turns `\mathbf b` into `StyleBox["b", FontSlant -> "Bold"]`, an invalid option value that the front end ignores, so that the letter is printed plain; it turns `\mathsf N` and `\mathbb R` into plain letters, `\mathcal L` and `\mathfrak g` into italic letters, and `\varepsilon` into the lunate epsilon of `\epsilon`. In the texts of Part II the bold bi-quaternions $\mathbf b$, $\mathbf e_p$, $\mathbf f_p$ were therefore printed as plain letters and $\mathbb R^{1,3}_{\rm LL}$ with a plain R (`logs/E17_text_formulas_before_and_after_fix.png`), and the energy $\varepsilon_{\rm LL}$ was printed with the character of the Levi-Civita symbol $\epsilon^{\rm LL}_{jj'j''}$; the texts of Part I were affected in the same way. Evidence: `tools/e17_tex_importer.wls` with its output `logs/E17_tex_importer.txt`. Fix (in the builder): before the import every styled letter is replaced by a placeholder word inside `\mathrm{...}` (for example `\mathbb R` by `\mathrm{QZbbQRZQ}`), which the importer keeps as one string, and `\varepsilon` by the placeholder of the curly epsilon; after the import each placeholder is replaced by the box that the converter of the notebook uses for the same letter (section 6): double-struck, script and Gothic characters, bold and sans-serif upright letters, and the curly epsilon. In the built notebook the 588 formulas of the text cells then contain 270 bold letters, 116 sans-serif letters, 76 double-struck, script and Gothic characters, 12 curly and 8 lunate epsilons, no invalid bold option and no placeholder (`logs/E17_tex_importer.txt`). (This fix kept the importer; error E22 showed that the importer also reads some formulas wrongly, and the reader of formulas of section 6.2 replaced it.)

**E18. Components of families typed in input cells were not symbols.** In the table of the basic matrices (section II.1.3 of the notebook) the label $\gamma^0_{\rm LL}$ was printed as 1 and $\gamma^1_{\rm LL}$ as $\gamma_{\rm LL}$, and the axis label $q^1_{\rm LL}$ of Figure 1 lost its superscript (`logs/E18_components_before_fix.png`). Cause: the builder writes the box of a component into the input cell, but the component had never been symbolized, so the kernel read the box $\gamma^0_{\rm LL}$ as the power 0 of the expression $\gamma_{\rm LL}$, which is 1, and $q^1_{\rm LL}$ as the first power of $q_{\rm LL}$. No check had caught this, because these results were only displayed. Fix (section 10.4): the builder collects all components typed in the notebook, the first cell of Part II makes those that do not yet exist, and the check T2-SYM verifies all of them. An intermediate state showed the internal name of $q^1_{\rm LL}$ in the label of the graphics, because labels of graphics are typeset in `TraditionalForm`, for which the new component had no rule (the same cause as E12); the cell now gives every component its `TraditionalForm` box and a usage message, and T2-SYM also checks that the TeX of every component contains no internal name. Evidence after the fix: `logs/E18_components_after_fix.png`.

**E19. Notations of Part II that were not registered symbols.** The first version of the texts and the labels of Part II wrote some objects with the short notations of the original program instead of the registered symbols: $\Delta x$ for the lattice spacing $\ell^{\rm x}_{\rm LL}$, $\Delta t$ for the time step $\ell^{\rm t}_{\rm LL}$, $r_W$ for $r^{\rm W}_{\rm LL}$, $U_j(s)$ for $\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})$, $V_0$ for $\mathsf A_{{\rm LL},0}$, $P$ for $\mathsf P_{\rm LL}$, $\xi_p$ for $\xi^{\rm LL}_p(\mathbf b)$, $S$ and $L$ for $\mathsf S_{\rm LL}(\mathbf g)$ and $\mathsf L_{\rm LL}(\mathbf g)$, $E$ for $E^{\rm qp}_{{\rm LL},n}$, a plain $x$ for the product of an energy and the time step, and $m$, $\varepsilon$, $\varepsilon_{\rm lat}$ without the label LL. A plain $x$, for example, could be taken for the coordinates $x^p_{\rm LL}$. This contradicts the rule of the project that every object has one unique, registered symbol. Fix: all texts and all labels of Part II use the registered symbols; in the labels of graphics they are the symbolized symbols themselves, printed with upright letters, as in the cells of the notebook (option `SingleLetterItalics -> False`; the reference page of the option states that it is typically `True` in cells of `TraditionalForm`); the one abbreviation that was not registered, $z$ for the product $E^{\rm qp}_{{\rm LL},n}\ell^{\rm t}_{\rm LL}$ of an energy and the time step, was defined where it was used. The printed lines of the checks keep the plain-text wording of the original report, whose names (for example `dx`, `r_W`, `Sspin`, `Kch`) are the plain-text names of the registry; only the abbreviation x of GS-13 was not a registered name. (The check of the Runge-Kutta step, with the abbreviations $z$ and x, went with error E20; the present GS-13 uses registered names only.)

**E20. The time integration was made with a Runge-Kutta method of a fixed step, written by hand.** The first version of Part II followed the first version of the original programs, which advanced the lattice equations with the Runge-Kutta method of the fourth order, with a fixed time step chosen from the bound $B^{\rm H}_{\rm LL}$ so that the method is stable (time step times $B^{\rm H}_{\rm LL}$ at most 2.5). The library of the notebook contained the step of this method for the time integrations of the later items (function `t2rk4Step`; item [1] itself integrates nothing in time), its check GS-13 tested the modulus of the factor by which one such step multiplies an eigenvector of $\mathcal H^{\rm lat}_{\rm LL}$, and its Figure 2 showed this modulus. Such a method has no control of its error: the step is fixed by an argument of stability, the error of the result is not estimated, and the norm of the state, which the exact evolution conserves, decays slowly. The author rejected the method on 18 November 2026 and required the integrators of the suite SUNDIALS instead (item 6 of section 1); the original programs had been changed to CVODE in the same way. Fix in the notebook: every time integration is made by IDA through `NDSolve` (section 11.5); `t2rk4Step` was removed; the check GS-13 was rewritten as the comparison of the integration with the exact solution that the original program now makes with CVODE (section 12.2); the new check T2-IDA tests the integrator and the meaning of its tolerances; the data cell and the check T2-DATA-1 use the file of the integrations by CVODE instead of the file of the Runge-Kutta factor; Figure 2 was redrawn; the registry received the entry 204 and restated entries 116, 119, 201, 202 and 203 (section 7). Rule for all later stages: no integrator of a fixed step written by hand, anywhere. Evidence: the checks T2-IDA and GS-13 of the final evaluation.

**E21. Errors of the first draft of the IDA version, found by review before the notebook was built.** The draft was read line by line against the original check and against measurements (`logs/IDA_lattice_probe.txt`) before it was put into the notebook. (a) It assumed that the derivative which `NDSolve` stores at a grid time equals the matrix `m` applied to the stored value. The measurement showed that it does not (Newton's method stops at a tolerance): the difference was up to $2.3\times10^{-9}$ at the 41 grid times that the experiment looked at, and it is up to $3.0\times10^{-7}$ in the check GS-13, which looks at the two ends of every internal step that contains an output time. The bound $B^{\rm out}_{\rm LL}$ received the term $\tfrac14h^{\rm IDA}_{\rm LL}r^{\rm der}_{\rm LL}$, and `t2evolveIDA` returns the differences (`"residualAt"`). (b) It used $N^{\rm st}_{\rm LL}B^{\rm loc}_{\rm LL}$ as a bound of the deviation without knowing the local errors; IDA controls only its estimate of them. The function `t2localErrorsIDA` now measures the exact local error of every internal step, and the bound follows from the unitarity of the evolution. (c) It had left out one requirement of the original check, the change of the norm below $10^{-10}$ for $r^{\rm tol}_{\rm LL}=10^{-12}$; restored. (d) Its text said that IDA solves the linear equations of Newton's method with a dense direct solver; this had not been verified, and the text now states only what ``Options[NDSolve`IDA]`` shows (`logs/IDA_tolerance_scaling.txt`). (e) A local variable of the new function had the name of a variable of Part I (`xs`, the list of the coordinate components of test S-8); a local variable does not change the other one, but the rule of the notebook is one name for one object, so the variables are `tIDA` and `yIDA`. (f) The weights of the two stored derivatives were bounded by the sum of their two largest values, $8/27$ of the step; the sum of their moduli is at most $1/4$ of the step (section 11.5), which gives the sharper bound. (g) Its text said that IDA computes with real numbers only, without evidence. The experiment gave IDA a complex equation: `NDSolve` answers with the message `NDSolveValue::mconly` and returns no solution (`logs/IDA_lattice_probe.txt`); the text of the notebook quotes the message. (h) Two comparisons of the experiments on the meaning of the tolerances, which set one of the two goals to 15 digits or to `Infinity` in order to switch one tolerance off, decided nothing (with `PrecisionGoal -> Infinity` the integration stops with the message `NDSolveValue::ndtola` where a component of the solution is essentially zero); the direct measurement of the local errors replaced them (section 11.5).

**E22. The TeX importer showed 28 formulas wrongly.** Found on 18 November 2026 on the exported page of a new text: the formula $\big(1+\tfrac14h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL}\big)B^{\rm err}_{\rm LL}$ was shown as $\big(\frac{1+1}{4h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL}}\big)B^{\rm err}_{\rm LL}$. Cause: the TeX importer of the Wolfram Language, which the builder used for the formulas of text cells and section I.8 of the notebook for the meanings of the registry, takes for a fraction whose arguments are written without braces everything up to the end of the group as the denominator and everything before the fraction as part of the numerator (section 6.2); TeX takes the single token that follows. The comparison of all 791 different formulas of the present notebook (section 6.2, `logs/E22_formula_compare_report.txt`) found 27 formulas with this error and one more error, the command `\hbar` shown as nothing. The same comparison for the notebook as it was committed before (741 different formulas; `logs/E22_formula_compare_report_committed_notebook.txt`) found 24 formulas that this notebook showed wrongly. Two stood in texts of Part II: in the link matrix (section 11.3) the argument of the cosine and of the sine was shown as $1/(2\ell^{\rm x}_{\rm LL}g^{\rm v}_{\rm LL}\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert)$ instead of one half times $\ell^{\rm x}_{\rm LL}g^{\rm v}_{\rm LL}\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert$, and in the text of GS-3 the element of the Lorentz matrix was shown as the fraction with the numerator $[\mathsf L_{\rm LL}(\mathbf g)]_{pp'}=1$ and the denominator $2\,{\rm Re}\,{\rm tr}(\dots)$ instead of $[\mathsf L_{\rm LL}(\mathbf g)]_{pp'}=\tfrac12{\rm Re}\,{\rm tr}(\dots)$. The other 22 stood in the meanings of the table of the symbols (section I.8 of the notebook), for example $Q_{\rm LL}=-\tfrac12$ shown as the fraction with the numerator $Q_{\rm LL}=-1$ and the denominator 2, $d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$ with $G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$ in the denominator, and the units "in which $\hbar$ and the speed of light are 1" without the $\hbar$. The four other formulas of the 28 are new formulas of 18 November 2026 (three with the bound $B^{\rm out}_{\rm LL}$ and its parts in the texts, one in the meaning of the registry entry 204); the first of them made the error visible. The numbers of the checks were not affected, because the code does not use the importer; what was wrong was what the reader was shown. No check had caught it, because the formulas of the texts were not tested at all. Fix: the notebook's own reader of formulas (section 6.2) replaces the importer in the builder and in section I.8; test S-15 and the warnings of the builder test it at every build and every evaluation; the comparison with the importer is kept as a tool. Evidence: `logs/E22_formulas_importer_and_own_reader.png` (one text converted by both programs), `logs/E22_notebook_texts_before_and_after_fix.png` (the two texts of the notebook before and after).

**E23. The table of the symbols (section I.8 of the notebook) showed letters in wrong alphabets and broke its lines badly.** Section I.8 converted the meanings with the TeX importer alone, without the protection of the alphabets that the builder had (error E17): bold, sans-serif, double-struck, script and Gothic letters were shown as plain letters, and `\varepsilon` with the character of `\epsilon`. The meanings of the committed notebook contain 826 formulas, 598 of them different; in the reading of section 6.2 the importer alone gives 278 of the 598 a reading that differs from what their TeX says (351 of the 826 places): the 22 wrong formulas of error E22, and 256 formulas with a letter in a wrong alphabet or the wrong epsilon (counted from the report `logs/E22_formula_compare_report_committed_notebook.txt`). In an output cell the front end sets a single letter in italics automatically, so the upright E of the gamma function $\Gamma_{\rm E}$ was shown in italics. Code was shown with its backquotes. The meaning was a row of pieces of text and formulas, and the lines were broken at the boundaries of the pieces. Fix: a meaning is now one inline text cell made by `SL31TextBoxes` (section 6.2), with the formulas of the reader of formulas (every letter with an explicit style), code in the code typeface, and the lines broken by the front end. Evidence: `logs/E23_table_of_symbols_before_and_after_fix.png`.

**E24. The comma after a long formula stood at the end of the first line of the formula.** A formula inside a text is an inline cell. Where such a formula was too long for one line, it was shown on several lines, and the text that follows it, a comma, was shown at the right end of its first line. In the notebook as committed before this happened to the formula of the link matrix and to the formula of the particle row of $\mathcal H^{\rm lat}_{\rm LL}$ (both of section 11.3; the picture shows them). Fix: a formula that needs more than one line is written in the source as a cell of the style `DisplayFormula`, with its comma inside the formula: the particle row, and the new formula of the bound $B^{\rm out}_{\rm LL}$, which stood alone in a text cell. The formula of the link matrix now fits into one line, because the formulas are set in Times and no longer in the wider typeface of code. The builder converts a display formula with the reader of formulas and sets it in Times at 1.25 times the size of the style. In the style sheet of the notebook (`Default.nb`) the style `DisplayFormula` has 14 points and the text has 15 points, so that a display formula has 17.5 points and a formula inside a text, which is set at 1.1 times the size of its text, 16.5 points (`tools/style_font_sizes.wls`, output `logs/E24_style_font_sizes.txt`); at the size of the style alone a display formula would be smaller than the formulas of the text around it. Evidence: `logs/E22_notebook_texts_before_and_after_fix.png`.

**E25. Labels of Figure 2 were ambiguous, and one lay on a line.** Found on the exported page of the first evaluation with IDA: in panel (a) the three labels of the tolerances stood between the six curves, so that a reader could not tell to which pair of curves a label belonged; in panel (b) the label of CVODE for $r^{\rm tol}_{\rm LL}=10^{-10}$ lay on the dashed line that arrives at its point. Fix: in panel (a) the time axis is drawn beyond the last time, and each pair of curves is joined by two thin grey lines to its label at the right; in panel (b) the label stands below and to the left of its point. Evidence: `logs/E25_figure2_before_and_after_fix.png`.

**E26. The letters of a group name were set apart like the factors of a product.** In a formula such as $SL(2,\mathbb C)$ each letter was a box of its own, and the front end puts the space of a product between neighbouring boxes; TeX sets neighbouring letters without a space. Fix: the reader of formulas joins neighbouring letters of one alphabet into one string (section 6.2).

**E27. A copy to the author's computer delivered an earlier version of a file once more.** It happened on 18 November 2026 during the experiments with IDA, although the cause was known since the first stage of Part II (section 13): the comparison of the digests on both sides finds such a copy, but it does not prevent it. Fix: the staging files of the copies are named by the digest of their content, so that a staged name is never used for two contents (section 13); the comparison of the digests is kept. No such copy was seen afterwards.

**E28. The coordinates x5, x6, x7 were shown like products.** Found on 18 November 2026 on the picture of the table of the symbols that was made as evidence for error E23: the meaning of the number of space dimensions $D$ names the coordinates $x5,x6,x7$ of the eight-dimensional spacetime of the main project, and the notebook showed them as $x\,5$, $x\,6$, $x\,7$, each with the space of a product between the letter and the number (the notebook as committed before showed them in the same way). Cause: the reader of formulas gave a row of two boxes, the letter and the number, and the front end puts the thin space of a product between two neighbouring items of a row (section 2.1); TeX sets them without a space. A test of several ways of writing the pair (`tools/letter_digit_spacing_test.wls`, pages `logs/E28_letter_digit_spacing_test.pdf`) showed that the option `AutoSpacing -> False` of a style box does not remove the space, and that the character `\[InvisibleSpace]` between the two items does. Fix: the reader of formulas joins a number that directly follows a letter to this letter with `\[InvisibleSpace]` (section 6.2); a number before a letter stays a factor. The rule applies to one of the 791 formulas of the notebook. Part (c) of test S-15 checks the two box structures, section I.3 of the notebook states the rule, and the readings of all formulas are unchanged (the comparison of section 6.2 was repeated: the same 763 equal and 28 different readings). Evidence: `logs/E28_letter_and_number_before_and_after_fix.png` (the test, and the row of the table before and after).

**E29. Three rows of the table of the files were printed as one (an error of this document).** Found on 18 November 2026 on the printed page of section 16: the row of the file `sl31_core.wl` also contained the texts of the two rows after it, set in the typeface of code, in the version of this document as committed before and again in the first print of the present version. Cause: the content of that row named the package with its context mark, written with a single backquote. In Markdown a single backquote opens a piece of code, which for the converter pandoc runs on to the next backquote, here across the ends of the rows. The same mistake in a text cell of the notebook had been error E14; it was repeated in this document. No check saw it: the check of the layout looks for lines that are too wide and for missing characters, and a merged row is neither. Fix: the two rows write the name of the package between double backquotes. The new script `tools/check_md_source.py` reports every line of this document with an odd number of single backquotes, and it compares the number of rows of every table of this document with the number of rows of the same table in the LaTeX file; the script that builds the `.tex` and `.pdf` versions runs both checks and stops if one of them reports something. The present version has 12 tables, each with the same rows in both files.

# 15. Decisions

- "./" of the task is the top folder of the repository, so the notebook is `3+1super-loop.nb` there; "./test" is the author's folder of the Test 2 documents, whose symbols are the complete registry (section 7). The author's folders `test` and `mathematica_symbols` are private and are not part of the repository.
- Function symbols are symbolized by their heads: $W_{\rm LL}(\mathbf u)$ is the head $W_{\rm LL}$ applied to $\mathbf u$, written `W_LL[u]` in the Wolfram Language.
- Indices ($p$, $j$, $\iota$) are not symbolized: they are the running letters inside the scripts; their components are symbolized on demand (`SL31Instance`). Primed indices are symbolized, because they also appear alone.
- Operators, number sets and the imaginary unit are not symbolized: they are Wolfram functions, domains and the symbol `I` (S-13).
- The single letter $D$ is symbolized in the box `TagBox["D", "SL31Plain"]`, so that the letter `D`, typed plainly, keeps its meaning (the derivative) in the Wolfram Language.
- Values are given to symbols only inside `Block`, so that after every test the symbols are free again.
- The registry is contained in the notebook as data (one input cell of about 140 000 characters) and also shown as formatted tables, group by group (section I.8).
- Part II rewrites the original programs in the order of their code and draws the random numbers in the same order with the same seeds, so that every random object of a check equals the one of the original program; results that are pure rounding errors are reported next to the values of the original report (section 10.2).
- The numbers of the three data files of the figures of item [1] are copied into the notebook (about 46 000 characters in one input cell) instead of being read from the files, so that the notebook is complete and stand-alone; they are used by the check T2-DATA-1 and, for the curves of CVODE, by Figure 2. The cell fills 22 pages of the printed notebook (pages 173 to 194).
- The symbolic algebra `sym.rs` of the original library is not rewritten, because the Wolfram Language is itself a symbolic algebra (section 11.4); GS-12 uses symbols of the Wolfram Language instead.
- Every name of Part II stands for one object only. For example, the variable of the sum over the plane waves of GS-5 is called `wave`, because the name `wv` belongs to the random vector of GS-12.
- Labels of graphics are built from expressions (`Row`, `Subscript`, `Superscript` and the registered symbols), not from strings in linear syntax, so that the source stays readable and the labels are typeset in the same way in the notebook and in the PDF.
- The checks T2-SYM, T2-RNG, T2-LINALG and T2-DATA-1 are checks of the rewriting itself and are not in the original report; GS-1 to GS-16 keep the names, the statements and the tolerances of the original report.
- The components of families are collected by the builder from the source rather than listed by hand, so that a component typed in a later item of Part II is made automatically (section 10.4).
- The notebook integrates in time with IDA of SUNDIALS, through `NDSolve` with `Method -> "IDA"`. The original programs use CVODE of the same suite, which a notebook that is complete in itself cannot call; IDA is the integrator of this suite that the Wolfram Language contains. The equations are given to it in real form, because it accepts real equations only (section 11.5).
- The checks of the time integration do not trust the error estimate of the integrator: they measure the exact local error of every internal step and derive their bounds from the tolerances of the integration; they have no tolerance of their own that could be adjusted (sections 11.5 and 12.2).
- The states at the output times are taken from the `InterpolatingFunction` of `NDSolve`, as a user of the notebook would take them; the check therefore includes the error of the interpolation between the internal steps (the bound $B^{\rm out}_{\rm LL}$), and it verifies that the interpolation is the cubic Hermite polynomial that the bound assumes.
- The file of the original program with the modulus of the factor of a Runge-Kutta step, its part of the data cell, its check and its figure were removed with the method itself (error E20); nothing of the removed method is kept in the notebook.
- Every formula that the notebook shows is converted by its own reader of formulas (section 6.2). A formula is set in the typeface Times with an explicit style for every letter, so that the alphabets of the project (italic, upright, bold, sans-serif, double-struck, script, Gothic) differ visibly in every kind of cell.
- The meanings of the table of the symbols are inline text cells (error E23), and a formula that stands alone is a display formula (error E24).
- Between the factors of a product the notebook keeps the thin space that the front end of the Wolfram Language puts there, although TeX sets a product without it: a notebook should show a product as the Wolfram Language shows it. Inside a name there is no space: letters of one alphabet form one string, and a number that directly follows a letter is joined to it (errors E26 and E28).
- The pictures that show a page before and after a fix are made by a script from the exported pages, and the pieces of a page are named in the script by words of the page (section 13), so that a reader can see what was cut from where and can make the pictures again.
- The comparison of the reader of formulas with the TeX importer is kept as a tool (`tools/formula_compare.wls`), although the importer is no longer used: two programs that share no code and agree on 763 formulas are a test of the reader that a list of examples could not give.
- In Figure 2 the curves are labelled directly, each pair joined to its label, instead of by a legend with six entries; the two integrators keep their colors, kinds of line and markers in both panels.

# 16. Files

All paths are relative to the top folder of the repository.

| file | content |
|---|---|
| `3+1super-loop.nb` | the notebook, evaluated (Part I; Part II: the library with the time integration by IDA, and item [1]) |
| `3+1super-loop_provenance/notebook_source/3+1super-loop.source.wl` | the source of the notebook (cells marked in the Wolfram package format) |
| `3+1super-loop_provenance/notebook_source/sl31_core.wl` | the package `` SL31` `` (the reader of symbols, kinds, names, symbolization), included in the notebook |
| `3+1super-loop_provenance/notebook_source/sl31_formula.wl` | the reader of formulas of the package `` SL31` `` (section 6.2), included in the notebook |
| `3+1super-loop_provenance/notebook_source/formula_inventory.wl` | the 791 different formulas that the notebook shows, written by formula_inventory.py |
| `3+1super-loop_provenance/notebook_source/registry_data.wl` | the registry as Wolfram Language data (204 entries), included in the notebook |
| `3+1super-loop_provenance/notebook_source/meaning_edits.wl` | the 16 restated meanings applied by registry_to_wl.wls |
| `3+1super-loop_provenance/notebook_source/all_symbols.wl` | the 440 symbols as LL[tex] references, included in the cell of test S-14 |
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
| `3+1super-loop_provenance/tools/build_notebook.wls` | the builder: source to notebook (includes, components of families, text cells and display formulas by the reader of formulas) |
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
| `3+1super-loop_provenance/tools/list_file_digests.py` | checks that this table names every file of the item, and writes the two lists of digests of this document (Python 3) |
| `3+1super-loop_provenance/tools/check_md_source.py` | E29: reports lines of this document with an odd number of single backquotes, and tables whose rows differ between the .md and the .tex version; run by build_provenance_docs.sh (Python 3) |
| `3+1super-loop_provenance/notebook_source/part2_A_core.source.wl` | the source of the introduction of Part II, of its first cell (the components of families) and of chapter II.1 (the library t2core, with the time integration by IDA and the check T2-IDA) |
| `3+1super-loop_provenance/notebook_source/part2_B_item1.source.wl` | the source of item [1], checks GS-1 to GS-11 |
| `3+1super-loop_provenance/notebook_source/part2_C_item1b.source.wl` | the source of item [1], checks GS-12 to GS-16, the figures, T2-DATA-1 and the summary of the item |
| `3+1super-loop_provenance/notebook_source/t2_fields_figure_data.wl` | the numbers of the three data files of t2_fields (the two files of Figure 1 and the file of the integrations by CVODE), included in the notebook (T2-DATA-1) |
| `3+1super-loop_provenance/tools/make_fig_data_cell.py` | writes t2_fields_figure_data.wl from the data files of the original program (Python 3) |
| `3+1super-loop_provenance/tools/splitmix64_reference.py` | the reference values of the check T2-RNG, computed in Python 3 |
| `3+1super-loop_provenance/tools/e15_dot_precedence.wls` | E15: the parsed form of the faulty expression and the behaviour of `Dot` with a number |
| `3+1super-loop_provenance/tools/e17_tex_importer.wls` | E17: what the TeX importer makes of styled letters, and the boxes of the text formulas of the notebook as it was built then |
| `3+1super-loop_provenance/tools/formula_inventory.py` | section 6.2: lists every formula that the notebook shows (Python 3) |
| `3+1super-loop_provenance/tools/formula_compare.wls` | section 6.2, E22: compares the reader of formulas with the TeX importer on every formula |
| `3+1super-loop_provenance/tools/formula_render_test.wls` | section 6.2, E22, E23: a page with one text converted by both programs and with rows of the table of the symbols |
| `3+1super-loop_provenance/tools/style_font_sizes.wls` | E24: prints the font sizes of the cell styles of the style sheet |
| `3+1super-loop_provenance/tools/letter_digit_spacing_test.wls` | E28: a page with eight ways of writing a letter followed by a number |
| `3+1super-loop_provenance/tools/make_evidence_pictures.py` | section 13: makes the pictures of the errors E22 to E25 and E28 and the picture of Figure 2 from the exported pages (Python 3) |
| `3+1super-loop_provenance/tools/make_dryrun.py` | section 13: writes the dry run of Part II (Python 3) |
| `3+1super-loop_provenance/tools/make_dryrun_part1.py` | section 13: writes the dry run of the cells of the reader of formulas (Python 3) |
| `3+1super-loop_provenance/tools/gs13_ida_local_errors.wls` | section 11.5: the local errors that IDA accepts on the decaying exponential |
| `3+1super-loop_provenance/tools/gs13_ida_tolerances.wls` | section 11.5: the times of the internal steps of IDA when the solution or the tolerances are scaled |
| `3+1super-loop_provenance/tools/gs13_ida_lattice_probe.wl` | section 11.5, E21: IDA on the lattice of GS-10 before the check was written (appended to the dry run of Part II up to GS-11) |
| `3+1super-loop_provenance/logs/T2-RNG_reference_values.txt` | the output of splitmix64_reference.py |
| `3+1super-loop_provenance/logs/E15_dot_precedence.txt` | E15: the output of e15_dot_precedence.wls |
| `3+1super-loop_provenance/logs/E17_tex_importer.txt` | E17: the output of e17_tex_importer.wls |
| `3+1super-loop_provenance/logs/E16_figures_and_table_before_fix.png` | E16: the pages of the figures and of the summary table before the fix |
| `3+1super-loop_provenance/logs/E16_figures_and_table_after_fix.png` | E16: the same pages after the fix |
| `3+1super-loop_provenance/logs/E17_text_formulas_before_and_after_fix.png` | E17: a text of section II.1.3 before and after the fix |
| `3+1super-loop_provenance/logs/E18_components_before_fix.png` | E18: the table of the basic matrices and the axis label of Figure 1 before the fix |
| `3+1super-loop_provenance/logs/E18_components_after_fix.png` | E18: the same after the fix |
| `3+1super-loop_provenance/logs/build_notebook.txt` | the output of the builder for the final notebook |
| `3+1super-loop_provenance/logs/dryrun_part1.txt` | the output of the dry run of the cells of the reader of formulas |
| `3+1super-loop_provenance/logs/dryrun_part2.txt` | the output of the dry run of Part II |
| `3+1super-loop_provenance/logs/IDA_local_errors_decay.txt` | the output of gs13_ida_local_errors.wls |
| `3+1super-loop_provenance/logs/IDA_tolerance_scaling.txt` | the output of gs13_ida_tolerances.wls |
| `3+1super-loop_provenance/logs/IDA_lattice_probe.txt` | the output of the dry run with gs13_ida_lattice_probe.wl |
| `3+1super-loop_provenance/logs/E22_formula_compare_report.txt` | E22: the report of formula_compare.wls: every formula read differently, with both readings and both box structures |
| `3+1super-loop_provenance/logs/E22_formula_inventory_committed_notebook.wl` | E22: the 741 different formulas of the notebook as committed before the reader of formulas (commit 213d205), written by formula_inventory.py from the sources of that commit |
| `3+1super-loop_provenance/logs/E22_formula_compare_committed_notebook.txt` | E22: the output of formula_compare.wls for this list |
| `3+1super-loop_provenance/logs/E22_formula_compare_report_committed_notebook.txt` | E22, E23: its report: the 24 formulas that the committed notebook showed wrongly, and the formulas that the importer alone reads differently |
| `3+1super-loop_provenance/logs/E22_formula_render_test.pdf` | E22, E23: the pages made by formula_render_test.wls |
| `3+1super-loop_provenance/logs/E22_formulas_importer_and_own_reader.png` | E22: one text converted by the reader of formulas and by the TeX importer |
| `3+1super-loop_provenance/logs/E22_notebook_texts_before_and_after_fix.png` | E22, E24: the texts of the link matrix, of the particle row and of GS-3 before and after the fix |
| `3+1super-loop_provenance/logs/E23_table_of_symbols_before_and_after_fix.png` | E23: rows of the table of the symbols before and after the fix |
| `3+1super-loop_provenance/logs/E24_style_font_sizes.txt` | E24: the output of style_font_sizes.wls |
| `3+1super-loop_provenance/logs/E25_figure2_before_fix.png` | E25: Figure 2 on the exported page of the first evaluation with IDA, before the fix (input of make_evidence_pictures.py) |
| `3+1super-loop_provenance/logs/E25_figure2_before_and_after_fix.png` | E25: Figure 2 before and after the fix |
| `3+1super-loop_provenance/logs/E28_letter_digit_spacing_test.pdf` | E28: the page made by letter_digit_spacing_test.wls |
| `3+1super-loop_provenance/logs/E28_letter_and_number_before_and_after_fix.png` | E28: the test page, and the row of the table of the symbols with the coordinates x5, x6, x7 before and after the fix |
| `3+1super-loop_provenance/figures/notebook_figure2_gs13.png` | Figure 2 of the evaluated notebook, shown in section 12.3 of this document (written by make_evidence_pictures.py) |

The SHA-256 digests of these files, as committed (a SHA-256 digest is a number of 64 hexadecimal digits computed from the content of a file; the list has the format of the program `sha256sum` and is written by `tools/list_file_digests.py`, which first checks that the table above names every file of the notebook and of the folder `3+1super-loop_provenance`, and no other; the three versions of this document cannot list their own digests):

```
c3f76e3b09809e6eb7e894e1e3287ffcddd09162a960097c5d7eacffcbf2697e  3+1super-loop.nb
b3bd44b40804e407a3a98a9bd23407517c7682040660d31771409c21198bacff  3+1super-loop_provenance/notebook_source/3+1super-loop.source.wl
36cf048620323e4ac04d5ebf285576e2e191404791ac666a780e1165791357ef  3+1super-loop_provenance/notebook_source/sl31_core.wl
6a74c3a5e05ed77db013f0b8360e3e05071a9cce149a242b3d9b59d57e51babb  3+1super-loop_provenance/notebook_source/sl31_formula.wl
76a5c58ad90eba6b7e2d43238920abc5d259960a7e654fe262d629520925cd04  3+1super-loop_provenance/notebook_source/formula_inventory.wl
4354761805aae40fd9cae8cae6acc60711a3339db786f850249cc11e67195b2d  3+1super-loop_provenance/notebook_source/registry_data.wl
5edd0eefdf3374e678e0654be8e0be517f702391f6f9f2399f24364e15351487  3+1super-loop_provenance/notebook_source/meaning_edits.wl
654124ba08b62ebf8739d8c397d65a1a6b9ae983b0651491dc05c5b88a8e16db  3+1super-loop_provenance/notebook_source/all_symbols.wl
35d000181d18c4b8ad64270e788942e25647962c4801ff645298a5868d72448e  3+1super-loop_provenance/notebook_source/3+1super-loop.unevaluated.nb
824f10785197145fc6117ab9eb5e949320f9bac814894ec07245e56dc6ea0e9a  3+1super-loop_provenance/export/3+1super-loop.pdf
a47dcc7cdcc57caf959d9ea2dbdde1eca1d3d185dcaf2f4cb021187b81e0f8ec  3+1super-loop_provenance/logs/3+1super-loop.evaluation.log.txt
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
5b4d618dc85eb909b58b48b9d846521677c4b54b81597a9b8883ce17a532b96d  3+1super-loop_provenance/tools/build_notebook.wls
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
9cb66e9653dfac1998fa8bcffa68834c6faea6beefb2db9f9bf188244cc21c42  3+1super-loop_provenance/tools/build_provenance_docs.sh
00d3e3c225b60acac822b408deed88744968de7f8ef78467f6799ea1b68a3d30  3+1super-loop_provenance/tools/doc_header.tex
2ebb59a1cb4757d4dd27e22398fb2a924fc435b8c79dba2d2480a505d404289c  3+1super-loop_provenance/tools/list_file_digests.py
e86a24a38a571b429ecd4905f4ddbe48944923e85e37e062d48d7c9420283567  3+1super-loop_provenance/tools/check_md_source.py
34e1738617b8ee551ba84a6225384bb22921a3254f03b1cf082d85bd3bb4f389  3+1super-loop_provenance/notebook_source/part2_A_core.source.wl
6543c2ec27ed15ec291865b39d2d52fa2190e1dbf524b83e05e00d0c9397b99d  3+1super-loop_provenance/notebook_source/part2_B_item1.source.wl
cdeafaad84993d6607428fdb455fd9c8d217b1848ddaa11e25274ead904a4371  3+1super-loop_provenance/notebook_source/part2_C_item1b.source.wl
5c7a4e20759c2ff5acb04f58eca538095b8e405996f0671ca3aa9f4c7d7ed302  3+1super-loop_provenance/notebook_source/t2_fields_figure_data.wl
60f6902082f6f06cc7266643af9affec8e82593f2cc4bae7766628e9fb72ea37  3+1super-loop_provenance/tools/make_fig_data_cell.py
06be9891ca81c88602943cabdca85f971193481d5f669207352b1a18bc53dca5  3+1super-loop_provenance/tools/splitmix64_reference.py
f28f35adb6abc25e59f0e6f389e4566632631d238d20e044c27ba23247ff437b  3+1super-loop_provenance/tools/e15_dot_precedence.wls
f34de46eaa0bbb6a7261868c62dc076e3927185bfcc0fdea91c0403770d172cb  3+1super-loop_provenance/tools/e17_tex_importer.wls
5d856bb642e225d3cf7512aa6b5481bb61d718337d9cc6ba3c5d2fb4fc4c9fec  3+1super-loop_provenance/tools/formula_inventory.py
6c587b365b12f09971da94bb1e541f5f48abbec1eb6cc3ce54886ed60e601d1d  3+1super-loop_provenance/tools/formula_compare.wls
4175362fe6f52a58e69e97b8474e5b29152652025fb34196dfaac338b7aa4eb5  3+1super-loop_provenance/tools/formula_render_test.wls
ff36dbdb12b8bd2bba60a5f70a59c7cbe6472dde66dcb2ce2e89e03479ba3a71  3+1super-loop_provenance/tools/style_font_sizes.wls
a8cfea5d9b6c1af1982c91ef34b22d88da8561958380d7b8ba2976dd9f1fa981  3+1super-loop_provenance/tools/letter_digit_spacing_test.wls
96ea5f9de2ab138aefac05e70249e63f588a5fb739ff93c0a06c769496c06322  3+1super-loop_provenance/tools/make_evidence_pictures.py
c0cc29db0f86f39510914219fb954bd2b0b15104247af6d1b5a31609f746590d  3+1super-loop_provenance/tools/make_dryrun.py
bcdef560a2e65808ce3f8b967dbf33e93792ec2947811b72c2d094083b28f8ca  3+1super-loop_provenance/tools/make_dryrun_part1.py
2f56f3cf7850a1f9b058f5e13d2a3af9809b1857fd719d33bbfe85e489d67b51  3+1super-loop_provenance/tools/gs13_ida_local_errors.wls
64f69bc4f59d6b86c6de7ee6a334f54474fec4b94b6d82f6548b03a74abdcb6c  3+1super-loop_provenance/tools/gs13_ida_tolerances.wls
12a60c5c03fa5e0bcd6ed8d9a85a64d0d45e17ce9e633c050a9f92653b17613b  3+1super-loop_provenance/tools/gs13_ida_lattice_probe.wl
ceafb181fbec77e8efb288b8f2c25ee73aba60c5121779d12da90840b1e2ad19  3+1super-loop_provenance/logs/T2-RNG_reference_values.txt
c64f1a909b9d008365fd5707547547cceb6f0df78b314099c676ba7d1e711f16  3+1super-loop_provenance/logs/E15_dot_precedence.txt
0b051cae79f44a26285ece8d21c4c46994342dd7a0f529d2358c35b621358647  3+1super-loop_provenance/logs/E17_tex_importer.txt
c719036debc776aab97c49fe2cb07f56e0e5bc7f14c0b4ee334f6e429265521b  3+1super-loop_provenance/logs/E16_figures_and_table_before_fix.png
dfe1af9e06b087239047f4f4ed05fea6bb9d27b0e14f4b738296b64df81e9298  3+1super-loop_provenance/logs/E16_figures_and_table_after_fix.png
8d1981333fc22357b76b79d9b6d30ef86579a24425067578d8fde2b48c67399a  3+1super-loop_provenance/logs/E17_text_formulas_before_and_after_fix.png
95f9e2e5adde8e72e471cd7a0bb0b81cf491c25a80e7b3857ca949c25e6eb82a  3+1super-loop_provenance/logs/E18_components_before_fix.png
87c29f8d4a0fedc884dce45f905201f9ac1a95c1b2b2052c35d02896d442c6f7  3+1super-loop_provenance/logs/E18_components_after_fix.png
b55155f8f8ca859dde1edefc11b68223b4cde4d0cc09d94c92157cacc8b9c393  3+1super-loop_provenance/logs/build_notebook.txt
47f6c8fb5422550ebb5d372b85e7d8d60663b0bda4b3b31320c8263ad6487d48  3+1super-loop_provenance/logs/dryrun_part1.txt
9a2614d92cc24297e156984df7dbd9a60dd67d13ca90e9829921b5ca6b157b6b  3+1super-loop_provenance/logs/dryrun_part2.txt
3c9bd1e72aadc55399c5bcc47f0f5a5f97e50615a83b3e79c36e4ba8aa648439  3+1super-loop_provenance/logs/IDA_local_errors_decay.txt
8c03a4f5885edfd63ef7a405ce43ca1ba7acb5cc7cc2b57e8a01e860b3c53436  3+1super-loop_provenance/logs/IDA_tolerance_scaling.txt
394a738c8bf9b2b632145610337efe21f9986f2e7ba9ed587cb2d9cbcc72fc49  3+1super-loop_provenance/logs/IDA_lattice_probe.txt
abac5b66af2e979c1f0d9ba86e463fde5c38dea5e314967d31c31dd3c2102e3f  3+1super-loop_provenance/logs/E22_formula_compare_report.txt
cdd35ec9f5cd09a283fcc6aab57c6ba226a002131423cd4422ff00d8c5d0f8ff  3+1super-loop_provenance/logs/E22_formula_inventory_committed_notebook.wl
a1af3d4a943185c792017966ffe31b7c48b8f138c55597d50cd14ff5894c2a08  3+1super-loop_provenance/logs/E22_formula_compare_committed_notebook.txt
26ec702c6f3c2f177fe34be9fa260a15d6af1a8c67e91fe5b27147a70dc7d5cf  3+1super-loop_provenance/logs/E22_formula_compare_report_committed_notebook.txt
397c3cadd58f21c65881bd3274d74573ddaa50e4b6c18a10a546eba4fa0f159a  3+1super-loop_provenance/logs/E22_formula_render_test.pdf
3eda29c302d6fbb20f9c9c709d50f6bf5768875d6497dec614e0510687f5a832  3+1super-loop_provenance/logs/E22_formulas_importer_and_own_reader.png
ff0efdd03a8fe51d860e00355e981840c599d2de6d4bd1db00b12648dad569d1  3+1super-loop_provenance/logs/E22_notebook_texts_before_and_after_fix.png
ef8b12927b5a1bbb45ae992bb9758f369bb1ab334614b25ad2423daf46e3593e  3+1super-loop_provenance/logs/E23_table_of_symbols_before_and_after_fix.png
475714cc43ca5fbbb5ec8c902ce1eb27db4af6647e5aac2e28e8727be9951abd  3+1super-loop_provenance/logs/E24_style_font_sizes.txt
8cd0076d7887f5e7736f89f0967735c2b108e63d02e428e8821403d85949e3eb  3+1super-loop_provenance/logs/E25_figure2_before_fix.png
cb1aa6fd66b12745dd81cddf799056e8f4af69606de5c16ccf532187bfcc1012  3+1super-loop_provenance/logs/E25_figure2_before_and_after_fix.png
49f12c4894cc62bc118570641b27e3b0c4d30372d3a87902a3717b0f32d55004  3+1super-loop_provenance/logs/E28_letter_digit_spacing_test.pdf
978af50142059b8bd9765ae33f65229b19a42e9a12bbebe7119f8f860588b060  3+1super-loop_provenance/logs/E28_letter_and_number_before_and_after_fix.png
a37bd5be8fefbdd4efe4784d7ce13b42f04a57c0c0bd558f66a45363c0322878  3+1super-loop_provenance/figures/notebook_figure2_gs13.png
```

# 17. Sources consulted

- The source code of the Notation package in the Wolfram 15.0.1 installation (`Notation.m`): the default of `WorkingForm`, the removal of `StyleBox`, `AdjustmentBox` and `FrameBox` before comparison, the conversion of strings that end in an underscore into patterns, the warning `Symbolize::bsymbexs` (E1, E5, E11).
- The Wolfram Language reference page of `SyntaxForm`: an option of `TagBox`, `InterpretationBox` and `TemplateBox` that specifies how a box takes part in the grammar of the surrounding input; by default the front end treats a box as an operand (E9). <https://reference.wolfram.com/language/ref/SyntaxForm.html>
- The reference page of `TagBox`: its options `StripWrapperBoxes` and `SyntaxForm` (E9). <https://reference.wolfram.com/language/ref/TagBox.html>
- The reference page of `StripBoxes`: when boxes are sent by the front end to the kernel for evaluation, the front end strips them with a more extensive process (E7). <https://reference.wolfram.com/language/ref/StripBoxes.html>
- The Notation package tutorial "Complex Patterns and Advanced Features": tag boxes and the option `SyntaxForm` in notations (E9). <https://reference.wolfram.com/language/Notation/tutorial/ComplexPatternsAndAdvancedFeatures.html>
- J. Fultz, answers on Mathematica Stack Exchange about the internal command of the front end that reads a string as typed input (its second argument `True` makes the stripping that is made at evaluation time) (E7).
- J. Fultz, MathGroup mailing list, 21 January 2014, "Re: Unnecessary parentheses around TagBox for MakeBoxes?": the value `SyntaxForm -> "symbol"` for a box that should act as a standalone thing (E9).
- The Wolfram Language tutorial "Operator Input Forms": the table of the operators in order of decreasing precedence, in which `expr1 . expr2` (`Dot`) stands above the implicit product `expr1 expr2` (`Times`) (E15). <https://reference.wolfram.com/language/tutorial/OperatorInputForms.html>
- The reference page of `SingleLetterItalics`: a cell option that controls whether names of one letter are printed in italics, typically `True` for cells of `TraditionalForm` (E18, E19). <https://reference.wolfram.com/language/ref/SingleLetterItalics.html>
- The documentation of SUNDIALS, chapter "Mathematical Considerations" of the integrator IDA: the backward differentiation formulas of variable order, 1 to 5; the weighted root-mean-square norm, in which the weight of a real unknown $y$ is $1/(r^{\rm tol}_{\rm LL}\lvert y\rvert+a^{\rm tol}_{\rm LL})$, the inverse of its tolerance (section 2.7); the test that the norm of the estimated local error is at most 1; the solution of the equations of a step by Newton's method; and the rule for the length of the next step after a successful step: the length is doubled if the norm of the error estimate of the order $k$ is at most $2^{-(k+2)}$, reduced by a factor between 0.5 and 0.9 if this norm is at least $1/2$, and kept otherwise, so that a step is lengthened only when it can be doubled (section 11.5, E20). <https://sundials.readthedocs.io/en/latest/ida/Mathematics_link.html>
- The Wolfram Language tutorial "IDA Method for NDSolve": IDA of SUNDIALS as a method of `NDSolve` for differential-algebraic equations, with backward differentiation formulas of the orders 1 to 5; the options `"ImplicitSolver"` (default `"Newton"`) and `"MaxDifferenceOrder"` (default 5) of the method; the tolerances are taken from the settings of `PrecisionGoal` and `AccuracyGoal`; the linear equations of Newton's method are solved by a method that `NDSolve` chooses (section 11.5, E21). The tutorial does not say that the method accepts real equations only; that is a measurement of this work. <https://reference.wolfram.com/language/tutorial/NDSolveIDAMethod.html>
- The Wolfram Language tutorial "Norms in NDSolve": the relative tolerance of `NDSolve` is 10 to the power minus the setting of `PrecisionGoal` and the absolute tolerance 10 to the power minus the setting of `AccuracyGoal`; the error of a step is scaled with them, and the tolerances are satisfied when the norm of the scaled error is below 1; most methods use the largest component as the norm, the method IDA a norm of the kind of the Euclidean norm (section 11.5; the meaning of the two tolerances was also measured, check T2-IDA). <https://reference.wolfram.com/language/tutorial/NDSolveVectorNorm.html>
- D. E. Knuth, The TeXbook (Addison-Wesley, 1984), chapter 20: an undelimited argument of a command is the next token, or the next group in braces (section 6.2, E22).
- The original programs of Test 2 in the repository (`Revision_SuperLoop/test2/rust`: the library `t2core` and the program `t2fields`), which Part II rewrites, and the record of their checks and the data files of their figures (`Revision_SuperLoop/test2/data`). Their SHA-256 digests, as used for this stage (the module `cvode.rs` is the time integration by CVODE of the original programs, and `t2_cvode_accuracy.csv` the file of their check GS-13, which the data cell of the notebook contains):

```
4a7beee875494e11a5778aa9bd94fca841bc5ab0abff6a3666a6ad25e72ac731  Revision_SuperLoop/test2/rust/t2core/src/rng.rs
8c412f5703dc49c7ba51fb86ff9a4fd4bcc3ce4667a11216884a4e2e2b6a43a3  Revision_SuperLoop/test2/rust/t2core/src/c64.rs
ace63b498d0e0fcbca02f1b659aa6249343f1b9ef9b16a113349bb9023915447  Revision_SuperLoop/test2/rust/t2core/src/cmat.rs
aecd9e01a40e211133cc8ff2c4697ec62e0370b46e0d7ea2d31fb19613a9eeb4  Revision_SuperLoop/test2/rust/t2core/src/model.rs
dab79f76867469c867f54f7596f5afe3541ff13ac20055e213fc1485b6770783  Revision_SuperLoop/test2/rust/t2core/src/lattice.rs
790b8758ce9bf919ffd93eb7f3afdaf66b4b4abb640a1fbf205146feeae6a8c7  Revision_SuperLoop/test2/rust/t2core/src/sym.rs
5ffa38243c640b9a0c56f599bd823d009f100d766827241d5bb916b2aa062489  Revision_SuperLoop/test2/rust/t2core/src/lib.rs
4d1f93b147745574fd24b0713b0fb1619c14e7b3274980ba05ed1474343e08e5  Revision_SuperLoop/test2/rust/t2core/src/cvode.rs
50b9903c673b420ba4ff1ef52d12fd4ba0b147628a01b9be687a26d182d0dbcf  Revision_SuperLoop/test2/rust/t2fields/src/main.rs
d7cf9c2d7e2d29f7e6e2e85fb076d35c89860cf92524e598ae2193ff30ba30c6  Revision_SuperLoop/test2/data/t2_fields_checks.txt
5e95b089a62bbb22de0736a71128c6aea361a085aecd83569447cca0462d89d7  Revision_SuperLoop/test2/data/t2_dispersion_lattice.csv
2f2849f03bb13b8475272648b8ec80617437c28a51afed9b7c2c11fabac2db01  Revision_SuperLoop/test2/data/t2_dispersion_curves.csv
c2c2fcc27332f8317b63377f93cb6a9b49788b1ba1ab39e2d347821dacc8e2b6  Revision_SuperLoop/test2/data/t2_cvode_accuracy.csv
```
