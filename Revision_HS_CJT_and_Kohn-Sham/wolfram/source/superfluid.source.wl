(* ::Package:: *)

(* ::Title:: *)
(*The superfluid boson singlet state of the gauge-covariant spinor field on the 3+1 manifold of the Lorentz super-loop*)

(* ::Subtitle:: *)
(*PROJECT spacetimes, folder Revision_HS_CJT_and_Kohn-Sham. The notebook superfluid.nb, prepared for Patrick Lee Nash, Ph.D.*)

(* ::Text:: *)
(*This notebook is the Wolfram Language companion of the manuscript of the solution in the same folder, written so that a first-year graduate student in physics can follow every step. It states the problem and the conventions, defines every object that a computation needs, writes out the equations that can be executed, and executes them: the algebra of the manifold M31 and of the Lorentz super-loop, the spinor field and the gauge group GEE with its covariant derivatives, the field equations and their analytic solutions, the condensation of the spinor field into the superfluid boson singlet state by the methods of Hubbard and Stratonovich (HS), of Kohn and Sham (KS) and of Cornwall, Jackiw and Tomboulis (CJT), the effective Lagrangians and the Mexican hat, the breaking of the symmetry and the masses of the gauge bosons, the surviving symmetry with its massless photon, and the dynamics. Every time integration in this notebook is done by the method IDA of the function `NDSolve`; no fixed-step integrator (such as the fourth-order Runge-Kutta method) is used anywhere.*)

(* ::Text:: *)
(*How to use it. Open the notebook in Wolfram (version 15 or later; it was evaluated with Wolfram 15.0.1) and choose Evaluation > Evaluate Notebook. The cells are evaluated in order. Nothing outside this notebook is read: every definition and every program is inside it. Every check prints one line that begins with PASS or FAIL, and the last chapter lists all checks.*)

(* ::Text:: *)
(*How to read it. A text cell explains, in words and formulas, what the next input cell does and why. Inside the code, comments (* like this *) explain each step, and every symbol of the code that stands for an object of the physics has a comment with its printed name and its meaning the first time it is defined. A symbol of the printed text with the suffix LL is taken unchanged from the earlier work of the project on the super-loop; a symbol with the suffix SF is introduced by this solution.*)

(* ::Text:: *)
(*The checks and their counterparts. Every computation of this notebook is also done by a program in Rust and by a program in Python, in the folders `rust/` and `python/` of the folder of the solution; users without a Wolfram license can follow the solution with those programs. Every check of this notebook carries the identifier of the same check in the Rust program with the prefix W (for example WA-3 is the check A-3 of the Rust program `sf_part1` and of the Python program `sf_part1.py`), and the line that it prints names both counterparts. The notebook computes its values itself, by methods of the Wolfram Language (exact arithmetic where the Rust program computes in floating point, symbolic derivatives where it differentiates automatically, the solvers of the Wolfram Language where it uses its own); the numbers agree with those of the two programs within the tolerances stated in each check.*)

(* ::Chapter:: *)
(*0. The register of the checks*)

(* ::Text:: *)
(*The next cell defines the functions that record and print a check. `check[id, text, dev, tol, rust, python]` records a numerical check: it passes when the measured deviation `dev` is at most the tolerance `tol`; `rust` and `python` are the identifiers of the same check in the Rust and in the Python program. `checkTrue[id, text, ok, detail, rust, python]` records a check that is decided by facts (a count, an exact identity), with a short `detail`. Both append an association to the list ``SFNB`Test`$Results``, the register of all checks of this notebook, and print one line. The function `sci` writes a number with four significant digits in the form `2.857e-8`, as the reports of the Rust and the Python programs do.*)

(* ::Input:: *)
(* the register of the checks of this notebook: one association per check, in the order of evaluation *)
SFNB`Test`$Results = {};
(* sci[x]: the number x with four significant digits, written as mantissa e exponent (for example 2.857e-8) *)
sci[x_?NumericQ] := Module[{y = N[x], e, m},
   If[y == 0, Return["0", Module]];
   e = Floor[Log10[Abs[y]]]; m = Round[y/10.^e, 0.001];
   If[Abs[m] >= 10, m = m/10; e = e + 1];
   ToString[NumberForm[m, {4, 3}]] <> "e" <> ToString[e]];
(* checkHook[]: called after every check; it does nothing here (a dry run of the code without the front end
   uses it to print the computing time of each check) *)
checkHook[] := Null;
(* check[id, text, dev, tol, rust, python]: a numerical check, passed when dev <= tol *)
check[id_String, text_String, dev_?NumericQ, tol_?NumericQ, rust_String, python_String] := Module[{ok = TrueQ[N[dev] <= tol]},
   AppendTo[SFNB`Test`$Results, <|"id" -> id, "pass" -> ok, "deviation" -> N[dev], "tolerance" -> N[tol],
      "text" -> text, "rust" -> rust, "python" -> python|>];
   Print[If[ok, "PASS ", "FAIL "], id, "  deviation ", sci[dev], " (tolerance ", sci[tol], "): ", text,
     "   [the same check: Rust ", rust, ", Python ", python, "]"];
   checkHook[]; ok];
(* checkTrue[id, text, ok, detail, rust, python]: a check decided by facts; detail states them *)
checkTrue[id_String, text_String, ok_, detail_String, rust_String, python_String] := Module[{pass = TrueQ[ok]},
   AppendTo[SFNB`Test`$Results, <|"id" -> id, "pass" -> pass, "deviation" -> Missing["NotNumerical"], "tolerance" -> Missing["NotNumerical"],
      "text" -> text <> " [" <> detail <> "]", "rust" -> rust, "python" -> python|>];
   Print[If[pass, "PASS ", "FAIL "], id, "  ", text, " [", detail, "]   [the same check: Rust ", rust, ", Python ", python, "]"];
   checkHook[]; pass];

(* @@INCLUDECELLS part1.source.wl@@ *)

(* @@INCLUDECELLS part3_landau.source.wl@@ *)

(* ::Chapter:: *)
(*The list of all checks*)

(* ::Text:: *)
(*The last cell lists every check of this notebook with its result and its counterparts in the Rust and the Python programs, and counts the checks that passed.*)

(* ::Input:: *)
(* the table of all checks: identifier, result, deviation, tolerance, counterparts *)
Grid[Prepend[Map[{#["id"], If[#["pass"], "PASS", "FAIL"], If[NumericQ[#["deviation"]], sci[#["deviation"]], "--"],
      If[NumericQ[#["tolerance"]], sci[#["tolerance"]], "--"], #["rust"], #["python"]} &, SFNB`Test`$Results],
   Style[#, Bold] & /@ {"check", "result", "deviation", "tolerance", "Rust", "Python"}],
  Frame -> All, Alignment -> Left, BaseStyle -> {FontSize -> 9}]

(* ::Input:: *)
(* the count of the checks that passed *)
Print["checks of this notebook: ", Length[SFNB`Test`$Results], "; passed: ", Count[SFNB`Test`$Results, r_ /; r["pass"]],
  "; failed: ", Select[SFNB`Test`$Results, ! #["pass"] &][[All, "id"]]]
