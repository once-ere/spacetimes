#!/usr/bin/env wolframscript
(* gs13_ida_local_errors.wls  (PROJECT spacetimes, notebook 3+1super-loop.nb, check GS-13)
   Which local errors does the SUNDIALS integrator IDA accept when NDSolve calls it with the options
   PrecisionGoal -> prec and AccuracyGoal -> acc?  Measured directly on the equation y' = -y, y(0) = 1, whose exact
   solution exp(-t) falls from 1 to 10^-11 on 0 <= t <= 25: the exact local error of the step from t_n to t_n+1 is
       e_n = y_(n+1) - y_n exp(-(t_(n+1) - t_n)),
   the difference between the computed value and the exact solution that starts from the computed value y_n.
   If the error test of IDA uses the weight 1/(rtol |y| + atol) with rtol = 10^-prec and atol = 10^-acc, then
     * while |y| is much larger than atol/rtol the accepted errors are a fixed fraction of rtol |y| and the steps
       have a constant size (the equation and the test are unchanged when y is multiplied by a constant);
     * once |y| is much smaller than atol/rtol the accepted errors are a fraction of atol, and the steps grow.
   For every pair (prec, acc) the script prints, for windows of the time, the mean step, the modulus of y, the
   largest |e_n| and the largest |e_n|/|y_n|, and the two ratios (largest |e_n|/|y_n|)/10^-prec, (largest |e_n|)/10^-acc. *)
Print["$Version = ", $Version];
(* a number with three significant digits on one line of text, for example 1.23e-6 *)
sci[x_] := If[x == 0, "0", With[{e = Floor[Log10[Abs[x]]]}, ToString[NumberForm[x/10.^e, {3, 2}]] <> "e" <> ToString[e]]];
run[pg_, ag_] := Module[{y, s, sol, grid, vals, h, e, rows},
   sol = NDSolveValue[{y'[s] == -y[s], y[0] == 1.}, y, {s, 0, 25}, Method -> "IDA", PrecisionGoal -> pg, AccuracyGoal -> ag, MaxSteps -> 10^6];
   If[Head[sol] =!= InterpolatingFunction, Print["prec = ", pg, ", acc = ", ag, ": no solution"]; Return[Null, Module]];
   grid = Flatten[sol["Grid"]]; vals = Flatten[sol["ValuesOnGrid"]];
   h = Differences[grid];
   e = Abs[Rest[vals] - Most[vals] Exp[-h]];
   Print["prec = ", pg, ", acc = ", ag, ": ", Length[h], " steps, last time ", Last[grid] // InputForm, "; deviation from exp(-t) at the end ",
     Abs[Last[vals] - Exp[-Last[grid]]] // InputForm, "; atol/rtol = ", N[10^(pg - ag)] // InputForm];
   rows = Table[With[{idx = Flatten[Position[Most[grid], tt_ /; t0 <= tt < t0 + 2.5, {1}]]},
      If[idx === {}, Nothing, {t0, Mean[h[[idx]]], Abs[vals[[First[idx]]]], Max[e[[idx]]], Max[e[[idx]]/Abs[vals[[idx]]]],
        Max[e[[idx]]/Abs[vals[[idx]]]]/10.^-pg, Max[e[[idx]]]/10.^-ag, Length[idx]}]], {t0, 2.5, 22.5, 2.5}];
   Print["   window from | mean step | |y| at its start | largest |e_n| | largest |e_n|/|y_n| | the latter / 10^-prec | largest |e_n| / 10^-acc | steps"];
   Scan[Print["   ", ToString[NumberForm[#[[1]], {4, 1}]], " | ", sci[#[[2]]], " | ", sci[#[[3]]], " | ", sci[#[[4]]], " | ",
      sci[#[[5]]], " | ", sci[#[[6]]], " | ", sci[#[[7]]], " | ", #[[8]]] &, rows]];
Scan[Apply[run], {{6, 10}, {8, 10}, {8, 12}, {10, 8}, {6, 6}, {12, 14}}];
Print["done"];
