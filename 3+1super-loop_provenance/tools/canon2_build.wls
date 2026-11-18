#!/usr/bin/env wolframscript
(* canon2_build.wls -- does every symbol box work as an OPERAND in typed input?  For every
   symbolized box b the notebook in/canon2.nb has two cells:  {b, 1}  and  {TagBox[b, "SL31Name",
   SyntaxForm -> "symbol"], 1}.  $PreRead records what the front end sends; canon2_compare.wls
   checks whether the front end kept the list structure {b, 1}.  The script also prints the TeX
   items that give the same box (duplicates), with their entries. *)
dir = "/Users/nsh/Developer/github/spacetimes/.git-transfer/mma";
Get[FileNameJoin[{dir, "source", "sl31_core.wl"}]];
Get[FileNameJoin[{dir, "source", "registry_data.wl"}]];
segments = Flatten[Map[Function[e, Map[{e["entry"], #} &, StringCases[e["symbol"], "$" ~~ Shortest[x__] ~~ "$" :> x]]], SL31Registry], 1];
items = Flatten[Map[Function[s, Map[<|"entry" -> s[[1]], "tex" -> #, "boxes" -> SL31TeXToBoxes[#]|> &, SL31SplitTeX[s[[2]]]]], segments], 1];
items = Map[Append[#, "kind" -> SL31Classify[#["tex"], #["boxes"]]] &, items];
toSymbolize = DeleteDuplicatesBy[Select[items, MemberQ[SL31Symbolized, #["kind"]] &], #["tex"] &];
Print["symbolized TeX items: ", Length[toSymbolize]];
Print["kinds: ", ToString[ReverseSort[Counts[Map[#["kind"] &, items]]], InputForm]];
mine = Map[SL31SymbolBoxes[#["tex"]] &, toSymbolize];
groups = Select[GatherBy[Transpose[{toSymbolize, mine}], Last], Length[#] > 1 &];
Print["boxes made by more than one TeX item: ", Length[groups]];
meaning = Association[Map[#["entry"] -> #["meaning"] &, SL31Registry]];
Scan[Function[g, Print["  ", ToString[Last[First[g]], InputForm]];
   Scan[Print["     entry ", #[[1]]["entry"], "  ", #[[1]]["tex"], "  ", StringTake[meaning[#[[1]]["entry"]], UpTo[150]]] &, g]], groups];
Put[mine, FileNameJoin[{dir, "out", "canon2_mine.wl"}]];
Put[Map[#["tex"] &, toSymbolize], FileNameJoin[{dir, "out", "canon2_tex.wl"}]];
recv = FileNameJoin[{dir, "out", "canon2_received.wl"}];
cells = Join[
  {Cell[BoxData[MakeBoxes[SL31Canon`received = {}; $PreRead = Function[b, AppendTo[SL31Canon`received, b]; b], StandardForm]], "Input"]},
  Flatten[Map[{Cell[BoxData[RowBox[{"{", RowBox[{#, ",", "1"}], "}"}]], "Input"],
      Cell[BoxData[RowBox[{"{", RowBox[{TagBox[#, "SL31Name", SyntaxForm -> "symbol"], ",", "1"}], "}"}]], "Input"]} &, mine]],
  {With[{f = recv}, Cell[BoxData[MakeBoxes[$PreRead =.; Put[SL31Canon`received, f], StandardForm]], "Input"]]}];
Export[FileNameJoin[{dir, "in", "canon2.nb"}], Notebook[cells, StyleDefinitions -> "Default.nb"]];
Print["cells: ", Length[cells]];
