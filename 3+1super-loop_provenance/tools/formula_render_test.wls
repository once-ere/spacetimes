#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   formula_render_test.wls     (PROJECT spacetimes, notebook 3+1super-loop.nb)

   PURPOSE
   Makes a small notebook that shows how formulas are DISPLAYED, and prints it to a PDF file, so
   that the display of the notebook's own reader of formulas (notebook_source/sl31_formula.wl)
   can be looked at before the whole notebook is built:
     1. a text cell with test formulas, converted by the own reader, and two display formulas;
     2. the same text converted by the TeX importer of the Wolfram Language with the alphabets
        protected (the way of the builder before the own reader existed), for comparison;
     3. rows of the table of the symbols (section I.8 of the notebook) in two layouts:
        (a) the meaning as a row of strings and formula boxes, (b) the meaning as an inline
        text cell with inline formula cells.

   USAGE
     wolframscript -file formula_render_test.wls <sl31_core.wl> <sl31_formula.wl> <registry_data.wl> <out.pdf>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
{core, formula, registry, outPDF} = args[[1 ;; 4]];
UsingFrontEnd[Get[core]];
Get[formula];
Get[registry];

sample = "Fractions and roots: $\\tfrac12\\ell^{\\rm x}_{\\rm LL}g^{\\rm v}_{\\rm LL}$, $\\cos(\\tfrac12a)$, " <>
   "$-\\tfrac{15}8\\int_{q_{\\rm LL}<\\Lambda_{\\rm LL}}d^3q_{\\rm LL}(2\\pi)^{-3}$, $\\frac{a}{b}c$, $\\sqrt2\\,x$, " <>
   "$a^{\\rm tol}_{\\rm LL}\\sqrt{2n^{\\rm cpx}_{\\rm LL}}$, $\\tfrac i2(y^+_{\\rm LL}-y^-_{\\rm LL})$. " <>
   "Alphabets: $\\mathbf e_j$, $\\mathbb R^{1,3}_{\\rm LL}$, $\\mathsf K^3_{\\rm LL}$, $\\mathcal H^{\\rm lat}_{\\rm LL}$, " <>
   "$\\mathfrak{so}(3,1)$, $\\Gamma_{\\rm E}$, $x^j_{\\rm LL}$, $\\mathsf 1_8$, $\\varepsilon^{\\rm lat}_{\\rm LL}$, $\\epsilon^{\\rm LL}_{jj'j''}$. " <>
   "Operators: $\\cos^2\\vartheta^{\\rm B}_{\\rm LL}=\\tfrac12\\big(1+(\\pm\\varepsilon_{\\rm LL}-\\mu_{\\rm LL})/E^\\mp_{\\rm LL}\\big)$, " <>
   "$\\mathbf b=\\sum_{p=0}^3\\xi^{\\rm LL}_p(\\mathbf b)\\mathbf e_p$, $\\lvert\\vec d_{\\rm LL}\\rvert^2$, " <>
   "$\\lVert w^{\\rm lat}_{\\rm LL}\\rVert$, $2\\times2$, $5\\times10^{-11}$, $\\hbar$, " <>
   "$\\Psi_{\\rm LL}\\to e^{i\\theta_{\\rm LL}}\\Psi_{\\rm LL}$, $\\overline{\\mathbf x\\mathbf y}=\\bar{\\mathbf y}\\,\\bar{\\mathbf x}$, " <>
   "$\\mathbb L_{\\rm LL}=\\{\\mathbf w\\in\\mathbb B_{\\rm LL}:N_{\\rm LL}(\\mathbf w)=1\\}$, $[0,1)$, $i\\sin x$, $2\\cos x$, " <>
   "$\\Omega_{\\rm LL}=\\lvert\\vec d_{\\rm LL}\\rvert^2/G_{\\rm LL}-\\tfrac12\\int\\!d^3q_{\\rm LL}(2\\pi)^{-3}\\sum(\\text{positive eigenvalues of }\\mathsf H_{\\rm LL})$. " <>
   "A matrix: $\\sigma^{\\rm LL}_2=\\begin{pmatrix}0&-i\\\\i&0\\end{pmatrix}$ and " <>
   "$\\mathsf H_{\\rm LL}=\\begin{pmatrix}\\mathsf h_{\\rm LL}(\\vec q_{\\rm LL})-\\mu_{\\rm LL}&\\mathsf P_{\\rm LL}\\\\\\mathsf P_{\\rm LL}^\\dagger&-(\\mathsf h_{\\rm LL}(-\\vec q_{\\rm LL})-\\mu_{\\rm LL})^T\\end{pmatrix}$. " <>
   "The bound: $B^{\\rm out}_{\\rm LL}=\\big(1+\\tfrac14h^{\\rm IDA}_{\\rm LL}E^{\\rm max}_{\\rm LL}\\big)B^{\\rm err}_{\\rm LL}+\\tfrac14h^{\\rm IDA}_{\\rm LL}r^{\\rm der}_{\\rm LL}+\\frac{(h^{\\rm IDA}_{\\rm LL}E^{\\rm max}_{\\rm LL})^4}{384}$. " <>
   "The link matrix: $e^{-i\\ell^{\\rm x}_{\\rm LL}g^{\\rm F}_{\\rm LL}A^{\\rm F}_{{\\rm LL},j}}\\big(\\cos(\\tfrac12\\ell^{\\rm x}_{\\rm LL}g^{\\rm v}_{\\rm LL}\\lvert\\vec A^{\\rm v}_{{\\rm LL},j}\\rvert)\\sigma^{\\rm LL}_0-i\\sin(\\tfrac12\\ell^{\\rm x}_{\\rm LL}g^{\\rm v}_{\\rm LL}\\lvert\\vec A^{\\rm v}_{{\\rm LL},j}\\rvert)\\sum_\\iota A^\\iota_{{\\rm LL},j}\\tau^{\\rm LL}_\\iota/\\lvert\\vec A^{\\rm v}_{{\\rm LL},j}\\rvert\\big)$. " <>
   "Groups: $SL(2,\\mathbb C)\\times SU(2)_{\\rm v}\\times U(1)^{\\rm F}_{\\rm LL}$, $SO(3)$. " <>
   "Code `t2link` and **bold** text.";
