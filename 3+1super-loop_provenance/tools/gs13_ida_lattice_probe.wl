(* probe 151 (PROJECT spacetimes, notebook 3+1super-loop.nb, check GS-13): IDA through NDSolve on the lattice of GS-10
   (the objects lat, fl, hl of the chapters before).  Questions:
   (1) the deviations from the exact spectral solution at the 41 output times, the numbers of steps, the step sizes;
   (2) are the derivative data of the InterpolatingFunction equal to m . y at the grid points?
   (3) the deviations AT grid points (no interpolation) compared with those at the output times;
   (4) does EvaluationMonitor count the evaluations, and does NDSolve accept the complex system directly? *)
rngKept = t2rngState; t2Seed[2026111713]; w0p = t2RndVec[t2dim[lat]]; t2rngState = rngKept; w0p = w0p/Norm[w0p];
Print["first components of the initial state: ", Take[w0p, 2] // InputForm];
{ep, vp} = t2eigh[hl];
Print["eigensolver: residual ", Norm[hl . vp - vp . DiagonalMatrix[ep], "Frobenius"], ", orthonormality ",
  Norm[ConjugateTranspose[vp] . vp - IdentityMatrix[Length[ep]], "Frobenius"], ", spectral radius ", Max[Abs[ep]] // InputForm];
coefp = ConjugateTranspose[vp] . w0p;
exactp[t_?NumericQ] := vp . (Exp[-I ep t] coefp);
toutsp = Range[0., 20., 0.5];
ncp = Length[w0p];
mp = Developer`ToPackedArray[N[ArrayFlatten[{{Im[hl], Re[hl]}, {-Re[hl], Im[hl]}}]]];
Print["m + transpose(m): ", Max[Abs[mp + Transpose[mp]]], "; 2-norm of m: ", Norm[mp] // InputForm, "; packed: ", Developer`PackedArrayQ[mp],
  "; nonzero entries of m: ", Count[Flatten[mp], x_ /; x != 0.], " of ", Length[mp]^2];
cplxp[y_] := y[[1 ;; ncp]] + I y[[ncp + 1 ;; 2 ncp]];
probe[rt_, monitor_] := Module[{nev = 0, t, sol, grid, devs, dsol, gsel, resid, devGrid, opts, y, s},
  opts = If[monitor, {EvaluationMonitor :> (nev++)}, {}];
  {t, sol} = AbsoluteTiming[NDSolveValue[{y'[s] == mp . y[s], y[0] == Join[Re[w0p], Im[w0p]]}, y, {s, 0, 20}, Method -> "IDA",
      PrecisionGoal -> -Log10[rt], AccuracyGoal -> -Log10[rt/100], MaxSteps -> 10^7, Sequence @@ opts]];
  If[Head[sol] =!= InterpolatingFunction, Print["  no solution: ", Short[sol, 2]]; Return[Null, Module]];
  grid = Flatten[sol["Grid"]];
  devs = Table[Norm[cplxp[sol[tt]] - exactp[tt]]/Norm[exactp[tt]], {tt, toutsp}];
  dsol = Derivative[1][sol];
  gsel = grid[[Round[Subdivide[2, Length[grid] - 1, 40]]]];
  resid = Max[Table[Norm[dsol[tg] - mp . sol[tg]], {tg, gsel}]];
  devGrid = Table[Norm[cplxp[sol[tg]] - exactp[tg]], {tg, gsel}];
  Print["rtol ", rt // InputForm, If[monitor, " (with EvaluationMonitor)", ""], ": seconds ", Round[t, 0.01], ", steps ", Length[grid] - 1,
    ", evaluations ", If[monitor, nev, "not counted"], ", largest deviation at the outputs ", Max[devs] // InputForm, ", at 20: ", Last[devs] // InputForm,
    ", at 0: ", First[devs] // InputForm, ", largest norm change ", Max[Abs[(Norm[cplxp[sol[#]]] & /@ toutsp) - 1]] // InputForm,
    ", largest step ", Max[Differences[grid]] // InputForm, ", smallest step ", Min[Differences[grid]] // InputForm,
    ", largest |y' - m y| on 41 grid points ", resid // InputForm, ", deviations on these grid points: largest ", Max[devGrid] // InputForm,
    ", last ", Last[devGrid] // InputForm, " at ", Last[gsel] // InputForm, ", interpolation order ", sol["InterpolationOrder"]];
  ];
probe[1.*^-8, False];
probe[1.*^-10, False];
probe[1.*^-12, False];
probe[1.*^-10, True];
(* the complex system given directly *)
Module[{w, s, t, sol},
  {t, sol} = AbsoluteTiming[Check[NDSolveValue[{w'[s] == -I hl . w[s], w[0] == w0p}, w, {s, 0, 20}, Method -> "IDA", PrecisionGoal -> 10,
      AccuracyGoal -> 12, MaxSteps -> 10^7], $Failed]];
  Print["complex system: seconds ", Round[t, 0.01], ", head ", Head[sol], If[Head[sol] === InterpolatingFunction,
     Row[{", steps ", Length[Flatten[sol["Grid"]]] - 1, ", deviation at 20: ", Norm[sol[20.] - exactp[20.]] // InputForm}], ""]]];
Print["probe 151 done"];
