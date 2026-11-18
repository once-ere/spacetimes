#!/usr/bin/env wolframscript
(* letter_digit_spacing_test.wls   (PROJECT spacetimes, notebook 3+1super-loop.nb, error E28 of the provenance)

   PURPOSE
     The registry of the symbols names three coordinates of the main project x5, x6, x7 (TeX: x5,x6,x7).
     TeX sets a letter and the number after it without a space.  The front end of the Wolfram Language puts
     the space of a product between two neighbouring items of a row of boxes, so that a row of the italic
     letter x and the number 5 is shown as "x 5", which reads like a product.  This script shows, on one
     page, how the front end sets several ways of writing the pair, so that the reader of formulas of the
     notebook can use one that is set without the space.

   METHOD
     A notebook with one text cell per way of writing is made by the kernel and exported to PDF through the
     front end.  Every formula is an inline cell with the same options as the formulas of the notebook
     (style InlineFormula, typeface Times, 1.1 times the size of the text).

   USAGE
     wolframscript -file letter_digit_spacing_test.wls <output.pdf>
*)
out = $ScriptCommandLine[[2]];
x = StyleBox["x", "TI"];                       (* an italic letter, as the reader of formulas writes it *)
formula[boxes_] := Cell[BoxData[FormBox[boxes, TraditionalForm]], "InlineFormula", FontFamily -> "Times",
   FontSize -> 1.1 Inherited];
ways = {
   {"A. a row of the letter and the number (the reader of formulas before the fix)",
     RowBox[{x, "5"}]},
   {"B. the same row inside a style box with the option AutoSpacing -> False",
     StyleBox[RowBox[{x, "5"}], AutoSpacing -> False]},
   {"C. the letter, the character InvisibleSpace, the number",
     RowBox[{x, "\[InvisibleSpace]", "5"}]},
   {"D. the letter, the character NoBreak, the number",
     RowBox[{x, "\[NoBreak]", "5"}]},
   {"E. a row of the number and the letter, a product (for comparison)",
     RowBox[{"5", x}]},
   {"F. way B for the three coordinates, with commas",
     RowBox[{StyleBox[RowBox[{x, "5"}], AutoSpacing -> False], ",", StyleBox[RowBox[{x, "6"}], AutoSpacing -> False], ",",
       StyleBox[RowBox[{x, "7"}], AutoSpacing -> False]}]},
   {"G. way A for the three coordinates, with commas",
     RowBox[{x, "5", ",", x, "6", ",", x, "7"}]},
   {"H. way B inside a larger formula: the product 2 x5 and a power",
     RowBox[{"2", StyleBox[RowBox[{x, "5"}], AutoSpacing -> False], "+",
       SuperscriptBox[RowBox[{"(", StyleBox[RowBox[{x, "6"}], AutoSpacing -> False], ")"}], "2"]}]}};
cells = Join[
   {Cell["A letter followed by a number: how the front end sets it", "Section"]},
   Map[Cell[TextData[{#[[1]] <> ":   the coordinate ", formula[#[[2]]], " of the main project."}], "Text"] &, ways]];
UsingFrontEnd[
  Module[{nb},
   nb = CreateDocument[cells, Visible -> False, StyleDefinitions -> "Default.nb", WindowSize -> {1000, 900}];
   Export[out, nb, "PDF"];
   NotebookClose[nb]]];
Print["ways: ", Length[ways]];
Print["written: ", FileNameTake[out], ", ", FileByteCount[out], " bytes"];