(* two display formulas, as the builder makes them from cells of the style DisplayFormula *)
displays = {"B^{\\rm out}_{\\rm LL}=\\big(1+\\tfrac14h^{\\rm IDA}_{\\rm LL}E^{\\rm max}_{\\rm LL}\\big)B^{\\rm err}_{\\rm LL}+\\tfrac14h^{\\rm IDA}_{\\rm LL}r^{\\rm der}_{\\rm LL}+\\frac{(h^{\\rm IDA}_{\\rm LL}E^{\\rm max}_{\\rm LL})^4}{384}",
   "\\sum_j\\Big[-\\frac{i\\alpha^j_{\\rm LL}}{2\\ell^{\\rm x}_{\\rm LL}}\\big(\\mathsf U^{\\rm link}_{{\\rm LL},j}(\\vec s_{\\rm LL})\\psi^{\\rm p}_{\\rm LL}(\\vec s_{\\rm LL}+\\vec\\delta^j_{\\rm LL})-\\mathsf U^{\\rm link}_{{\\rm LL},j}(\\vec s_{\\rm LL}-\\vec\\delta^j_{\\rm LL})^\\dagger\\psi^{\\rm p}_{\\rm LL}(\\vec s_{\\rm LL}-\\vec\\delta^j_{\\rm LL})\\big)-\\frac{r^{\\rm W}_{\\rm LL}}{2\\ell^{\\rm x}_{\\rm LL}}\\beta_{\\rm LL}\\big(\\mathsf U^{\\rm link}_{{\\rm LL},j}(\\vec s_{\\rm LL})\\psi^{\\rm p}_{\\rm LL}(\\vec s_{\\rm LL}+\\vec\\delta^j_{\\rm LL})+\\mathsf U^{\\rm link}_{{\\rm LL},j}(\\vec s_{\\rm LL}-\\vec\\delta^j_{\\rm LL})^\\dagger\\psi^{\\rm p}_{\\rm LL}(\\vec s_{\\rm LL}-\\vec\\delta^j_{\\rm LL})-2\\psi^{\\rm p}_{\\rm LL}(\\vec s_{\\rm LL})\\big)\\Big]+m_{\\rm LL}\\beta_{\\rm LL}\\psi^{\\rm p}_{\\rm LL}(\\vec s_{\\rm LL})-\\big(\\mathsf A_{{\\rm LL},0}(\\vec s_{\\rm LL})+\\mu_{\\rm LL}\\big)\\psi^{\\rm p}_{\\rm LL}(\\vec s_{\\rm LL})+\\mathsf P_{\\rm LL}(\\vec s_{\\rm LL})\\psi^{\\rm h}_{\\rm LL}(\\vec s_{\\rm LL}),"};
