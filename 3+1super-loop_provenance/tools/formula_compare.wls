#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   formula_compare.wls     (PROJECT spacetimes, notebook 3+1super-loop.nb)

   PURPOSE
   Compares, for every formula that the notebook prints (the list written by
   tools/formula_inventory.py), three readings:
     own        the notebook's own reader of formulas, SL31TeXFormula (notebook_source/sl31_formula.wl);
     importer+  the TeX importer of the Wolfram Language, ImportString["$...$", "TeX"], with the
                alphabets of the letters protected and restored, as the notebook builder used it
                for the text cells before the own reader existed (the functions are copied here
                from that version of tools/build_notebook.wls);
     importer   the TeX importer alone, as section I.8 of the notebook used it for the meanings
                of the registry.
   Every reading is written on one line by SL31FormulaReading (a fraction as frac{a}{b}, a root
   as sqrt{a}, ...; spaces, the italic and upright styles and the grouping of rows are left
   out), so that two readings are equal exactly if they show the same formula.  The report lists
   every formula in which the readings differ, and every problem that the own reader reports.

   USAGE
     wolframscript -file formula_compare.wls <sl31_core.wl> <sl31_formula.wl> <formula_inventory.wl> <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
{core, formula, inventory, report} = args[[1 ;; 4]];
UsingFrontEnd[Get[core]];
Get[formula];
Get[inventory];
Print["formulas: ", Length[SL31FormulaInventory]];

(* ---- the importer as the builder used it (copied from the builder before the change) ---- *)
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
importBoxes[tex_String] := Module[{c = Cases[Quiet[ImportString["$" <> tex <> "$", "TeX"]],
      Cell[BoxData[f_FormBox], "InlineFormula", ___] :> f, Infinity]},
   If[c === {}, $Failed, First[c]]];
impRaw[tex_String] := importBoxes[tex];
impProt[tex_String] := With[{b = importBoxes[protectAlphabets[tex]]}, If[b === $Failed, $Failed, restoreAlphabets[b]]];

(* ---- the three readings ----
   The importer writes some items in another way than the own reader, with the same meaning; before
   the comparison these are brought to the form of the own reader: an absolute value as a template
   (the bars are added), the empty item \[Null], the cross for \times, the arrow \[Rule] for \to,
   another small circle for \circ, and the negative thin space. *)
sameMeaning = {"\[Null]" -> "", "\[NegativeThinSpace]" -> "", "\[Cross]" -> "\[Times]", "\[Rule]" -> "\[RightArrow]",
   "\[EmptySmallCircle]" -> "\[SmallCircle]"};
normalize[b_] := b /. {TemplateBox[{x_}, "Abs"] :> RowBox[{"|", x, "|"}], TemplateBox[{x_}, "Norm"] :> RowBox[{"||", x, "||"}]} /.
   s_String :> StringReplace[s, sameMeaning];
reading[$Failed] := "<not converted>";
reading[b_] := SL31FormulaReading[normalize[b]];
show[b_] := ToString[b, InputForm, CharacterEncoding -> "ASCII"];
t0 = AbsoluteTime[];
rows = UsingFrontEnd[Table[
   Module[{kind = item[[1]], tex = item[[2]], parsed, own, prot, raw},
    parsed = SL31`Private`formulaParse[tex];
    own = First[parsed];
    prot = impProt[tex];
    raw = impRaw[tex];
    <|"kind" -> kind, "tex" -> tex, "own" -> own, "problems" -> Last[parsed], "prot" -> prot, "raw" -> raw,
      "rOwn" -> reading[own], "rProt" -> reading[prot], "rRaw" -> reading[raw]|>],
   {item, SL31FormulaInventory}]];
Print["time: ", Round[AbsoluteTime[] - t0], " s"];

