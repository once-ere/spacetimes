(* ::Package:: *)

(* ::Title:: *)
(*The 3+1 super-loop: the symbols and the programs of Test 2 in the Wolfram Language*)

(* ::Subtitle:: *)
(*PROJECT spacetimes. Notebook 3+1super-loop.nb, prepared for Patrick Lee Nash, Ph.D.*)

(* ::Text:: *)
(*This notebook is written for a physics student who has not seen the work before. It does two things. Part I gives every symbol of the project a Wolfram Language form that looks exactly like the printed symbol (for example $\Delta^{\rm vac}_{\rm LL}$, the gap of the condensed vacuum) and behaves as one indivisible symbol in computations; it then checks this with computations that use sums, arithmetic and algebra. Part II (added in later stages) collects, as an encyclopedia, every program of Test 2, rewritten in the Wolfram Language, with every check of the original programs.*)

(* ::Text:: *)
(*How to use it. Open the notebook in Wolfram (version 15 or later; it was evaluated with Wolfram 15.0.1) and choose Evaluation > Evaluate Notebook. Every cell is evaluated in order; every test prints PASS or FAIL, and the last cell of each part lists all tests. Nothing outside this notebook is read: the registry of the symbols and all programs are inside it.*)

(* ::Text:: *)
(*How to read it. Text cells explain, in words, what the next input cell does and why. Inside the code, comments (* like this *) explain each step. Every symbol of the physics is written with its printed form; every Wolfram Language function that is used is named in the text the first time it appears.*)

(* ::Chapter:: *)
(*Part I. The symbols*)

(* ::Section:: *)
(*I.1 Why every symbol of the project carries the suffix LL*)

(* ::Text:: *)
(*Papers and notebooks in physics often use one letter for several different objects: $\Delta$ may be a gap, a Laplacian or a difference; $m$ a mass or an index. In a long project with many documents and programs this produces errors that are hard to find. The project therefore keeps a registry: every symbol that it creates has a unique printed form, and the symbols of the super-loop revision carry the suffix LL, in a subscript or superscript (for example $G_{\rm LL}$, the coupling of the contact interaction, and $\tau^{\rm LL}_\iota$, the Pauli matrices of isospin). The registry lists for each symbol its printed form (written in TeX), its plain-text names (the names under which programs and data files write it), its meaning and its origin. Part I brings this whole registry into the Wolfram Language.*)

(* ::Text:: *)
(*In the Wolfram Language an ordinary subscript is not a symbol: the input $a_{\rm LL}$ is the expression `Subscript[a, LL]`, a function of two symbols. One cannot safely give it a value, and it can be confused with other subscripted expressions. Part I uses the Notation package, which is part of every Wolfram installation, to turn each printed form into one indivisible symbol (a "symbolized" symbol).*)

(* ::Section:: *)
(*I.2 The technique: symbolized symbols, created by a program*)

(* ::Text:: *)
(*The Notation package works on boxes, the structures that describe how an input looks on the screen. The two-dimensional input $a_{\rm LL}$ is the box `SubscriptBox["a", "LL"]`. The command `Symbolize[ParsedBoxWrapper[box], WorkingForm -> StandardForm]` tells the Wolfram Language: whenever an input contains exactly this box, read it as one new symbol, and whenever that symbol is printed, print it as this box. The wrapper `ParsedBoxWrapper` marks its argument as a box structure (this is the way to call `Symbolize` from a program; the palette of the package inserts an equivalent wrapper when one works by hand).*)

(* ::Text:: *)
(*The option `WorkingForm -> StandardForm` is essential here. Without it the package asks the front end for the format of the notebook in which the input was typed (`InputNotebook[]`). When a notebook is evaluated by a program, or when no notebook is selected, there is no such notebook, the answer is `$Failed`, and the package stores its reading rule for the format `$Failed`: the symbol is created but the input box is never recognized. This was found while this notebook was being prepared: the tests below failed until the option was given explicitly.*)

(* ::Text:: *)
(*The next cell loads the package and symbolizes one example, $a_{\rm LL}$. The cells after it show that the two-dimensional input is now one symbol, that it can receive a value, and how a program reaches the symbol from its box: `MakeExpression[box, StandardForm]` reads a box the same way the notebook reads input, and returns the result wrapped in `HoldComplete` (so that it is not evaluated).*)

(* ::Input:: *)
(* load the Notation package (it ships with every Wolfram installation) *)
Needs["Notation`"];
(* create the symbol a_LL from the box "a with the subscript LL" *)
Symbolize[ParsedBoxWrapper[SubscriptBox["a", "LL"]], WorkingForm -> StandardForm];
(* the name that the package gave to the new symbol (the separator is the character \[UnderBracket]) *)
Names["Global`a\[UnderBracket]*"]

(* ::Input:: *)
(* the box of a_LL, read as input, is now ONE symbol: its head is Symbol and it is an atom *)
With[{h = MakeExpression[SubscriptBox["a", "LL"], StandardForm]},
  {h, Head @@ h, AtomQ @@ h}]

(* ::Text:: *)
(*The ordinary expression `Subscript[a, LL]`, typed as text, is NOT the symbol: only the box is symbolized. A program therefore reaches a symbolized symbol through its box, with `MakeExpression`, which returns `HoldComplete[symbol]`. To give it a value, the held symbol is put into an assignment with `Apply` (written `@@`): `Set @@ HoldComplete[symbol, value]` replaces the head `HoldComplete` by `Set` and gives `Set[symbol, value]`; because `Set` does not evaluate its first argument, the symbol itself (and not its old value) receives the value. In the same way `Clear @@ HoldComplete[symbol]` gives `Clear[symbol]`.*)

(* ::Input:: *)
(* the text input Subscript[a, LL] is a compound expression, not the symbol *)
{Head[Subscript[a, LL]], AtomQ[Subscript[a, LL]]}

(* ::Input:: *)
(* assign the value 42 to the symbol a_LL through its box, then read it back:
   Join[HoldComplete[a_LL], HoldComplete[42]] is HoldComplete[a_LL, 42], and Set @@ ... is a_LL = 42 *)
With[{s = MakeExpression[SubscriptBox["a", "LL"], StandardForm]},
  Set @@ Join[s, HoldComplete[42]]];
ReleaseHold[MakeExpression[SubscriptBox["a", "LL"], StandardForm]] + 1

(* ::Input:: *)
(* remove the value again, so that a_LL is a free symbol for the rest of the notebook:
   Clear @@ HoldComplete[a_LL] is Clear[a_LL] *)
With[{s = MakeExpression[SubscriptBox["a", "LL"], StandardForm]}, Clear @@ s];
ReleaseHold[MakeExpression[SubscriptBox["a", "LL"], StandardForm]]

(* ::Text:: *)
(*A symbolized symbol is one indivisible atom: the letters inside its scripts are not separate symbols. The next cell shows this with the symbol $a_{{\rm LL},p}$, whose subscript holds the label LL and the index letter $p$. In a sum whose summation variable is also called `LL`, the whole symbol is multiplied by the summation variable, so that $\sum_{{\rm LL}=1}^{2}{\rm LL}\,a_{{\rm LL},p}=3\,a_{{\rm LL},p}$: the LL inside the subscript is untouched. For comparison, the ordinary subscripted expression `Subscript[b, LL, p]`, which is not symbolized, is a compound expression in which `LL` is replaced: its sum is $b_{1,p}+2\,b_{2,p}$.*)

