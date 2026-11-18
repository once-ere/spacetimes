#!/usr/bin/env wolframscript
(* canon_build.wls -- builds in/canon.nb: one input cell for every symbolized TeX item of the
   registry, holding exactly the boxes made by the converter of sl31_core.wl.  When the notebook is
   evaluated in the front end, $PreRead (set in the first cell) records the boxes that the front end
   really sends to the kernel for every cell; the last cell saves them.  Comparing them with the
   converter's boxes shows which boxes the front end changes before the kernel reads them. *)
dir = "/Users/nsh/Developer/github/spacetimes/.git-transfer/mma";
Get[FileNameJoin[{dir, "source", "sl31_core.wl"}]];
Get[FileNameJoin[{dir, "source", "registry_data.wl"}]];
segments = Flatten[Map[Function[e, Map[{e["entry"], #} &, StringCases[e["symbol"], "$" ~~ Shortest[x__] ~~ "$" :> x]]], SL31Registry], 1];
items = Flatten[Map[Function[s, Map[<|"entry" -> s[[1]], "tex" -> #, "boxes" -> SL31TeXToBoxes[#]|> &, SL31SplitTeX[s[[2]]]]], segments], 1];
items = Map[Append[#, "kind" -> SL31Classify[#["tex"], #["boxes"]]] &, items];
toSymbolize = DeleteDuplicatesBy[Select[items, MemberQ[SL31Symbolized, #["kind"]] &], #["tex"] &];
mine = Map[SL31SymbolBoxes[#["tex"]] &, toSymbolize];
Put[Map[#["tex"] &, toSymbolize], FileNameJoin[{dir, "out", "canon_tex.wl"}]];
Put[mine, FileNameJoin[{dir, "out", "canon_mine.wl"}]];
recv = FileNameJoin[{dir, "out", "canon_received.wl"}];
cells = Join[
  {Cell[BoxData[MakeBoxes[SL31Canon`received = {}; $PreRead = Function[b, AppendTo[SL31Canon`received, b]; b], StandardForm]], "Input"]},
  Map[Cell[BoxData[#], "Input"] &, mine],
  {With[{f = recv}, Cell[BoxData[MakeBoxes[$PreRead =.; Put[SL31Canon`received, f], StandardForm]], "Input"]]}];
Export[FileNameJoin[{dir, "in", "canon.nb"}], Notebook[cells, StyleDefinitions -> "Default.nb"]];
Print[{Length[toSymbolize], Length[cells]}];