displayCell[t_String] := Cell[BoxData[FormBox[SL31TeXFormula[t], TraditionalForm]], "DisplayFormula", FontFamily -> "Times",
   FontSize -> 1.25 Inherited];

(* ---- the way of the old builder (the importer with the alphabets protected) ---- *)
alphaCode = <|"mathbf" -> "bf", "mathsf" -> "sf", "mathbb" -> "bb", "mathcal" -> "cal", "mathfrak" -> "frak"|>;
protectAlphabets[tex_String] := StringReplace[StringReplace[tex,
    RegularExpression["\\\\(mathbf|mathsf|mathbb|mathcal|mathfrak)\\s*(?:\\{([A-Za-z0-9]+)\\}|([A-Za-z0-9]))"] :>
      "\\mathrm{QZ" <> alphaCode["$1"] <> "Q" <> "$2" <> "$3" <> "ZQ}"],
   RegularExpression["\\\\varepsilon(?![A-Za-z])"] -> "\\mathrm{QZgrQvarepsilonZQ}"];
special[kind_String, ch_String] := Which[
   DigitQ[ch], ch,
   UpperCaseQ[ch], ToExpression["\"\\[" <> kind <> "Capital" <> ch <> "]\""],
   True, ToExpression["\"\\[" <> kind <> ToUpperCase[ch] <> "]\""]];