(* ::Text:: *)
(*One rule must be observed when a box is symbolized by a program: the box must be exactly the box that the front end sends to the kernel when the symbol is typed. Before an input cell is evaluated, the front end reads every string of its boxes again, as typed text, and cuts it into tokens: names (a letter followed by letters and digits), numbers and single other characters. The subscript of a typed $a_{{\rm LL},p}$ therefore arrives as `RowBox[{"LL", ",", "p"}]`, three tokens. A box written by hand as `SubscriptBox["a", "LL,p"]`, with the whole subscript in one string, is never matched by typed input, because the front end never sends that string. This was found while the notebook was prepared (a test notebook, evaluated in the front end, showed exactly this), and the converter of section I.3 builds the cut form.*)

(* ::Input:: *)
(* symbolize a_{LL,p}: the subscript "LL, p" exactly as the front end stores it, three tokens in a RowBox *)
Symbolize[ParsedBoxWrapper[SubscriptBox["a", RowBox[{"LL", ",", "p"}]]], WorkingForm -> StandardForm];
(* the summation variable is called LL; the symbol a_{LL,p} is one atom, so its subscript is untouched *)
Sum[LL LL["a_{{\\rm LL},p}"], {LL, 1, 2}]
(* the same sum, compared with 3 a_{LL,p} *)
3 LL["a_{{\\rm LL},p}"] === Sum[LL LL["a_{{\\rm LL},p}"], {LL, 1, 2}]
(* the ordinary, NOT symbolized, subscripted expression: here LL is replaced by 1 and 2 *)
Sum[LL Subscript[b, LL, p], {LL, 1, 2}]

