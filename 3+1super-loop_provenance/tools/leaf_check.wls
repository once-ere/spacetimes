#!/usr/bin/env wolframscript
(* leaf_check.wls -- every string inside the converter's symbol boxes (not the tags of TagBox and
   not the option values of StyleBox), and how the front end's own parser reads it *)
dir = "/Users/nsh/Developer/github/spacetimes/.git-transfer/mma";
mine = Get[FileNameJoin[{dir, "out", "canon_mine.wl"}]];
leaves[b_] := Cases[b /. {TagBox[x_, _String, ___] :> x, StyleBox[x_, ___] :> x}, _String, Infinity];
all = Union[Flatten[Map[leaves, mine]]];
parsed = UsingFrontEnd[Map[# -> First[FrontEndExecute[FrontEnd`UndocumentedTestFEParserPacket[#, True]]] &, all]];
Print["distinct strings: ", Length[all]];
Print["read as one token by the front end: ", Count[parsed, (s_ -> BoxData[s_])]];
Print["read as several tokens:"];
Scan[Print["   ", ToString[#[[1]], InputForm], "  ->  ", ToString[#[[2]], InputForm]] &, Select[parsed, ! MatchQ[#, (s_ -> BoxData[s_])] &]];
Print["all strings: ", ToString[all, InputForm]];