diffProt = Select[rows, #["rOwn"] =!= #["rProt"] &];
diffRaw = Select[rows, #["rOwn"] =!= #["rRaw"] &];
withProblems = Select[rows, #["problems"] =!= {} &];
count[rs_, k_] := Count[rs, r_ /; r["kind"] === k];
Print["own reader: formulas with a reported problem: ", Length[withProblems]];
Print["own = importer+ (alphabets protected): ", Length[rows] - Length[diffProt], " of ", Length[rows],
  "; different: ", Length[diffProt], " (text ", count[diffProt, "text"], ", display ", count[diffProt, "display"],
  ", meaning ", count[diffProt, "meaning"], ")"];
Print["own = importer (alone): ", Length[rows] - Length[diffRaw], " of ", Length[rows],
  "; different: ", Length[diffRaw], " (text ", count[diffRaw, "text"], ", display ", count[diffRaw, "display"],
  ", meaning ", count[diffRaw, "meaning"], ")"];
(* the number of fractions and roots: commands in the TeX against boxes of the own reader *)
nFracTeX = Total[Map[StringCount[#["tex"], RegularExpression["\\\\[td]?frac(?![A-Za-z])"]] &, rows]];
nFracOwn = Total[Map[Count[#["own"], _FractionBox, {0, Infinity}] &, rows]];
nFracProt = Total[Map[Count[#["prot"], _FractionBox, {0, Infinity}] &, rows]];
nSqrtTeX = Total[Map[StringCount[#["tex"], RegularExpression["\\\\sqrt(?![A-Za-z])"]] &, rows]];
nSqrtOwn = Total[Map[Count[#["own"], _SqrtBox | _RadicalBox, {0, Infinity}] &, rows]];
Print["fractions: ", nFracTeX, " commands in the TeX, ", nFracOwn, " fraction boxes of the own reader, ", nFracProt, " of the importer"];
Print["roots: ", nSqrtTeX, " commands in the TeX, ", nSqrtOwn, " root boxes of the own reader"];

lines = {"formula_compare.wls: " <> ToString[Length[rows]] <> " formulas",
   "own reader, formulas with a reported problem: " <> ToString[Length[withProblems]],
   "different readings, own against importer+ (alphabets protected): " <> ToString[Length[diffProt]],
   "different readings, own against the importer alone: " <> ToString[Length[diffRaw]],
   "fractions: " <> ToString[nFracTeX] <> " in the TeX, " <> ToString[nFracOwn] <> " own, " <> ToString[nFracProt] <> " importer+",
   "roots: " <> ToString[nSqrtTeX] <> " in the TeX, " <> ToString[nSqrtOwn] <> " own", ""};
AppendTo[lines, "==== A. problems reported by the own reader ===="];
Do[lines = Join[lines, {"[" <> r["kind"] <> "] " <> r["tex"], "   problems: " <> StringRiffle[r["problems"], "; "]}], {r, withProblems}];
AppendTo[lines, ""];
AppendTo[lines, "==== B. formulas read differently by the own reader and by importer+ ===="];
Do[lines = Join[lines, {"[" <> r["kind"] <> "] " <> r["tex"], "   own      : " <> r["rOwn"], "   importer+: " <> r["rProt"]}], {r, diffProt}];
AppendTo[lines, ""];
AppendTo[lines, "==== C. formulas read differently by the own reader and by the importer alone, but equally by importer+ ===="];
Do[lines = Join[lines, {"[" <> r["kind"] <> "] " <> r["tex"], "   own     : " <> r["rOwn"], "   importer: " <> r["rRaw"]}],
  {r, Select[diffRaw, #["rOwn"] === #["rProt"] &]}];
AppendTo[lines, ""];
AppendTo[lines, "==== D. the boxes of the formulas of part B (own, then importer+) ===="];
Do[lines = Join[lines, {"[" <> r["kind"] <> "] " <> r["tex"], "   own      : " <> show[r["own"]], "   importer+: " <> show[r["prot"]]}], {r, diffProt}];
AppendTo[lines, ""];
AppendTo[lines, "==== E. the boxes of sample formulas read equally (own, then importer+) ===="];
sampleQ[tex_] := StringContainsQ[tex, "\\sum" | "\\int" | "\\cos" | "\\exp" | "\\lvert" | "\\lVert" | "\\langle" | "\\overline" | "\\bar" | "\\hat" |
     "\\tilde" | "\\text" | "\\!" | "\\," | "'" | "\\begin" | "\\mapsto" | "\\to" | "\\dots" | "\\{" | "\\in" | "\\otimes" | "\\dagger" | "\\partial" | "/" | "<" | "*"];
samples = Select[Complement[rows, diffProt], sampleQ[#["tex"]] &];
Do[lines = Join[lines, {"[" <> r["kind"] <> "] " <> r["tex"], "   own      : " <> show[r["own"]], "   importer+: " <> show[r["prot"]]}],
  {r, Take[samples, UpTo[80]]}];
Export[report, StringRiffle[lines, "\n"], "Text"];
Print["report: ", report, " (", Length[lines], " lines)"];
Exit[0];