(* ::Input:: *)
(* the name LL is used later for the function LL of the package of section I.3; the summation
   variable LL of the demonstration (a symbol of the context Global`) and the symbol b are removed,
   so that they cannot shadow it *)
Remove[LL, b];
Names["Global`a\[UnderBracket]*"]

(* ::Section:: *)
(*I.3 From the printed form (TeX) to boxes*)

(* ::Text:: *)
(*The registry stores every symbol in TeX, the language in which the documents of the project are typeset; for example the gap of the condensed vacuum is `\Delta^{\rm vac}_{\rm LL}`. The next cell defines, in the context SL31, a small reader of exactly this kind of TeX. It splits the TeX into tokens, reads them from left to right, and builds the box. A command such as `\Delta` becomes the Greek letter; `^{...}` and `_{...}` become a superscript and a subscript; `{\rm LL}` becomes the upright text LL (roman text that holds several tokens, such as `{\rm int,F}` or `{\rm s(2)}`, is cut into tokens exactly as the front end cuts typed text, section I.2); `\vec d` becomes a d with an arrow. Letters in special alphabets keep their alphabet: double-struck ($\mathbb R$), script ($\mathcal L$) and Gothic ($\mathfrak m$) letters are separate characters of the Wolfram Language, and sans-serif ($\mathsf N$) and bold ($\mathbf x$) letters are wrapped in a `TagBox`, which the Notation package does not remove when it compares boxes, so that a bold, a sans-serif and an ordinary letter with the same scripts remain three different symbols.*)

(* ::Text:: *)
(*The same cell defines the functions that split an entry of the registry into single symbols, classify every item (a symbol, a family of symbols with running indices, a function symbol, a group, an index, an operator, and so on), give every symbolized symbol a readable name, symbolize it, and reach it by its TeX: `LL[tex]` returns the symbol of the TeX item `tex`, and `LL[tex, index -> value]` the symbol of one of its components. In the input cells of this notebook the symbols appear directly in their printed form.*)

(* ::Input:: *)
(* @@INCLUDE sl31_core.wl@@ *)

(* ::Text:: *)
(*A first look at the reader: the TeX of six registered symbols and the boxes that it builds, shown both as boxes and as they are displayed.*)

(* ::Input:: *)
examples = {"\\Delta^{\\rm vac}_{\\rm LL}", "A^\\iota_{{\\rm LL},p}", "\\mathsf N^{\\rm g}_{\\rm LL}(\\vec d_{\\rm LL})",
   "\\vec d^{\\,*}_{\\rm LL}", "\\mathfrak m_{\\rm LL}", "\\lvert\\Phi^{\\rm SF}_{\\rm LL}\\rangle"};
Grid[Prepend[Map[{#, Style[ToString[SL31TeXToBoxes[#], InputForm], FontFamily -> "Source Code Pro", 9], RawBoxes[SL31TeXToBoxes[#]]} &, examples],
   Style[#, Bold] & /@ {"TeX", "boxes", "displayed"}], Frame -> All, Alignment -> Left]

(* ::Text:: *)
(***Complete formulas.** The texts of this notebook and the meanings of the registry (section I.8) contain complete formulas, written in TeX between dollar signs, for example `B^{\rm out}_{\rm LL}=\big(1+\tfrac14h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL}\big)B^{\rm err}_{\rm LL}`. A second reader, defined in the next cell, converts them into boxes. It is built like the reader of symbols (tokens, read from left to right), and it knows what a complete formula needs: fractions and roots, sums and integrals with their limits, accents, matrices, delimiters, the names of functions, and the spaces of TeX. Four properties matter for the correctness of what is shown.*)

(* ::Item:: *)
(***The arguments of a command.** In TeX an argument that is not written in braces is the single token that follows the command: `\tfrac12\ell^{\rm x}_{\rm LL}` is one half times $\ell^{\rm x}_{\rm LL}$, and `\sqrt2\,\ell^{\rm x}_{\rm LL}` is the square root of 2, times $\ell^{\rm x}_{\rm LL}$. The reader follows this rule. The TeX importer of the Wolfram Language (`ImportString` with the format "TeX") does not: it takes everything up to the end of the group as the denominator and everything before the fraction as part of the numerator, so that it shows `\tfrac12\ell^{\rm x}_{\rm LL}` as the fraction with the numerator 1 and the denominator $2\ell^{\rm x}_{\rm LL}$, and `Q_{\rm LL}=-\tfrac12` as the fraction with the numerator $Q_{\rm LL}=-1$ and the denominator 2; it also shows `\hbar` as nothing. The importer is therefore not used anywhere in this notebook.*)

(* ::Item:: *)
(***The alphabets.** Every letter is given its alphabet by an explicit style: the letter of a variable is set in italics, text written with `\rm` upright, `\mathbf` bold and upright, `\mathsf` sans-serif (in the typeface Helvetica), and the double-struck, script and Gothic letters are the characters of the Wolfram Language for them. A formula is set in the typeface Times, so that an upright roman letter (the label K of $\ell^{\rm K}_{\rm LL}$) differs visibly from a sans-serif letter (the matrix $\mathsf K^3_{\rm LL}$) and from an italic letter (the generator $K^j_{\rm LL}$); the text around a formula is set in the sans-serif typeface of the text cells.*)

(* ::Item:: *)
(***Names are not products.** TeX sets neighbouring letters and numbers without a space. The front end of the Wolfram Language puts the thin space of a product between two neighbouring items of a row of boxes, and this notebook keeps that space between the factors of a product, as the Wolfram Language itself shows a product. Inside a name there must be no such space. The reader therefore joins neighbouring letters of one alphabet into one string (the letters `SL` and `SU` in the names of groups), and it puts the character `\[InvisibleSpace]`, which has no width, between a letter and a number that directly follows it: the coordinates `x5`, `x6`, `x7` of the main project (they occur in the meaning of the number of space dimensions, section I.8) are names, not products of a letter and a number.*)

(* ::Item:: *)
(***Nothing is passed over in silence.** A command that the reader does not know, a missing argument or an unmatched brace is recorded: `SL31TeXFormulaProblems[tex]` lists the problems of a formula, and the list must be empty. The program that builds this notebook stops with a warning for every problem in a text cell, and test S-15 (section I.7) checks every formula of the registry.*)

(* ::Text:: *)
(*The function `SL31FormulaReading` writes the boxes of a formula on one line (a fraction as `frac{a}{b}`, a root as `sqrt{a}`, scripts as `_{..}` and `^{..}`, a bold or sans-serif letter as `<bf:b>` or `<sf:N>`), without the spaces and without the italic and upright styles. Two conversions of a formula show the same formula exactly if their readings are equal; the tests use this.*)

(* ::Input:: *)
(* @@INCLUDE sl31_formula.wl@@ *)

(* ::Text:: *)
(*A first look at the reader of formulas: the TeX of seven formulas, the reading of the boxes on one line, and the formula as it is displayed.*)

(* ::Input:: *)
formulaExamples = {"\\tfrac12\\ell^{\\rm x}_{\\rm LL}g^{\\rm v}_{\\rm LL}", "\\cos(\\tfrac12\\vartheta^{\\rm B}_{\\rm LL})", "\\sqrt2\\,\\ell^{\\rm x}_{\\rm LL}",
   "-\\tfrac{15}8\\int_{q_{\\rm LL}<\\Lambda_{\\rm LL}}d^3q_{\\rm LL}", "\\mathbf b=\\sum_{p=0}^3\\xi^{\\rm LL}_p(\\mathbf b)\\mathbf e_p",
   "\\lvert\\vec d_{\\rm LL}\\rvert^2/G_{\\rm LL}", "\\sigma^{\\rm LL}_2=\\begin{pmatrix}0&-i\\\\i&0\\end{pmatrix}"};
Grid[Prepend[Map[{Style[#, FontFamily -> "Source Code Pro", 9], Style[SL31FormulaReading[SL31TeXFormula[#]], FontFamily -> "Source Code Pro", 9],
      RawBoxes[SL31FormulaCell[#]]} &, formulaExamples],
   Style[#, Bold] & /@ {"TeX", "reading on one line", "displayed"}], Frame -> All, Alignment -> Left]

(* ::Section:: *)
(*I.4 The registry of the symbols*)

(* ::Text:: *)
(*The next cell holds the complete registry of the symbols of the super-loop revision: 204 entries. Every entry is an association with the keys "entry" (its number), "group" (one of nine groups), "symbol" (the printed forms, in TeX, between dollar signs), "names" (the plain-text names used by programs and data files), "meaning", "meaningEdited" (True where the meaning was restated so that this notebook is complete in itself: the original wording pointed to equations, theorems or sections of other texts) and "origin".*)

(* ::Input:: *)
(* @@INCLUDE registry_data.wl@@ *)
Length[SL31Registry]

(* ::Input:: *)
(* the number of entries in each of the nine groups *)
KeySort[Counts[Map[#["group"] &, SL31Registry]]]

(* ::Section:: *)
(*I.5 The single symbols and their kinds*)

(* ::Text:: *)
(*An entry may list several symbols, for example "$m_{\rm LL}$, $\mu_{\rm LL}$, $\Lambda_{\rm LL}$" (the mass of the spinor charges, the chemical potential and the momentum cutoff). The next cell takes every TeX segment between dollar signs, splits it into single items, converts each into boxes, and classifies it. The kinds are: symbol (one fixed object, such as $\Delta^{\rm vac}_{\rm LL}$); family (a symbol with a running index, such as $\tau^{\rm LL}_\iota$ with $\iota=1,2,3$); function and function family (a symbol followed by its argument, such as $W_{\rm LL}(\mathbf u)$); letter (a single letter such as $D$, the number of space dimensions); index and primed index ($p$, $p'$, $j$, $\iota$, ...); set ($\mathbb R$, $\mathbb C$, ...); group (${\rm SL}(2,\mathbb C)$, ${\rm SU}(2)_{\rm v}$, ...); ket; operator (tr, det, the transpose, the tensor product, ...); expression (a combination such as $(T_{\rm LL})^2$); imaginary unit. Entries that are not written in TeX at all are the labels of the machine checks and of the hypotheses of the no-go theorems; they are names, not symbols, and are kept as text.*)

(* ::Input:: *)
(* every TeX segment of every entry: {entry number, segment} *)
segments = Flatten[Map[Function[e, Map[{e["entry"], #} &, StringCases[e["symbol"], "$" ~~ Shortest[x__] ~~ "$" :> x]]], SL31Registry], 1];
(* the entries without any TeX: labels *)
labelEntries = Select[SL31Registry, ! StringContainsQ[#["symbol"], "$"] &];
(* split every segment into single items and convert each item *)
items = Flatten[Map[Function[s, Map[<|"entry" -> s[[1]], "tex" -> #, "boxes" -> SL31TeXToBoxes[#]|> &, SL31SplitTeX[s[[2]]]]], segments], 1];
items = Map[Append[#, "kind" -> SL31Classify[#["tex"], #["boxes"]]] &, items];
{Length[segments], Length[items], Length[labelEntries]}

(* ::Input:: *)
(* how many items of each kind *)
ReverseSort[Counts[Map[#["kind"] &, items]]]

(* ::Section:: *)
(*I.6 Symbolizing every symbol*)

(* ::Text:: *)
(*Now every item of the kinds symbol, family, function, function family, letter, primed index, group, ket and expression is symbolized: `SL31Symbolize` calls `Symbolize` with the explicit working form and with a readable name built from the letters, the scripts and the alphabet, and returns the new symbol, held. Items whose boxes coincide are ONE symbol and are symbolized once: a function symbol written with different arguments, such as $N_{\rm LL}(\mathbf x)$ and $N_{\rm LL}(\mathbf x,\mathbf y)$, has the one head $N_{\rm LL}$ (symbolizing the same box twice would only replace the first symbolization, with a warning of the Notation package). For every new symbol the cell also stores its meaning as the usage message (so that `?symbol` prints it), and a formatting rule for `TraditionalForm`: `TeXForm` writes TeX from the `TraditionalForm` boxes of an expression, so with this rule the symbol is written in TeX with its printed form and not with its internal name.*)

(* ::Input:: *)
(* the items that are symbolized, each TeX item once *)
toSymbolize = DeleteDuplicatesBy[Select[items, MemberQ[SL31Symbolized, #["kind"]] &], #["tex"] &];
(* items with the same boxes are one symbol: they are gathered into groups *)
byBoxes = GatherBy[toSymbolize, SL31SymbolBoxes[#["tex"]] &];
meaningOf = Association[Map[#["entry"] -> #["meaning"] &, SL31Registry]];
(* symbolize every group; record the TeX items, the entries, the kind, the boxes and the held symbol *)
symbolTable = Map[Function[group, Module[{b = SL31SymbolBoxes[First[group]["tex"]], h, texs, entries},
      texs = Map[#["tex"] &, group];
      entries = DeleteDuplicates[Map[#["entry"] &, group]];
      h = SL31Symbolize[b];
      (* the usage message: the TeX items of the symbol and the meanings of their entries *)
      Replace[h, HoldComplete[s_Symbol] :>
        (MessageName[s, "usage"] = StringRiffle[texs, ", "] <> ": " <> StringRiffle[Map[meaningOf, entries], " | "])];
      (* TraditionalForm (and therefore TeXForm) shows the printed form, not the internal name *)
      Replace[h, HoldComplete[s_Symbol] :> (MakeBoxes[s, TraditionalForm] = b)];
      <|"entries" -> entries, "tex" -> First[texs], "texAll" -> texs, "kind" -> First[group]["kind"],
        "boxes" -> b, "symbol" -> h|>]],
   byBoxes];
{Length[toSymbolize], Length[symbolTable]}

(* ::Text:: *)
(*The TeX items that share one symbol, with their entries:*)

(* ::Input:: *)
Grid[Prepend[Map[{RawBoxes[#["boxes"]], Column[#["texAll"]], #["entries"]} &, Select[symbolTable, Length[#["texAll"]] > 1 &]],
   Style[#, Bold] & /@ {"symbol", "TeX items", "entries"}], Frame -> All, Alignment -> Left]

(* ::Text:: *)
(*Twelve symbols chosen from the nine groups, as the notebook now prints them, with their internal names:*)

(* ::Input:: *)
Grid[Prepend[Map[{RawBoxes[#["boxes"]], #["kind"], Replace[#["symbol"], HoldComplete[s_] :> SymbolName[Unevaluated[s]]]} &,
    symbolTable[[Round[Range[1, Length[symbolTable], (Length[symbolTable] - 1)/11]]]]],
   Style[#, Bold] & /@ {"symbol", "kind", "internal name"}], Frame -> All, Alignment -> Left]

(* ::Section:: *)
(*I.7 Verification tests of the symbols*)

(* ::Text:: *)
(*Every test below has a name (S-1, S-2, ...), states in words what it checks, computes it, and records PASS or FAIL with the numbers that decide it in the list ``SL31`Test`$Results``. The function `SL31Check[name, statement, condition, details]` does the recording and prints one line.*)

(* ::Input:: *)
SL31`Test`$Results = {};
SL31Check[name_String, statement_String, ok_, details_ : ""] := (
   AppendTo[SL31`Test`$Results, <|"name" -> name, "pass" -> TrueQ[ok], "statement" -> statement, "details" -> details|>];
   (* the details are printed as an expression, so that the symbols appear in their printed form *)
   Print[Style[If[TrueQ[ok], "PASS  ", "FAIL  "], Bold, If[TrueQ[ok], Darker[Green], Red]], name, ": ", statement,
     If[details === "", "", Row[{"  -- ", details}]]]);

(* ::Text:: *)
(*S-1 and S-2: every TeX item was converted into boxes (no item failed), and every symbolized box is now read as one atomic symbol.*)

(* ::Input:: *)
SL31Check["S-1", "every one of the TeX items of the registry is converted into a box structure",
  AllTrue[items, FreeQ[#["boxes"], $Failed | Null] &], Length[items]];
SL31Check["S-2", "every symbolized box is read as one atomic symbol (head Symbol)",
  AllTrue[symbolTable, MatchQ[#["symbol"], HoldComplete[_Symbol]] &],
  Count[symbolTable, r_ /; MatchQ[r["symbol"], HoldComplete[_Symbol]]]];

(* ::Text:: *)
(*S-0, the strings as the front end reads them. Every string inside every symbolized box (the letters, names and numbers of the scripts) must be ONE token when the front end reads it as typed text; otherwise the front end would cut it before it sends a typed symbol to the kernel, and the symbol would not be recognized (section I.2). The test asks the front end itself. The command ``FrontEndExecute[FrontEnd`UndocumentedTestFEParserPacket[string, True]]`` is an internal command of the front end: it reads a string exactly as typed input is read when a cell is evaluated, and returns the boxes; for a single token it returns the string unchanged. (The tags of `TagBox` and the options of `StyleBox` are not input text and are not tested.)*)

(* ::Input:: *)
(* the strings inside a box: TagBox and StyleBox wrappers are opened first *)
boxStrings[b_] := Cases[b //. {TagBox[x_, _String, ___] :> x, StyleBox[x_, ___] :> x}, _String, {0, Infinity}];
allStrings = Union[Flatten[Map[boxStrings[#["boxes"]] &, symbolTable]]];
(* how the front end reads every one of them *)
feReading = Map[# -> First[FrontEndExecute[FrontEnd`UndocumentedTestFEParserPacket[#, True]]] &, allStrings];
cut = Select[feReading, ! MatchQ[#, (s_ -> BoxData[s_])] &];
SL31Check["S-0", "every string inside the symbolized boxes is one token for the front end, so that typed symbols reach the kernel unchanged",
  cut === {}, {Length[allStrings], cut}];

(* ::Text:: *)
(*S-14, the typed symbols. The next cell contains every symbolized symbol of the registry, each written in its printed form, exactly as a reader of this notebook would type it. When the cell is evaluated, the front end sends these boxes to the kernel and the Notation package reads them. Every one must be one atomic symbol, and it must be the very symbol that section I.6 created from its registry TeX (the list has the order of the table `symbolTable`).*)

(* ::Input:: *)
(* @@INCLUDE all_symbols.wl@@ *)
SL31Check["S-14", "every symbol typed in its printed form is read as one atomic symbol, the one created from its registry TeX",
  Length[typed] == Length[symbolTable] && AllTrue[typed, AtomQ] && typed === Map[ReleaseHold[#["symbol"]] &, symbolTable],
  {Length[typed], Count[typed, _?AtomQ], Length[symbolTable]}];

(* ::Text:: *)
(*S-3, uniqueness: different printed forms give different symbols. If two different boxes had received the same name, two printed symbols would have become one Wolfram symbol. The test compares the number of different boxes with the number of different symbols, and also checks the comparison that the Notation package itself makes: it removes style boxes (StyleBox, AdjustmentBox, FrameBox) before it compares input with a symbolized box, so the boxes must still be different after this removal. (TeX items with the same boxes, listed in section I.6, are one symbol by construction.)*)

(* ::Input:: *)
strip[b_] := b //. {StyleBox[x_, ___] :> x, AdjustmentBox[x_, ___] :> x, FrameBox[x_, ___] :> x};
syms = Map[#["symbol"] &, symbolTable];
SL31Check["S-3", "different boxes give different symbols, and the boxes stay different after the removal of style boxes",
  Length[DeleteDuplicates[Map[#["boxes"] &, symbolTable]]] == Length[DeleteDuplicates[syms]] ==
   Length[DeleteDuplicates[Map[strip[#["boxes"]] &, symbolTable]]] == Length[symbolTable],
  {Length[symbolTable], Length[DeleteDuplicates[syms]], Length[DeleteDuplicates[Map[strip[#["boxes"]] &, symbolTable]]]}];

(* ::Text:: *)
(*S-4, the round trip: a symbol, printed again, gives exactly the box it was made from (`ToBoxes[symbol, StandardForm]`). This is the other half of the symbolization: input box to symbol (S-2) and symbol to output box (S-4).*)

(* ::Input:: *)
roundTrip = Map[Function[r, Replace[r["symbol"], HoldComplete[s_] :> (ToBoxes[Unevaluated[s], StandardForm] === r["boxes"])]], symbolTable];
SL31Check["S-4", "every symbol prints as the box it was made from", And @@ roundTrip, Count[roundTrip, True]];

(* ::Text:: *)
(*S-5, TeX: with its `TraditionalForm` rule, every symbol is written in TeX from its displayed boxes; the internal name, with its separator character, does not appear.*)

(* ::Input:: *)
texOut = Map[Function[r, Replace[r["symbol"], HoldComplete[s_] :> ToString[TeXForm[Unevaluated[s]]]]], symbolTable];
SL31Check["S-5", "the TeX of every symbol is written from its printed form, without the internal name",
  AllTrue[texOut, ! StringContainsQ[#, "UnderBracket" | "\[UnderBracket]"] &], Count[texOut, x_ /; ! StringContainsQ[x, "UnderBracket"]]];
TableForm[Take[Transpose[{Map[#["tex"] &, symbolTable], texOut}], 8], TableHeadings -> {None, {"registry TeX", "TeXForm of the symbol"}}]

(* ::Text:: *)
(*S-6, arithmetic with symbols and values: the coupling $G_{\rm LL}$, the mass $m_{\rm LL}$ and the cutoff $\Lambda_{\rm LL}$ of the spinor charges receive the values used throughout Test 2 ($m_{\rm LL}=0.2$, $\Lambda_{\rm LL}=1$, $G_{\rm LL}=22.5$, units in which the cutoff is 1). The gap of the condensed vacuum at zero chemical potential solves the gap equation $1/G_{\rm LL}=\big(\Lambda_{\rm LL}E_\Lambda-(M^{\rm q}_{\rm LL})^2\,{\rm arsinh}(\Lambda_{\rm LL}/M^{\rm q}_{\rm LL})\big)/(2\pi^2)$ with $(M^{\rm q}_{\rm LL})^2=m_{\rm LL}^2+(\Delta^{\rm vac}_{\rm LL})^2$ and $E_\Lambda=(\Lambda_{\rm LL}^2+(M^{\rm q}_{\rm LL})^2)^{1/2}$. The cell writes this equation with the symbolized symbols, gives them values only inside `Block` (so that they stay free symbols outside), solves for the gap with `FindRoot`, and compares with the value 0.2094933033 found by the Rust program of Test 2. It also checks the derived masses $2\Delta^{\rm vac}_{\rm LL}$ and $2M^{\rm q}_{\rm LL}$.*)

(* ::Input:: *)
gapEquation = 1/LL["G_{\\rm LL}"] == (LL["\\Lambda_{\\rm LL}"] Sqrt[LL["\\Lambda_{\\rm LL}"]^2 + LL["m_{\\rm LL}"]^2 + LL["\\Delta^{\\rm vac}_{\\rm LL}"]^2] -
     (LL["m_{\\rm LL}"]^2 + LL["\\Delta^{\\rm vac}_{\\rm LL}"]^2) ArcSinh[LL["\\Lambda_{\\rm LL}"]/Sqrt[LL["m_{\\rm LL}"]^2 + LL["\\Delta^{\\rm vac}_{\\rm LL}"]^2]])/(2 Pi^2)

(* ::Input:: *)
(* exact values (45/2, 1/5, 1), so that FindRoot can work with 30 digits *)
gapValue = Block[{LL["G_{\\rm LL}"] = 45/2, LL["m_{\\rm LL}"] = 1/5, LL["\\Lambda_{\\rm LL}"] = 1},
   LL["\\Delta^{\\rm vac}_{\\rm LL}"] /. FindRoot[gapEquation, {LL["\\Delta^{\\rm vac}_{\\rm LL}"], 0.2}, WorkingPrecision -> 30, AccuracyGoal -> 20]];
SL31Check["S-6", "the gap equation written with the symbols gives the gap of the condensed vacuum 0.2094933033 at G = 22.5, m = 0.2, cutoff 1; 2 Delta_vac = 0.4189866 and 2 M_q = 0.5792666",
  Abs[gapValue - 0.2094933033] < 1*^-10 && Abs[2 gapValue - 0.4189866] < 1*^-7 && Abs[2 Sqrt[0.04 + gapValue^2] - 0.5792666] < 1*^-7,
  N[gapValue, 12]];

(* ::Text:: *)
(*S-7, sums over a family of symbols: the Pauli matrices. The registry symbol $\sigma^{\rm LL}_j$ is a family with the running index $j=1,2,3$. A symbolized symbol is indivisible, so the index letter inside it cannot be replaced by a number in a sum. The components are therefore separate symbols: `LL[tex, "j" -> "1"]` (written in the input cells as the printed component $\sigma^{\rm LL}_1$) symbolizes and returns the component with $j=1$. The cell gives the three components the Pauli matrices as values, together with the identity $\mathsf 1_2$, and checks two identities that involve sums over the index: $\sum_j\sigma^{\rm LL}_j\sigma^{\rm LL}_j=3\,\mathsf 1_2$, and the product rule $\sigma^{\rm LL}_j\sigma^{\rm LL}_{j'}=\delta_{jj'}\mathsf 1_2+i\sum_{j''}\epsilon^{\rm LL}_{jj'j''}\sigma^{\rm LL}_{j''}$, in which the 27 components of the symbol $\epsilon^{\rm LL}_{jj'j''}$ (the totally antisymmetric symbol with $\epsilon^{\rm LL}_{123}=1$) are symbolized and given their values.*)

(* ::Input:: *)
(* symbolize the components sigma^LL_1, sigma^LL_2, sigma^LL_3 and epsilon^LL_{j j' j''} (j, j', j'' = 1, 2, 3) *)
sigma = Table[ReleaseHold[SL31Instance[SL31SymbolBoxes["\\sigma^{\\rm LL}_j"], {"j" -> ToString[k]}]], {k, 3}];
eps = Table[ReleaseHold[SL31Instance[SL31SymbolBoxes["\\epsilon^{\\rm LL}_{jj'j''}"],
      {"j" -> ToString[a], SuperscriptBox["j", "\[Prime]"] -> ToString[b], SuperscriptBox["j", "\[Prime]\[Prime]"] -> ToString[c]}]], {a, 3}, {b, 3}, {c, 3}];
{sigma, eps[[1, 2, 3]], eps[[2, 1, 3]]}

(* ::Input:: *)
(* the values: inside Block only, so that the symbols stay free elsewhere *)
pauliValues = {{{0, 1}, {1, 0}}, {{0, -I}, {I, 0}}, {{1, 0}, {0, -1}}};
check7 = Block[{LL["\\sigma^{\\rm LL}_j", "j" -> "1"] = pauliValues[[1]], LL["\\sigma^{\\rm LL}_j", "j" -> "2"] = pauliValues[[2]],
     LL["\\sigma^{\\rm LL}_j", "j" -> "3"] = pauliValues[[3]], LL["\\mathsf 1_2"] = IdentityMatrix[2]},
   Module[{sum, rule},
     (* the sum over j of sigma_j . sigma_j, written with the three components *)
     sum = Sum[sigma[[j]] . sigma[[j]], {j, 3}] - 3 LL["\\mathsf 1_2"];
     (* the product rule for all nine pairs (j, j'), with the 27 symbolized components of epsilon *)
     rule = Table[sigma[[j]] . sigma[[jp]] - (KroneckerDelta[j, jp] LL["\\mathsf 1_2"] +
           I Sum[(eps[[j, jp, jpp]] /. Thread[Flatten[eps] -> Flatten[Normal[LeviCivitaTensor[3]]]]) sigma[[jpp]], {jpp, 3}]), {j, 3}, {jp, 3}];
     {Max[Abs[sum]], Max[Abs[Flatten[rule]]]}]];
SL31Check["S-7", "sum over j of sigma^LL_j sigma^LL_j = 3 times the unit matrix, and sigma_j sigma_j' = delta_jj' + i sum_j'' epsilon_jj'j'' sigma_j'' for all nine pairs (components as symbolized symbols)",
  check7 === {0, 0}, check7];

(* ::Text:: *)
(*S-8, a double sum with a metric: the Minkowski square. The family $x^p_{\rm LL}$ (the coordinates of space-time, $p=0,1,2,3$) and the metric $\eta^{\rm LL}_{pp'}$ ($+1$ for $p=p'=0$, $-1$ for $p=p'=1,2,3$, zero otherwise) are symbolized component by component; the double sum $\sum_{p,p'}\eta^{\rm LL}_{pp'}x^p_{\rm LL}x^{p'}_{\rm LL}$ is formed with the 16 components of the metric as free symbols, then the metric receives its values and the sum must become $(x^0_{\rm LL})^2-\sum_j(x^j_{\rm LL})^2$, with the coordinate components remaining free symbols. The test is algebraic: `Expand` and a comparison of polynomials in the four coordinate symbols.*)

(* ::Input:: *)
xs = Table[ReleaseHold[SL31Instance[SL31SymbolBoxes["x^p_{\\rm LL}"], {"p" -> ToString[k]}]], {k, 0, 3}];
etas = Table[ReleaseHold[SL31Instance[SL31SymbolBoxes["\\eta^{\\rm LL}_{pp'}"], {"p" -> ToString[a], SuperscriptBox["p", "\[Prime]"] -> ToString[b]}]], {a, 0, 3}, {b, 0, 3}];
quadratic = Sum[etas[[a, b]] xs[[a]] xs[[b]], {a, 4}, {b, 4}];
minkowski = Expand[quadratic /. Thread[Flatten[etas] -> Flatten[DiagonalMatrix[{1, -1, -1, -1}]]]];
{Length[quadratic], minkowski}

(* ::Input:: *)
SL31Check["S-8", "the double sum of eta^LL_pp' x^p x^p' over p, p' = 0..3 (16 + 4 symbolized components) is (x^0)^2 - (x^1)^2 - (x^2)^2 - (x^3)^2",
  Expand[minkowski - (xs[[1]]^2 - xs[[2]]^2 - xs[[3]]^2 - xs[[4]]^2)] === 0 && Length[quadratic] == 16, minkowski];

(* ::Text:: *)
(*S-9, algebra with the symbols of the Higgs mechanism of the superfluid boson singlet state. The rest masses of the two charged vector bosons and of the vector boson of the fermion number are $(M^{\rm W}_{\rm LL})^2=2Z^{\rm s}_{\rm LL}(g^{\rm v}_{\rm LL})^2(\Delta^{\rm vac}_{\rm LL})^2$ and $(M^{\rm F}_{\rm LL})^2=8Z^{\rm s}_{\rm LL}(g^{\rm F}_{\rm LL})^2(\Delta^{\rm vac}_{\rm LL})^2$, where $Z^{\rm s}_{\rm LL}$ is the stiffness of the space derivatives and $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ are the gauge couplings. The cell checks with `Simplify` that the ratio of the squared masses is $4(g^{\rm F}_{\rm LL}/g^{\rm v}_{\rm LL})^2$, independent of the stiffness and of the gap, and with `Solve` that the gap can be recovered from $M^{\rm W}_{\rm LL}$.*)

(* ::Input:: *)
massRules = {LL["M^{\\rm W}_{\\rm LL}"] -> Sqrt[2 LL["Z^{\\rm s}_{\\rm LL}"]] LL["g^{\\rm v}_{\\rm LL}"] LL["\\Delta^{\\rm vac}_{\\rm LL}"],
   LL["M^{\\rm F}_{\\rm LL}"] -> Sqrt[8 LL["Z^{\\rm s}_{\\rm LL}"]] LL["g^{\\rm F}_{\\rm LL}"] LL["\\Delta^{\\rm vac}_{\\rm LL}"]};
ratio = Simplify[(LL["M^{\\rm F}_{\\rm LL}"]/LL["M^{\\rm W}_{\\rm LL}"])^2 /. massRules]

(* ::Input:: *)
gapFromMass = Solve[LL["M^{\\rm W}_{\\rm LL}"]^2 == 2 LL["Z^{\\rm s}_{\\rm LL}"] LL["g^{\\rm v}_{\\rm LL}"]^2 LL["\\Delta^{\\rm vac}_{\\rm LL}"]^2, LL["\\Delta^{\\rm vac}_{\\rm LL}"]]

(* ::Input:: *)
SL31Check["S-9", "(M^F_LL / M^W_LL)^2 = 4 (g^F_LL / g^v_LL)^2 and the gap follows from M^W_LL (algebra with symbolized symbols)",
  Simplify[ratio - 4 LL["g^{\\rm F}_{\\rm LL}"]^2/LL["g^{\\rm v}_{\\rm LL}"]^2] === 0 && Length[gapFromMass] == 2, ratio];

(* ::Text:: *)
(*S-10, the longitudinal vector boson. For a plane wave of wave number $k^{\rm B}_{\rm LL}$ and frequency $\omega^{\rm B}_{\rm LL}$ the time component and the longitudinal component of a massive vector field obey two linear equations; their determinant is $\big((k^{\rm B}_{\rm LL})^2+(Z^{\rm t}_{\rm LL}/Z^{\rm s}_{\rm LL})M^2\big)\big((\omega^{\rm B}_{\rm LL})^2-M^2\big)-(k^{\rm B}_{\rm LL}\omega^{\rm B}_{\rm LL})^2$ with $M=M^{\rm W}_{\rm LL}$. The cell checks with `Factor` and `Solve` that it vanishes exactly for $(\omega^{\rm B}_{\rm LL})^2=(M^{\rm W}_{\rm LL})^2+(Z^{\rm s}_{\rm LL}/Z^{\rm t}_{\rm LL})(k^{\rm B}_{\rm LL})^2$: the absorbed Goldstone boson moves with the velocity $(Z^{\rm s}_{\rm LL}/Z^{\rm t}_{\rm LL})^{1/2}$.*)

(* ::Input:: *)
det = (LL["k^{\\rm B}_{\\rm LL}"]^2 + LL["Z^{\\rm t}_{\\rm LL}"]/LL["Z^{\\rm s}_{\\rm LL}"] LL["M^{\\rm W}_{\\rm LL}"]^2) (LL["\\omega^{\\rm B}_{\\rm LL}"]^2 - LL["M^{\\rm W}_{\\rm LL}"]^2) -
   (LL["k^{\\rm B}_{\\rm LL}"] LL["\\omega^{\\rm B}_{\\rm LL}"])^2;
(* Solve needs a symbol as unknown: the square of the frequency is called w *)
root = Solve[(det /. LL["\\omega^{\\rm B}_{\\rm LL}"] -> Sqrt[w]) == 0, w]

(* ::Input:: *)
SL31Check["S-10", "the determinant of the time and longitudinal equations vanishes exactly at omega_B^2 = M_W^2 + (Z_s/Z_t) k_B^2",
  Length[root] == 1 && Simplify[(w /. First[root]) - (LL["M^{\\rm W}_{\\rm LL}"]^2 + LL["Z^{\\rm s}_{\\rm LL}"]/LL["Z^{\\rm t}_{\\rm LL}"] LL["k^{\\rm B}_{\\rm LL}"]^2)] === 0,
  Factor[det]];

(* ::Text:: *)
(*S-11, the sum over the isospin index with the generators $T^\iota_{\rm LL}=\tau^{\rm LL}_\iota/2$: the Casimir $(T_{\rm LL})^2=\sum_\iota T^\iota_{\rm LL}T^\iota_{\rm LL}$ of the isospin one-half is $\tfrac34$ times the unit matrix. The components $\tau^{\rm LL}_1,\tau^{\rm LL}_2,\tau^{\rm LL}_3$ and $T^1_{\rm LL},T^2_{\rm LL},T^3_{\rm LL}$ are symbolized; the generators are defined from the Pauli matrices by rules.*)

(* ::Input:: *)
taus = Table[ReleaseHold[SL31Instance[SL31SymbolBoxes["\\tau^{\\rm LL}_\\iota"], {"\[Iota]" -> ToString[k]}]], {k, 3}];
Ts = Table[ReleaseHold[SL31Instance[SL31SymbolBoxes["T^\\iota_{\\rm LL}"], {"\[Iota]" -> ToString[k]}]], {k, 3}];
casimir = Sum[Ts[[k]] . Ts[[k]], {k, 3}] /. Thread[Ts -> taus/2] /. Thread[taus -> pauliValues];
SL31Check["S-11", "the sum over iota of T^iota_LL T^iota_LL (with T = tau/2 for the symbolized components) is 3/4 times the unit matrix",
  casimir === 3/4 IdentityMatrix[2], casimir];

(* ::Text:: *)
(*S-12, uniqueness of the plain-text names: no plain-text name of the registry belongs to two different entries (the programs and data files can therefore always be read back without ambiguity), and every name maps to one entry.*)

(* ::Input:: *)
nameToEntries = GroupBy[Flatten[Map[Function[e, Map[# -> e["entry"] &, e["names"]]], SL31Registry]], First -> Last];
shared = Select[nameToEntries, Length[DeleteDuplicates[#]] > 1 &];
SL31Check["S-12", "no plain-text name is registered for two different entries",
  shared === <||>, {Length[nameToEntries], Normal[shared]}];

(* ::Text:: *)
(*S-13, the kinds that are not symbolized. The imaginary unit $i$ is the Wolfram symbol `I`; the number sets $\mathbb R$ and $\mathbb C$ are the Wolfram domains `Reals` and `Complexes` (which the Wolfram Language prints as double-struck letters); the operators correspond to Wolfram functions: tr is `Tr`, det is `Det`, diag is `DiagonalMatrix`, Re and Im are `Re` and `Im`, the tensor product $\otimes$ is `KroneckerProduct`, the transpose ${}^T$ is `Transpose`. The test checks each correspondence on the Pauli matrices.*)

(* ::Input:: *)
p1 = pauliValues[[1]]; p2 = pauliValues[[2]]; p3 = pauliValues[[3]];
correspondences = {I^2 == -1, Element[Sqrt[2], Reals], Element[I, Complexes], ! Element[I, Reals],
   Tr[p3] == 0, Det[p1] == -1, DiagonalMatrix[{1, -1}] == p3, Re[p2] == {{0, 0}, {0, 0}}, Im[p2] == {{0, -1}, {1, 0}},
   KroneckerProduct[p1, p3] . KroneckerProduct[p1, p3] == IdentityMatrix[4], Transpose[p2] == -p2};
SL31Check["S-13", "the imaginary unit, the number sets and the operators correspond to I, Reals, Complexes, Tr, Det, DiagonalMatrix, Re, Im, KroneckerProduct, Transpose",
  And @@ correspondences, correspondences];

(* ::Text:: *)
(*S-15, the reader of formulas (section I.3). (a) Eighteen formulas whose reading is known are converted and their readings on one line are compared with the known readings; among them are the cases that the TeX importer of the Wolfram Language reads wrongly (a fraction with arguments without braces, and `\hbar`). (b) Every formula in the meanings of the registry is converted: the reader must report no problem, the number of fraction boxes must equal the number of the commands `\frac` and `\tfrac` in the TeX, the number of root boxes must equal the number of the commands `\sqrt`, and no fraction box may have an empty numerator or denominator. (c) A name is not a product: the boxes of `x5` must be the letter, the character `\[InvisibleSpace]` and the number, and the boxes of `5x` the number and the letter with nothing between them (a product, between whose factors the front end puts its thin space); the last number of the result counts the names of this kind in the meanings of the registry (the three coordinates `x5`, `x6`, `x7`).*)

(* ::Input:: *)
cases15 = {
   {"\\tfrac12\\ell^{\\rm x}_{\\rm LL}g^{\\rm v}_{\\rm LL}", "frac{1}{2}\[ScriptL]_{LL}^{x}g_{LL}^{v}"},
   {"Q_{\\rm LL}=-\\tfrac12", "Q_{LL}=-frac{1}{2}"},
   {"\\cos(\\tfrac12\\vartheta^{\\rm B}_{\\rm LL})", "cos(frac{1}{2}\[CurlyTheta]_{LL}^{B})"},
   {"-\\tfrac{15}8\\int_{q_{\\rm LL}<\\Lambda_{\\rm LL}}d^3q_{\\rm LL}", "-frac{15}{8}\[Integral]_{q_{LL}<\[CapitalLambda]_{LL}}d^{3}q_{LL}"},
   {"\\tfrac i2(y^+_{\\rm LL}-y^-_{\\rm LL})", "frac{i}{2}(y_{LL}^{+}-y_{LL}^{-})"},
   {"\\frac{r^{\\rm W}_{\\rm LL}}{2\\ell^{\\rm x}_{\\rm LL}}\\beta_{\\rm LL}", "frac{r_{LL}^{W}}{2\[ScriptL]_{LL}^{x}}\[Beta]_{LL}"},
   {"\\sqrt2\\,\\ell^{\\rm x}_{\\rm LL}", "sqrt{2}\[ScriptL]_{LL}^{x}"},
   {"\\sqrt{2n^{\\rm cpx}_{\\rm LL}}", "sqrt{2n_{LL}^{cpx}}"},
   {"5\\times10^{-11}", "5\[Times]10^{-11}"},
   {"(q_{\\rm LL}^2+(M^{\\rm q}_{\\rm LL})^2)^{-7/2}", "(q_{LL}^{2}+(M_{LL}^{q})^{2})^{-7/2}"},
   {"\\mathbf e_p", "<bf:e>_{p}"},
   {"\\mathsf K^3_{\\rm LL}", "<sf:K>_{LL}^{3}"},
   {"\\bar\\Psi_{\\rm LL}", "over{\[CapitalPsi]}{_}_{LL}"},
   {"\\lvert\\vec d_{\\rm LL}\\rvert^2", "|over{d}{\[RightVector]}_{LL}|^{2}"},
   {"\\sum_{p=0}^3\\xi^{\\rm LL}_p(\\mathbf b)\\mathbf e_p", "\[Sum]_{p=0}^{3}\[Xi]_{p}^{LL}(<bf:b>)<bf:e>_{p}"},
   {"\\begin{pmatrix}0&-i\\\\i&0\\end{pmatrix}", "(grid[0&-i;i&0])"},
   {"\\hbar", "\[HBar]"},
   {"x5,x6,x7", "x5,x6,x7"}};
readings15 = Map[SL31FormulaReading[SL31TeXFormula[First[#]]] &, cases15];
(* every formula of the meanings of the registry *)
meaningFormulas = DeleteDuplicates[Flatten[Map[SL31TextFormulas[StringReplace[#["meaning"], "\\_" -> "_"]] &, SL31Registry]]];
problems15 = Flatten[Map[SL31TeXFormulaProblems, meaningFormulas]];
boxes15 = Map[SL31TeXFormula, meaningFormulas];
fractions15 = Cases[boxes15, _FractionBox, Infinity];
counts15 = {Total[StringCount[meaningFormulas, RegularExpression["\\\\[td]?frac(?![A-Za-z])"]]], Length[fractions15],
   Total[StringCount[meaningFormulas, RegularExpression["\\\\sqrt(?![A-Za-z])"]]], Count[boxes15, _SqrtBox | _RadicalBox, Infinity]};
(* a name and a product: the boxes of x5 and of 5x *)
names15 = SL31TeXFormula["x5"] === RowBox[{StyleBox["x", "TI"], "\[InvisibleSpace]", "5"}] &&
   SL31TeXFormula["5x"] === RowBox[{"5", StyleBox["x", "TI"]}];
SL31Check["S-15", "the reader of formulas reads 18 formulas of known reading correctly, converts every formula of the meanings of the registry without a problem, with one fraction box for every fraction command and one root box for every root command, and sets a number that directly follows a letter as part of a name (x5), not as a factor",
  readings15 === cases15[[All, 2]] && problems15 === {} && counts15[[1]] == counts15[[2]] && counts15[[3]] == counts15[[4]] &&
   FreeQ[fractions15, FractionBox["", _] | FractionBox[_, ""]] && names15,
  {Length[cases15], Count[MapThread[SameQ, {readings15, cases15[[All, 2]]}], True], Length[meaningFormulas], Length[problems15], counts15,
   Count[boxes15, "\[InvisibleSpace]", Infinity]}];

(* ::Section:: *)
(*I.8 The complete table of the symbols*)

(* ::Text:: *)
(*The table below shows every entry of the registry, group by group: the printed symbols (as the notebook now displays them), the plain-text names and the meaning. In the meanings the formulas are shown in their printed form, converted by the reader of formulas of section I.3, and the names of programs and of functions are set in the code typeface.*)

(* ::Input:: *)
(* the meaning as an inline text cell: the formulas between dollar signs are converted by the
   reader of formulas (section I.3) and the code between backquotes is set in the code typeface;
   an underscore written as a backslash and an underscore in the registry is an underscore *)
textWithMath[t_String] := RawBoxes[SL31TextBoxes[StringReplace[t, "\\_" -> "_"], FontSize -> 9.5, FontFamily -> "Source Sans Pro"]];
(* one row of the table: the printed symbols (a Pane of fixed width, in which the list of symbols
   may break after a comma), the plain-text names, and the meaning in the text font; the three
   widths (125 + 95 + 290 printer's points) let the table fit the width of a printed page *)
symbolRow[e_] := {Pane[If[StringContainsQ[e["symbol"], "$"],
      Row[Riffle[Map[RawBoxes[SL31TeXToBoxes[#]] &,
        Flatten[Map[SL31SplitTeX, StringCases[e["symbol"], "$" ~~ Shortest[x__] ~~ "$" :> x]]]], ", "]],
      Style[e["symbol"], 9]], 125],
   Pane[Style[StringRiffle[e["names"], " "], FontFamily -> "Source Code Pro", 8], 95],
   Pane[textWithMath[e["meaning"]], 290]};
groupTable[g_] := Grid[Prepend[Map[symbolRow, Select[SL31Registry, #["group"] === g &]],
    Style[#, Bold, FontFamily -> "Source Sans Pro"] & /@ {"symbol", "plain-text names", "meaning"}],
   Frame -> All, Alignment -> {Left, Top}, Background -> {None, {LightGray}}, Spacings -> {1, 0.6}];
groups = DeleteDuplicates[Map[#["group"] &, SL31Registry]];
Length[groups]

(* ::Input:: *)
Column[Map[Column[{Style[#, Bold, 12], groupTable[#]}] &, groups[[1 ;; 3]]], Spacings -> 2]

(* ::Input:: *)
Column[Map[Column[{Style[#, Bold, 12], groupTable[#]}] &, groups[[4 ;; 8]]], Spacings -> 2]

(* ::Input:: *)
Column[{Style[groups[[9]], Bold, 12], groupTable[groups[[9]]]}]

(* ::Section:: *)
(*I.9 Summary of the tests of Part I*)

(* ::Input:: *)
Grid[Prepend[Map[{#["name"], If[#["pass"], "PASS", "FAIL"], #["statement"]} &, SL31`Test`$Results], Style[#, Bold] & /@ {"test", "result", "statement"}],
  Frame -> All, Alignment -> Left]

(* ::Input:: *)
{"tests" -> Length[SL31`Test`$Results], "passed" -> Count[SL31`Test`$Results, r_ /; r["pass"]]}

(* @@INCLUDECELLS part2_A_core.source.wl@@ *)
(* @@INCLUDECELLS part2_B_item1.source.wl@@ *)
(* @@INCLUDECELLS part2_C_item1b.source.wl@@ *)
