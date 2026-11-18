#!/usr/bin/env wolframscript
(* reparse_check.wls -- the front end re-reads the boxes of an input cell when the notebook is
   written (and before a cell is sent to the kernel); a box that it reads as an OPERATOR (for
   example a partial-derivative sign with a subscript) makes it regroup the whole line.  For every
   symbolized box b this script writes the cell  {x, b, y}  into a notebook of the front end, reads
   it back, and reports every b for which the cell changed: first for b itself, then for b wrapped
   in TagBox[b, "SL31Name", SyntaxForm -> "symbol"]. *)
dir = "/Users/nsh/Developer/github/spacetimes/.git-transfer/mma";
Get[FileNameJoin[{dir, "source", "sl31_core.wl"}]];
Get[FileNameJoin[{dir, "source", "registry_data.wl"}]];
segments = Flatten[Map[Function[e, Map[{e["entry"], #} &, StringCases[e["symbol"], "$" ~~ Shortest[x__] ~~ "$" :> x]]], SL31Registry], 1];
items = Flatten[Map[Function[s, Map[<|"entry" -> s[[1]], "tex" -> #, "boxes" -> SL31TeXToBoxes[#]|> &, SL31SplitTeX[s[[2]]]]], segments], 1];
items = Map[Append[#, "kind" -> SL31Classify[#["tex"], #["boxes"]]] &, items];
toSymbolize = DeleteDuplicatesBy[Select[items, MemberQ[SL31Symbolized, #["kind"]] &], #["tex"] &];
groups = GatherBy[toSymbolize, SL31SymbolBoxes[#["tex"]] &];
boxes = Map[SL31SymbolBoxes[First[#]["tex"]] &, groups];
texs = Map[First[#]["tex"] &, groups];
wrap[TagBox[x_, "SL31Name", opts___]] := TagBox[x, "SL31Name", SyntaxForm -> "symbol"];
wrap[b_] := TagBox[b, "SL31Name", SyntaxForm -> "symbol"];
cellOf[b_] := Cell[BoxData[RowBox[{"{", RowBox[{"x", ",", b, ",", "y"}], "}"}]], "Input"];
check[bs_] := UsingFrontEnd[Module[{nb = NotebookPut[Notebook[Map[cellOf, bs], Visible -> False]], back},
    back = NotebookGet[nb]; NotebookClose[nb];
    Map[First, Cases[back, Cell[c_, "Input", ___], Infinity]]]];
backPlain = check[boxes];
badPlain = Select[Range[Length[boxes]], backPlain[[#]] =!= BoxData[RowBox[{"{", RowBox[{"x", ",", boxes[[#]], ",", "y"}], "}"}]] &];
Print["symbols: ", Length[boxes], "   regrouped by the front end (plain box): ", Length[badPlain]];
Scan[Print["   ", texs[[#]], "   kind ", First[groups[[#]]]["kind"], "\n      box:  ", ToString[boxes[[#]], InputForm],
    "\n      read: ", ToString[backPlain[[#]], InputForm]] &, badPlain];
wrapped = Map[wrap, boxes[[badPlain]]];
backWrapped = check[wrapped];
badWrapped = Select[Range[Length[wrapped]], backWrapped[[#]] =!= BoxData[RowBox[{"{", RowBox[{"x", ",", wrapped[[#]], ",", "y"}], "}"}]] &];
Print["regrouped after wrapping with SyntaxForm -> symbol: ", Length[badWrapped]];
Scan[Print["   ", texs[[badPlain[[#]]]], "\n      read: ", ToString[backWrapped[[#]], InputForm]] &, badWrapped];
