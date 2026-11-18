(* E15: how the Wolfram Language reads  -(a - b) (s) . c  and what Dot does with a number *)
Print["1. the form that the parser makes of  -(a - b) (s) . c :"];
Print["   ", ToString[FullForm[Hold[-(a - b) (s) . c]]]];
Print["2. the form of  (-(s) (a - b)) . c :"];
Print["   ", ToString[FullForm[Hold[(-(s) (a - b)) . c]]]];
m = {{1., 2.}, {3., 4.}};
r = Check[0.5 . m, "a message was issued"];
Print["3. 0.5 . {{1., 2.}, {3., 4.}}  gives  ", ToString[r, InputForm], "   (head: ", Head[r], ")"];
r2 = -({{1., 0.}, {0., 1.}} - {{0., 1.}, {1., 0.}}) (0.5) . m;
Print["4. -(A - B) (0.5) . m  gives  ", ToString[r2, InputForm]];
r3 = (-(0.5) ({{1., 0.}, {0., 1.}} - {{0., 1.}, {1., 0.}})) . m;
Print["5. (-(0.5) (A - B)) . m  gives  ", ToString[r3, InputForm]];
