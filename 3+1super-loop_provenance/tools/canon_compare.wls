#!/usr/bin/env wolframscript
(* canon_compare.wls -- compares the boxes made by the converter with the boxes that the front end
   sent to the kernel (see canon_build.wls); prints every difference *)
dir = "/Users/nsh/Developer/github/spacetimes/.git-transfer/mma";
tex = Get[FileNameJoin[{dir, "out", "canon_tex.wl"}]];
mine = Get[FileNameJoin[{dir, "out", "canon_mine.wl"}]];
recv = Get[FileNameJoin[{dir, "out", "canon_received.wl"}]];
Print["items ", Length[tex], "  received ", Length[recv]];
(* the first received entry is the first cell (it sets $PreRead only afterwards, so it is not
   recorded); the last one is the cell that saves the list *)
got = Take[recv, UpTo[Length[tex]]];
diff = Select[Transpose[{tex, mine, PadRight[got, Length[tex], Missing[]]}], #[[2]] =!= #[[3]] &];
Print["differences ", Length[diff]];
Scan[Print[#[[1]], "\n   mine: ", ToString[#[[2]], InputForm], "\n   fe:   ", ToString[#[[3]], InputForm]] &, diff];
