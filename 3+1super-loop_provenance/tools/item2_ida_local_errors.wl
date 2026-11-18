(* item2_ida_local_errors.wl      (PROJECT spacetimes, notebook 3+1super-loop.nb, chapter II.3)

   PURPOSE
   Measurements on the integrations by IDA of the checks AS-2 and AS-4 (and, for comparison, of GS-13), made to decide
   what these checks may require of IDA.  For every internal step the exact local error is computed (the computed
   state at the end of the step minus the exact solution that starts from the computed state at its beginning), and:
     wrmsMax       the largest weighted root-mean-square norm of an exact local error, with the weights of IDA,
                   1/(rtol |unknown| + atol), taken from the state at the beginning of the step.  IDA accepts a step if
                   this norm of its own ESTIMATE of the local error is at most 1;
     ratioMax      the largest ratio of the Euclidean norm of an exact local error to
                   budget_step = rtol (norm of the state) + atol sqrt(2 n_complex);
     sumLoc        the sum of the Euclidean norms of the exact local errors of all steps (a bound of the deviation at
                   the end of every step, because the evolution is unitary);
     zeroUnknowns  the number of real unknowns that are exactly zero at every time of the grid of IDA, and, for the
                   lattice states of 16 components per site, their numbers in the four groups particle and isospin up,
                   particle and isospin down, hole and isospin up, hole and isospin down.
   The file also prints three facts about the arithmetic that the rewriting of the gap equation relies on.

   USE
   The file needs the objects of the chapters II.1 to II.3.  It is appended to the dry run of Part II:
     python3 tools/make_dryrun.py OUT.wl notebook_source/part2_A_core.source.wl notebook_source/part2_B_item1.source.wl \
        notebook_source/part2_C_item1b.source.wl notebook_source/part2_D_item2.source.wl --append tools/item2_ida_local_errors.wl
     wolframscript -file OUT.wl
   The output of 2026-11-18 is logs/IDA_item2_local_errors.txt. *)
expAnalysis[ev_List, vv_?MatrixQ, rtol_, atol_][sol_] :=
  Module[{grid = Flatten[sol["Grid"]], vals = sol["ValuesOnGrid"], nc = Length[ev], npts, wrmsMax = 0., ratioMax = 0., sumLoc = 0., idx, wgrid, coef, errCoef,
    errVec, errReal, tolReal, nrm, isZero, groups},
   npts = Length[grid];
   (* for every real unknown: True if it is exactly zero at all grid times; the unknowns are the real parts of the nc
      complex components, then their imaginary parts *)
   isZero = Map[# == 0. &, Map[Max, Transpose[Abs[vals]]]];
   (* the four groups of the 16 components of a site: 1 particle up, 2 particle down, 3 hole up, 4 hole down *)
   groups = If[Mod[nc, 16] == 0,
     Table[Count[Pick[Join[#, #] &[Table[If[Mod[c - 1, 16] < 8, 0, 2] + If[OddQ[Mod[c - 1, 16]], 2, 1], {c, nc}]], isZero], grp], {grp, 4}], {}];
   Do[
    idx = Range[first, Min[first + 1000, npts]];
    wgrid = vals[[idx, 1 ;; nc]] + I vals[[idx, nc + 1 ;; 2 nc]];
    (* the coefficients of the states in the eigenvectors; the exact step multiplies them by the phase factors *)
    coef = wgrid . Conjugate[vv];
    errCoef = Rest[coef] - Most[coef] Exp[-I Outer[Times, Differences[grid[[idx]]], ev]];
    (* the local errors as vectors of the real unknowns, and the tolerances of the unknowns at the beginning of the steps *)
    errVec = errCoef . Transpose[vv];
    errReal = Join[Re[errVec], Im[errVec], 2];
    tolReal = rtol Abs[Most[vals[[idx]]]] + atol;
    nrm = Map[Norm, wgrid];
    wrmsMax = Max[wrmsMax, Max[Map[Sqrt[Mean[#^2]] &, errReal/tolReal]]];
    ratioMax = Max[ratioMax, Max[Map[Norm, errCoef]/(rtol Most[nrm] + atol Sqrt[2. nc])]];
    sumLoc += Total[Map[Norm, errCoef]],
    {first, 1, npts - 1, 1000}];
   <|"steps" -> npts - 1, "wrmsMax" -> wrmsMax, "ratioMax" -> ratioMax, "sumLoc" -> sumLoc, "realUnknowns" -> 2 nc, "zeroUnknowns" -> Count[isZero, True],
     "zeroUnknownsByGroup" -> groups|>];
expRun[name_, hm_, w0_, touts_, rtol_, atol_, exactStates_] := Module[{ev, vv, run, dev},
   {ev, vv} = t2eigh[hm];
   run = t2evolveIDA[hm, w0, touts, rtol, atol, expAnalysis[ev, vv, rtol, atol]];
   dev = Max[MapThread[Norm[#1 - #2] &, {run["states"], exactStates}]];
   Print[name, ": ", ToString[run["analysis"], InputForm], "; largest deviation at the output times ", ToString[dev, InputForm], "; largest h_IDA ",
    ToString[run["hmax"], InputForm], "; E_max ", ToString[Max[Abs[ev]], InputForm], "; largest deriv_residual ", ToString[Max[run["residualAt"]], InputForm]]];
expRun["AS-2 (rtol 1e-12, atol 1e-14)", as2H, as2Exact[0.], as2Touts, 1.*^-12, 1.*^-14, as2ExactStates];
expRun["AS-4 (rtol 1e-12, atol 1e-14)", as4H, as4Exact[0.], as4Touts, 1.*^-12, 1.*^-14, as4ExactStates];
Do[expRun["GS-13 (rtol " <> ToString[rt, InputForm] <> ", atol rtol/100)", hl, w0IDA13, touts13, rt, rt/100, exact13], {rt, rtols13}];
(* the arithmetic on which the gap at the critical coupling depends: the quotient of two machine numbers is the correctly
   rounded quotient; the ratio G/G_crit = 1 of the list of couplings is exactly 1; and at this ratio and mu = 0 the right
   side of the gap equation at the lower end 1e-12 of the bisection equals 1/G exactly *)
Print["division of machine numbers is correctly rounded (5./3., 224./280.): ", {5./3. === 1.6666666666666667, 224./280. === 0.8}];
Print["the coupling ratio number 80 is exactly 1: ", N[(20 + 80)/100] === 1.];
Print["t2gapIntegralClosed[1., 0., 1.*^-12] == 1/G_crit exactly: ", t2gapIntegralClosed[as5Lam, 0., 1.*^-12] === 1./as5Gcrit];
Print["the rows of the gap at mu = 0 for G/G_crit = 0.99, 1, 1.01: ", ToString[Select[as5GapRows, #[[1]] == 0. && 0.985 < #[[2]] < 1.015 &], InputForm]];
Print["computing times: t2eigh of the matrix of 960 rows of AS-9 ", Round[First[AbsoluteTiming[t2eigh[as9H];]], 0.1], " s; t2dense of the same lattice ",
  Round[First[AbsoluteTiming[t2dense[as9Lat, as9Fields];]], 0.01], " s"];
