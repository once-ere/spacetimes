#!/usr/bin/env wolframscript
dir = "/Users/nsh/Developer/github/spacetimes/.git-transfer/mma";
tex = Get[FileNameJoin[{dir, "out", "canon2_tex.wl"}]];
mine = Get[FileNameJoin[{dir, "out", "canon2_mine.wl"}]];
recv = Get[FileNameJoin[{dir, "out", "canon2_received.wl"}]];
Print["items ", Length[tex], "  received ", Length[recv]];
expectA = Map[RowBox[{"{", RowBox[{#, ",", "1"}], "}"}] &, mine];
expectB = Map[RowBox[{"{", RowBox[{TagBox[#, "SL31Name", SyntaxForm -> "symbol"], ",", "1"}], "}"}] &, mine];
gotA = recv[[1 ;; 2 Length[tex] ;; 2]]; gotB = recv[[2 ;; 2 Length[tex] ;; 2]];
badA = Select[Range[Length[tex]], gotA[[#]] =!= expectA[[#]] &];
badB = Select[Range[Length[tex]], gotB[[#]] =!= expectB[[#]] &];
Print["not kept as an operand (plain box): ", Length[badA]];
Scan[Print["   ", tex[[#]], "\n      sent: ", ToString[gotA[[#]], InputForm]] &, badA];
Print["not kept as an operand (wrapped with SyntaxForm -> symbol): ", Length[badB]];
Scan[Print["   ", tex[[#]], "\n      sent: ", ToString[gotB[[#]], InputForm]] &, badB];
