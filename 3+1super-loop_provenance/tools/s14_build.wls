#!/usr/bin/env wolframscript
(* s14_build.wls -- builds in/s14.nb to find out why the cell of test S-14 produced no result:
   cell 1 loads the package and the registry and symbolizes every item (as sections I.3-I.6);
   cell 2 is the cell of test S-14 exactly as the builder made it (copied from in/3+1super-loop.nb);
   cell 3 reports; then the same list split into cells of 10 symbols, each reporting. *)
dir = "/Users/nsh/Developer/github/spacetimes/.git-transfer/mma";
nb = Get[FileNameJoin[{dir, "in", "3+1super-loop.nb"}]];
cells = Cases[nb, Cell[_, "Input", ___], Infinity];
s14 = SelectFirst[cells, ! FreeQ[#, "typed"] && ! FreeQ[#, "\"S-14\""] &];
Print[If[MissingQ[s14], "S-14 cell not found", "S-14 cell found"]];
(* the list of typed boxes inside it *)
listBoxes = FirstCase[s14, RowBox[{"typed", " ", "=", " ", RowBox[{"{", ___, RowBox[l_List], ___, "}"}]}] :> l, Missing[], Infinity];
items = If[MissingQ[listBoxes], {}, DeleteCases[listBoxes, "," | "\n" | "  " | " "]];
Print["typed items: ", Length[items]];
setup = StringJoin[
  "Needs[\"Notation`\"];\n",
  "Get[\"", dir, "/source/sl31_core.wl\"]; Get[\"", dir, "/source/registry_data.wl\"];\n",
  "segments = Flatten[Map[Function[e, Map[{e[\"entry\"], #} &, StringCases[e[\"symbol\"], \"$\" ~~ Shortest[x__] ~~ \"$\" :> x]]], SL31Registry], 1];\n",
  "items = Flatten[Map[Function[s, Map[<|\"entry\" -> s[[1]], \"tex\" -> #, \"boxes\" -> SL31TeXToBoxes[#]|> &, SL31SplitTeX[s[[2]]]]], segments], 1];\n",
  "items = Map[Append[#, \"kind\" -> SL31Classify[#[\"tex\"], #[\"boxes\"]]] &, items];\n",
  "toSymbolize = DeleteDuplicatesBy[Select[items, MemberQ[SL31Symbolized, #[\"kind\"]] &], #[\"tex\"] &];\n",
  "symbolTable = Map[<|\"tex\" -> #[\"tex\"], \"symbol\" -> SL31Symbolize[SL31SymbolBoxes[#[\"tex\"]]]|> &, toSymbolize];\n",
  "SL31Check[name_, st_, ok_, d_ : \"\"] := Print[name, \" \", TrueQ[ok], \" \", d];\n",
  "Length[symbolTable]"];
chunkCell[k_, part_] := Cell[BoxData[RowBox[{RowBox[{"chunk", "[", ToString[k], "]"}], "=", RowBox[{"{", RowBox[Riffle[part, ","]], "}"}]}]], "Input"];
reportCell[k_] := Cell[BoxData[RowBox[{"{", RowBox[{ToString[k], ",", RowBox[{"Length", "[", RowBox[{"chunk", "[", ToString[k], "]"}], "]"}], ",", RowBox[{"Count", "[", RowBox[{RowBox[{"chunk", "[", ToString[k], "]"}], ",", RowBox[{"_", "?", "AtomQ"}]}], "]"}]}], "}"}]], "Input"];
parts = Partition[items, UpTo[10]];
out = Notebook[Join[
   {Cell[setup, "Input"], s14, Cell["{ValueQ[typed], If[ValueQ[typed], {Length[typed], Count[typed, _?AtomQ]}, \"no value\"], $Line}", "Input"]},
   Flatten[MapIndexed[{chunkCell[First[#2], #1], reportCell[First[#2]]} &, parts]]], StyleDefinitions -> "Default.nb"];
Export[FileNameJoin[{dir, "in", "s14.nb"}], out];
Print["cells: ", Length[First[out]]];
