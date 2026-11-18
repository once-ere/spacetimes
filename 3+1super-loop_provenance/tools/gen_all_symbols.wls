#!/usr/bin/env wolframscript
(* gen_all_symbols.wls -- writes source/all_symbols.wl: one assignment  typed = {LL["tex"], ...};
   listing every symbolized TeX item of the registry in the order of the table symbolTable of the
   notebook.  The builder replaces every LL["tex"] by the two-dimensional box of the symbol, so that
   the notebook cell shows all symbols in their printed form. *)
dir = "/Users/nsh/Developer/github/spacetimes/.git-transfer/mma";
Get[FileNameJoin[{dir, "source", "sl31_core.wl"}]];
Get[FileNameJoin[{dir, "source", "registry_data.wl"}]];
segments = Flatten[Map[Function[e, Map[{e["entry"], #} &, StringCases[e["symbol"], "$" ~~ Shortest[x__] ~~ "$" :> x]]], SL31Registry], 1];
items = Flatten[Map[Function[s, Map[<|"entry" -> s[[1]], "tex" -> #, "boxes" -> SL31TeXToBoxes[#]|> &, SL31SplitTeX[s[[2]]]]], segments], 1];
items = Map[Append[#, "kind" -> SL31Classify[#["tex"], #["boxes"]]] &, items];
toSymbolize = DeleteDuplicatesBy[Select[items, MemberQ[SL31Symbolized, #["kind"]] &], #["tex"] &];
(* items with the same boxes are one symbol (as in section I.6 of the notebook): the first TeX item of each group *)
byBoxes = GatherBy[toSymbolize, SL31SymbolBoxes[#["tex"]] &];
refs = Map["LL[" <> ToString[First[#]["tex"], InputForm] <> "]" &, byBoxes];
lines = Map[StringRiffle[#, ", "] &, Partition[refs, UpTo[6]]];
code = "(* every symbolized symbol of the registry (" <> ToString[Length[refs]] <> "), typed in its printed form *)\n" <>
   "typed = {\n  " <> StringRiffle[lines, ",\n  "] <> "};";
Export[FileNameJoin[{dir, "source", "all_symbols.wl"}], code, "Text"];
Print[Length[refs]];