alphaBox["bf", x_] := StyleBox[x, FontWeight -> "Bold", FontSlant -> "Plain"];
alphaBox["sf", x_] := StyleBox[x, FontFamily -> "Helvetica", FontSlant -> "Plain"];
alphaBox["bb", x_] := StyleBox[StringJoin[special["DoubleStruck", #] & /@ Characters[x]], FontSlant -> "Plain"];
alphaBox["cal", x_] := StyleBox[StringJoin[special["Script", #] & /@ Characters[x]], FontSlant -> "Plain"];
alphaBox["frak", x_] := StyleBox[StringJoin[special["Gothic", #] & /@ Characters[x]], FontSlant -> "Plain"];
alphaBox["gr", "varepsilon"] := "\[CurlyEpsilon]";
placeholderRE = RegularExpression["QZ(bf|sf|bb|cal|frak|gr)Q([A-Za-z0-9]+)ZQ"];
placeholderBox[s_String] := alphaBox @@ First[StringCases[s, placeholderRE :> {"$1", "$2"}]];
restoreAlphabets[boxes_] := boxes /. {
    StyleBox[s_String /; StringMatchQ[s, placeholderRE], ___] :> placeholderBox[s],
    s_String /; StringMatchQ[s, placeholderRE] :> placeholderBox[s]};
oldInline[tex_String] := Module[{nb = Quiet[ImportString["$" <> protectAlphabets[tex] <> "$", "TeX"]], c},
   c = Cases[nb, Cell[BoxData[f_FormBox], "InlineFormula", ___], Infinity];
   If[c === {}, "$" <> tex <> "$", Cell[BoxData[restoreAlphabets[First[c]]], "InlineFormula"]]];
oldTextData[t_String] := Module[{r},
   r = StringReplace[t, {
      RegularExpression["\\$([^$]+)\\$"] :> oldInline["$1"],
      RegularExpression["`([^`]+)`"] :> StyleBox["$1", FontFamily -> "Source Code Pro"],
      RegularExpression["\\*\\*([^*]+)\\*\\*"] :> StyleBox["$1", FontWeight -> "Bold"]}];
   TextData[If[Head[r] === StringExpression, List @@ r, {r}]]];

(* ---- the two layouts of a meaning in the table of the symbols ---- *)
clean[t_String] := StringReplace[t, "\\_" -> "_"];
(* (a) a row of strings and boxes *)
rowForm[t_String] := Row[Replace[First[SL31TextData[clean[t]]], {
      Cell[BoxData[b_], ___] :> RawBoxes[b], b_StyleBox :> RawBoxes[b]}, {1}]];
(* (b) an inline text cell *)
cellForm[t_String] := RawBoxes[SL31TextBoxes[clean[t], FontSize -> 9.5, FontFamily -> "Source Sans Pro"]];
symbolsOf[e_] := Row[Riffle[Map[RawBoxes[SL31TeXToBoxes[#]] &,
     Flatten[Map[SL31SplitTeX, StringCases[e["symbol"], "$" ~~ Shortest[x__] ~~ "$" :> x]]]], ", "]];
tableOf[layout_, entries_List] := Grid[Prepend[
    Map[Function[e, {Pane[If[StringContainsQ[e["symbol"], "$"], symbolsOf[e], Style[e["symbol"], 9]], 125],
        Pane[Style[StringRiffle[e["names"], " "], FontFamily -> "Source Code Pro", 8], 95],
        Pane[If[layout === "row", Style[rowForm[e["meaning"]], 9.5, FontFamily -> "Source Sans Pro"], cellForm[e["meaning"]]], 290]}],
     entries],
    Style[#, Bold, FontFamily -> "Source Sans Pro"] & /@ {"symbol", "plain-text names", "meaning"}],
   Frame -> All, Alignment -> {Left, Top}, Background -> {None, {LightGray}}, Spacings -> {1, 0.6}];
pick = Join[Take[SL31Registry, 4],
   Select[SL31Registry, StringContainsQ[#["meaning"], "pmatrix"] &, 1],
   Select[SL31Registry, StringContainsQ[#["meaning"], "\\tfrac{15}8"] &, 1],
   Select[SL31Registry, StringContainsQ[#["meaning"], "budget_out"] || StringContainsQ[#["meaning"], "B^{\\rm out}_{\\rm LL}="] &, 1]];
Print["entries shown: ", Map[#["entry"] &, pick]];

nb = UsingFrontEnd[Notebook[{
     Cell["1. The own reader of formulas", "Section"],
     Cell[SL31TextData[sample], "Text"],
     displayCell[displays[[1]]],
     displayCell[displays[[2]]],
     Cell["2. The TeX importer with the alphabets protected (the old way)", "Section"],
     Cell[oldTextData[sample], "Text"],
     Cell["3a. Rows of the table of the symbols: the meaning as a row of strings and boxes", "Section"],
     Cell[BoxData[ToBoxes[tableOf["row", pick], StandardForm]], "Output"],
     Cell["3b. Rows of the table of the symbols: the meaning as an inline text cell", "Section"],
     Cell[BoxData[ToBoxes[tableOf["cell", pick], StandardForm]], "Output"]},
    StyleDefinitions -> "Default.nb", WindowSize -> {1000, 900}]];
problems = Flatten[Map[SL31TeXFormulaProblems, StringCases[sample, RegularExpression["\\$([^$]+)\\$"] :> "$1"]]];
Print["problems reported for the sample: ", problems];
res = UsingFrontEnd[Export[outPDF, nb, "PDF"]];
Print["export: ", res];
If[! FileExistsQ[outPDF],
  Print["PDF export failed; trying NotebookPut and NotebookPrint"];
  UsingFrontEnd[Module[{nbo = NotebookPut[nb]}, NotebookPrint[nbo, outPDF]; NotebookClose[nbo]]]];
Print["file exists: ", FileExistsQ[outPDF], If[FileExistsQ[outPDF], " (" <> ToString[FileByteCount[outPDF]] <> " bytes)", ""]];
Exit[0];
