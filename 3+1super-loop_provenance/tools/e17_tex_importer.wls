(* E17: what the Wolfram TeX importer makes of styled letters and of \varepsilon, and what the builder makes of them *)
tests = {"\\mathbf b", "\\mathbf{b}", "\\mathsf N", "\\mathbb R", "\\mathcal L", "\\mathfrak g", "\\varepsilon", "\\epsilon",
   "\\mathbf b=\\sum_{p=0}^3\\xi^{\\rm LL}_p(\\mathbf b)\\mathbf e_p", "\\varepsilon^{\\rm lat}_{\\rm LL}", "\\mathfrak{so}(3,1)", "\\mathsf 1_2"};
imp[t_] := Cases[Quiet[ImportString["$" <> t <> "$", "TeX"]], Cell[BoxData[f_FormBox], "InlineFormula", ___] :> f, Infinity];
Print["--- the TeX importer alone ---"];
Do[Print[t, "  ==>  ", ToString[imp[t], InputForm, CharacterEncoding -> "ASCII"]], {t, tests}];
(* the built notebook (path given as the first argument): the boxes of the formulas in its text cells *)
nbFile = Last[$ScriptCommandLine];
nb = Get[nbFile];
textCells = Cases[nb, Cell[_, "Text" | "Section" | "Subsection" | "Chapter" | "Title" | "Item", ___], Infinity];
inl = Cases[textCells, Cell[BoxData[f_], "InlineFormula", ___] :> f, Infinity];
Print["--- the built notebook ", nbFile, " ---"];
Print["text cells: ", Length[textCells], ", inline formulas: ", Length[inl]];
Print["bold upright letters (FontWeight -> Bold): ", Count[inl, StyleBox[_, FontWeight -> "Bold", FontSlant -> "Plain"], Infinity]];
Print["sans-serif letters (FontFamily -> Helvetica): ", Count[inl, StyleBox[_, FontFamily -> "Helvetica", FontSlant -> "Plain"], Infinity]];
Print["double-struck, script and Gothic characters: ",
  Count[inl, s_String /; StringContainsQ[ToString[s, InputForm, CharacterEncoding -> "ASCII"], "DoubleStruck" | "Script" | "Gothic"], Infinity]];
Print["invalid FontSlant -> Bold left by the importer: ", Count[inl, StyleBox[_, FontSlant -> "Bold"], Infinity]];
Print["placeholders left: ", Count[inl, s_String /; StringContainsQ[s, "QZ"], Infinity]];
Print["curly epsilon (from \\varepsilon): ", Count[inl, s_String /; StringContainsQ[s, "\[CurlyEpsilon]"], Infinity],
  ", lunate epsilon (from \\epsilon): ", Count[inl, s_String /; StringContainsQ[s, "\[Epsilon]"], Infinity]];
