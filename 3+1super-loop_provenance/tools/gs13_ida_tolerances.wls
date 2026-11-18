#!/usr/bin/env wolframscript
(* gs13_ida_tolerances.wls  (PROJECT spacetimes, notebook 3+1super-loop.nb, check GS-13)
   Which tolerances does NDSolve hand to the SUNDIALS integrator IDA?  The error test of IDA accepts a step if the
   weighted root-mean-square norm of its estimated local error is at most 1, with the weight 1/(rtol |y_j| + atol) of
   the real component y_j.  The documentation of NDSolve states that PrecisionGoal -> prec asks for the relative
   local error 10^-prec and AccuracyGoal -> acc for the absolute local error 10^-acc.  This script tests the three
   consequences of  rtol |y_j| + atol = kappa (10^-prec |y_j| + 10^-acc)  that can be tested from outside, through
   the list of the times of the internal steps (the "grid") of the solution of a small linear system:
     T1  with a negligible relative part (prec = 15), multiplying the solution by 1024 and lowering acc by
         log10(1024) must leave the grid unchanged up to rounding (the absolute part is proportional to 10^-acc);
     T2  with a negligible absolute part (acc = 15), multiplying the solution by 1024 (and lowering acc by
         log10(1024)) must leave the grid unchanged (the relative part is proportional to |y_j|);
     T3  a solution shifted by the large constant K = 2^20, integrated with (prec = 12, acc = 15), must give the
         grid of the unshifted solution integrated with (prec = 15, acc = 12 - log10(K)), up to the relative size
         1/K of the solution compared with the shift (the same constant kappa multiplies both parts); T3a and T3b
         show that the test would notice a factor 2 between the constants of the two parts.
   The constant kappa itself cannot be seen from outside; the reference page of NDSolve fixes it to 1.
   The script also prints the options of the method and what NDSolve answers to a complex system. *)
Print["$Version = ", $Version];
Print["Options[NDSolve`IDA] = ", Options[NDSolve`IDA]];
(* two oscillators with the frequencies 1 and 2.5, as a real antisymmetric 4 x 4 system *)
mat = {{0., 1., 0., 0.}, {-1., 0., 0., 0.}, {0., 0., 0., 2.5}, {0., 0., -2.5, 0.}};
y0 = {1., 0.3, -0.2, 0.7};
gridOf[scale_, shift_, pg_, ag_] := Module[{y, s, sol},
   sol = NDSolveValue[{y'[s] == mat . (y[s] - shift), y[0] == scale y0 + shift}, y, {s, 0, 30}, Method -> "IDA",
     PrecisionGoal -> pg, AccuracyGoal -> ag, MaxSteps -> 10^6];
   If[Head[sol] === InterpolatingFunction, Flatten[sol["Grid"]], $Failed]];
compare[name_, g1_, g2_] := Print[name, ": steps ", Length[g1] - 1, " and ", Length[g2] - 1, "; last times ", Last[g1] // InputForm, " and ",
   Last[g2] // InputForm, If[Length[g1] == Length[g2], Row[{"; largest difference of the step times ", Max[Abs[g1 - g2]] // InputForm,
      "; largest relative difference of the step sizes ", Max[Abs[Differences[g1]/Differences[g2] - 1]] // InputForm}],
    Row[{"; the first ", Min[Length[g1], Length[g2]], " step times differ by at most ",
      Max[Abs[Take[g1, Min[Length[g1], Length[g2]]] - Take[g2, Min[Length[g1], Length[g2]]]]] // InputForm}]]];
zero = {0., 0., 0., 0.};
kshift = 2.^20;
(* T1: the relative part switched off by prec = 15 (10^-15 |y_j| is 10^-7 of the absolute part 10^-8) *)
compare["T1 (prec = 15; acc = 8 against the solution times 1024 with acc = 8 - log10(1024))",
  gridOf[1., zero, 15, 8], gridOf[1024., zero, 15, 8 - Log10[1024.]]];
(* T2: the absolute part switched off by acc = 15 where the components are of the order 1 (it acts only within
   10^-7 of a zero of a component) *)
compare["T2 (acc = 15, prec = 8; the solution against the solution times 1024 with acc = 15 - log10(1024))",
  gridOf[1., zero, 8, 15], gridOf[1024., zero, 8, 15 - Log10[1024.]]];
(* T3 *)
compare["T3 (the solution shifted by 2^20 with prec = 12, acc = 15, against the unshifted solution with prec = 15, acc = 12 - log10(2^20))",
  gridOf[1., kshift {1., 1., 1., 1.}, 12, 15], gridOf[1., zero, 15, 12 - Log10[kshift]]];
(* for comparison: with the absolute tolerance of the second run doubled or halved the numbers of steps must differ
   from those of T3 (the test can tell a factor 2 between the constants of the two parts) *)
compare["T3a (as T3, the absolute tolerance of the second run doubled)",
  gridOf[1., kshift {1., 1., 1., 1.}, 12, 15], gridOf[1., zero, 15, 12 - Log10[kshift] - Log10[2.]]];
compare["T3b (as T3, the absolute tolerance of the second run halved)",
  gridOf[1., kshift {1., 1., 1., 1.}, 12, 15], gridOf[1., zero, 15, 12 - Log10[kshift] + Log10[2.]]];
(* what PrecisionGoal -> Infinity and AccuracyGoal -> Infinity do *)
Module[{g}, g = gridOf[1., zero, Infinity, 8]; Print["prec = Infinity, acc = 8: ", If[ListQ[g], Row[{Length[g] - 1, " steps, last time ", Last[g] // InputForm}], g]]];
Module[{g}, g = gridOf[1., zero, 8, Infinity]; Print["prec = 8, acc = Infinity: ", If[ListQ[g], Row[{Length[g] - 1, " steps, last time ", Last[g] // InputForm}], g]]];
(* a complex system *)
Module[{w, s, r},
  r = Check[NDSolveValue[{w'[s] == -I w[s], w[0] == 1. + 0.5 I}, w, {s, 0, 1}, Method -> "IDA"], "no solution (message above)"];
  Print["a complex equation given to IDA: ", Head[r] /. String -> r]];
Print["done"];
