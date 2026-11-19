(* probe 217 (PROJECT spacetimes, notebook 3+1super-loop.nb, checks GS-13 and GS-17 to GS-20): the internal steps of
   IDA through NDSolve on the lattice of GS-10 (the objects lat, fl, hl of the chapters before), read from the grid of
   the solution.  NDSolve returns the times t_0 = 0, t_1, ..., t_N = 20 at the ends of the internal steps of IDA ("the
   grid") as machine numbers; the sizes of the steps themselves stay inside IDA.  Questions:
   (1) After its first steps IDA keeps one step size h ("the settled step size") to the last step but one.  Which
       machine number is h?  IDA advances its time by the rounded sum t_s = fl(t_(s-1) + h); every grid point of the
       settled steps therefore confines h to an interval of the width of one unit in the last place of t_s, and the
       machine numbers in the intersection of all these intervals are the candidates for h.  A candidate is accepted
       if repeated machine addition of it, started at the grid point before the first settled step, reproduces every
       grid point of the settled steps bit for bit.
   (2) The time defect of the settled steps: (number of settled steps) x h minus the time that the grid advances over
       them, in exact rational arithmetic.  It is the sum of the rounding errors of the additions, with the sign
       "exact sum of the step sizes minus reported time".
   (3) The sizes of the steps before the settled ones and of the last step (differences of grid points, exact), and
       the rounding error of one addition t_s - (t_(s-1) + h), in units of the last place of t_s, for the settled
       steps grouped by this unit (it is the same number for all steps that begin and end between the same two powers
       of 2, because there all times are multiples of one unit).
   The numbers are used by the program t2_fields of the folder Revision_SuperLoop (check GS-20) and by the notebook. *)
rngKept = t2rngState; t2Seed[2026111713]; w0s = t2RndVec[t2dim[lat]]; t2rngState = rngKept; w0s = w0s/Norm[w0s];
mps = Developer`ToPackedArray[N[ArrayFlatten[{{Im[hl], Re[hl]}, {-Re[hl], Im[hl]}}]]];
(* the exact value of a machine number, and its unit in the last place as an exact power of 2 *)
exactOf[x_] := SetPrecision[x, Infinity];
unitOf[x_] := 2^(Last[MantissaExponent[x, 2]] - 53);
(* a machine number is printed with the digits that identify it (InputForm); the integer k and the exponent e with
   x = k 2^e state it exactly *)
show17[x_] := ToString[N[x], InputForm];
showExact[x_] := Module[{m, e}, {m, e} = MantissaExponent[x, 2]; ToString[Round[m 2^53]] <> " x 2^(" <> ToString[e - 53] <> ")"];
settledProbe[rt_] := Module[{rtText = ToString[rt, InputForm], sol, y, s, grid, nst, d, u, dmed, ok, runs, a, b, hLow, hHigh, uh, kLow, kHigh, cands, good, defects, early, units,
    frac, bin, tab},
  sol = NDSolveValue[{y'[s] == mps . y[s], y[0] == Join[Re[w0s], Im[w0s]]}, y, {s, 0, 20}, Method -> "IDA",
    PrecisionGoal -> -Log10[rt], AccuracyGoal -> -Log10[rt/100], MaxSteps -> 10^7];
  grid = Flatten[sol["Grid"]];
  nst = Length[grid] - 1;
  (* step s leads from grid[[s]] to grid[[s + 1]]; its size as the exact difference, and the unit of its end point *)
  d = Differences[exactOf /@ grid];
  u = unitOf /@ Rest[grid];
  (* the settled steps: the longest run of consecutive steps whose exact differences lie within one unit in the last
     place of the time 20 (3.6e-15, the largest unit of the grid) of the median difference; two different step sizes
     of IDA differ by far more *)
  dmed = Median[d];
  ok = Table[Boole[Abs[d[[k]] - dmed] <= unitOf[20.]], {k, nst}];
  runs = Split[Transpose[{Range[nst], ok}], #1[[2]] == #2[[2]] &];
  runs = Select[runs, #[[1, 2]] == 1 &];
  {a, b} = {#[[1, 1]], #[[-1, 1]]} &[First[MaximalBy[runs, Length]]];
  (* the interval of h: t_s = fl(t_(s-1) + h) means |t_(s-1) + h - t_s| <= u_s/2 *)
  hLow = Max[Table[d[[k]] - u[[k]]/2, {k, a, b}]];
  hHigh = Min[Table[d[[k]] + u[[k]]/2, {k, a, b}]];
  uh = unitOf[N[dmed]];
  {kLow, kHigh} = {Ceiling[hLow/uh], Floor[hHigh/uh]};
  cands = Table[N[k uh], {k, kLow, kHigh}];
  (* the candidates that reproduce the grid of the settled steps by machine addition *)
  good = Select[cands, Function[c, Max[Abs[FoldList[Plus, grid[[a]], ConstantArray[c, b - a + 1]] - grid[[a ;; b + 1]]]] == 0.]];
  defects = Table[(b - a + 1) exactOf[c] - (exactOf[grid[[b + 1]]] - exactOf[grid[[a]]]), {c, good}];
  early = Total[Table[u[[k]]/2, {k, 1, a - 1}]];
  Print["SETTLED rtol ", rtText, ": internal steps ", nst, "; settled steps from the step ", a, " to the step ", b, " (", b - a + 1,
    " steps); grid point before the first settled step ", show17[grid[[a]]], " = ", showExact[grid[[a]]], "; after the last settled step ",
    show17[grid[[b + 1]]], "; end ", show17[Last[grid]]];
  Print["SETTLED rtol ", rtText, ": interval of h from the grid: width ", show17[hHigh - hLow], " = ",
    show17[(hHigh - hLow)/uh], " units in the last place of h; machine numbers in it: ", Length[cands], "; of these reproduce the grid bit for bit: ",
    Length[good]];
  Do[Print["SETTLED rtol ", rtText, ": settled step size ", show17[good[[j]]], " = ", showExact[good[[j]]], "; time defect of the settled steps ",
     show17[defects[[j]]]], {j, Length[good]}];
  Print["SETTLED rtol ", rtText, ": bound of the rounding errors of the ", a - 1, " steps before the settled ones (half a unit each) ",
    show17[early], "; the last step: grid difference ", show17[d[[nst]]], ", the machine subtraction of its two grid points is exact: ",
    exactOf[grid[[nst + 1]] - grid[[nst]]] == d[[nst]]];
  Print["SETTLED rtol ", rtText, ": sizes of the steps 1 to ", a, " (the machine numbers nearest to the exact differences of the grid points): ",
    StringRiffle[Table[show17[d[[k]]], {k, 1, a}], ", "]];
  If[good =!= {},
   (* the rounding error of one addition, t_s - (t_(s-1) + h), in units of the last place of t_s, grouped by this unit *)
   frac = Table[(exactOf[grid[[k + 1]]] - exactOf[grid[[k]]] - exactOf[First[good]])/u[[k]], {k, a, b}];
   units = u[[a ;; b]];
   bin = GatherBy[Transpose[{units, frac}], First];
   tab = Table[{N[g[[1, 1]]], Length[g], N[Median[g[[All, 2]]]], Count[g[[All, 2]], x_ /; x != Median[g[[All, 2]]]], N[Total[g[[All, 2]]] g[[1, 1]]]}, {g, bin}];
   Print["SETTLED rtol ", rtText, ": settled steps by the unit in the last place of their end point {unit, steps, rounding error of one addition in units (the median over the steps), steps with another rounding error (a step that begins below a power of 2 and ends above it), sum of the rounding errors}: ",
    ToString[tab, InputForm]];
   Print["SETTLED rtol ", rtText, ": minus the sum of all these rounding errors ", show17[-Total[tab[[All, 5]]]]]];
  ];
settledProbe[1.*^-8];
settledProbe[1.*^-10];
settledProbe[1.*^-12];
Print["probe 217 done"];
