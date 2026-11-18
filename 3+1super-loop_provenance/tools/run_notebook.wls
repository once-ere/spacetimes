#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   SUPERSEDED.  This evaluator ran the cells of a notebook in a kernel started by wolframscript, with
   a front end that has no window.  It was replaced by guirun.wl, which lets the front end of
   Wolfram.app itself evaluate the notebook (Evaluation > Evaluate Notebook) in a fresh kernel.
   Two defects of this file are recorded in the provenance P_3+1super-loop (errors E2 and E6): a
   cell with several statements on separate lines was read as ONE product of the statements, and
   some runs stopped without a result.  Its last version is kept below this note as the record of
   what was tried; the splitting of a cell into statements at its line breaks (SL31Run`statements)
   was written after the last run of this file and was never executed, because the evaluation was
   moved to the front end of Wolfram.app.
   ================================================================================================ *)
(* ================================================================================================
   run_notebook.wls        (PROJECT spacetimes, notebook 3+1super-loop.nb)

   PURPOSE
   Evaluates every input cell of ONE notebook, in order, in a FRESH Wolfram kernel that is
   connected to a front end (UsingFrontEnd), exactly as "Evaluation > Evaluate Notebook" does in
   Wolfram.app, and saves the evaluated notebook (inputs, typeset outputs, printed lines and
   messages).  It also exports the evaluated notebook to PDF and writes a plain-text log.

   WHY NOT NotebookEvaluate?
   NotebookEvaluate[nb, InsertResults -> True] works, but in a script kernel it inserts the
   results as plain-text OutputForm ("OutputFormData") instead of typeset StandardForm.  This
   script therefore performs the same three steps as the interactive front end itself:
     1. read the cell; 2D input (BoxData) is converted to an expression by MakeExpression in
        StandardForm (this is where the Notation package recognizes symbolized boxes), and a
        cell holding a string is read as InputForm text;
     2. evaluate it, collecting printed lines (Print) and messages;
     3. write Print, Message and Output cells after the input cell; the output is typeset with
        MakeBoxes in StandardForm (where the Notation package displays symbolized symbols).

   USAGE
     wolframscript -file run_notebook.wls  <input.nb>  <output.nb>  <log.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 3, Print["usage: run_notebook.wls in.nb out.nb log.txt"]; Exit[2]];
{inNB, outNB, logFile} = args[[1 ;; 3]];

(* every message raised anywhere (except the notices of new symbols, which are off by default) *)
SL31Run`allMessages = {};
SL31Run`cellMessages = {};
Internal`AddHandler["Message",
  If[! MatchQ[#, Hold[Message[General::newsym, ___], _]],
     AppendTo[SL31Run`allMessages, #]; AppendTo[SL31Run`cellMessages, #]] &];

(* the text of a message, as the front end prints it *)
SL31Run`messageText[Hold[Message[mn : MessageName[s_, tag_], args___], _]] :=
  Module[{template},
    (* the template of s::tag, or of General::tag, as Message itself would choose it *)
    template = Replace[MessageName[s, tag],
      _MessageName :> Replace[MessageName[General, tag], _MessageName -> "(no message text)"]];
    ToString[HoldForm[mn]] <> ": " <> ToString[StringForm[template, args]]];
SL31Run`messageText[other_] := ToString[other, InputForm];

(* convert the content of an input cell into held expressions, one for every statement.
   A cell may hold several statements on separate lines.  The front end separates them at the
   line breaks of the outermost level of the box structure and sends them one by one; a whole
   multi-line box given at once to ToExpression would instead read the line breaks as spaces,
   that is, as multiplication.  The same separation is made here: the outermost RowBox is cut at
   its line-break tokens, comments and white space are dropped, and every remaining piece is one
   statement. *)
SL31Run`commentQ[RowBox[{"(*", ___, "*)"}]] := True;
SL31Run`commentQ[_] := False;
(* white space, and a line break (a white-space string that contains a newline character) *)
SL31Run`blankQ[s_String] := StringMatchQ[s, Whitespace | ""];
SL31Run`blankQ[_] := False;
SL31Run`newlineQ[s_String] := StringMatchQ[s, Whitespace] && StringContainsQ[s, "\n"];
SL31Run`newlineQ[_] := False;
(* leading and trailing white space and comments of one statement are removed; the inside of a
   statement is left exactly as it is *)
SL31Run`trim[items_List] := Module[{r = items},
   While[r =!= {} && (SL31Run`blankQ[First[r]] || SL31Run`commentQ[First[r]]), r = Rest[r]];
   While[r =!= {} && (SL31Run`blankQ[Last[r]] || SL31Run`commentQ[Last[r]]), r = Most[r]];
   r];
SL31Run`statements[RowBox[items_List]] := Module[{pieces},
   pieces = SplitBy[items, SL31Run`newlineQ];
   pieces = Select[pieces, ! SL31Run`newlineQ[First[#]] &];
   pieces = Select[Map[SL31Run`trim, pieces], # =!= {} &];
   Map[If[Length[#] == 1, First[#], RowBox[#]] &, pieces]];
SL31Run`statements[b_] := If[SL31Run`commentQ[b] || SL31Run`blankQ[b], {}, {b}];
(* every statement is read separately; if one of them cannot be read, the whole cell is reported
   as not readable *)
SL31Run`readStatement[b_] := ToExpression[b, StandardForm, HoldComplete];
SL31Run`toHeld[BoxData[b_List]] := Module[{parts = Map[SL31Run`toHeld[BoxData[#]] &, b]},
   If[MemberQ[parts, $Failed], $Failed, Join @@ Prepend[parts, HoldComplete[]]]];
SL31Run`toHeld[BoxData[b_]] := Module[{parts = Map[SL31Run`readStatement, SL31Run`statements[b]]},
   If[! AllTrue[parts, MatchQ[#, _HoldComplete] &], $Failed, Join @@ Prepend[parts, HoldComplete[]]]];
SL31Run`toHeld[s_String] := ToExpression[s, InputForm, HoldComplete];
SL31Run`toHeld[other_] := $Failed;

(* an output is shown unless it is Null (for example, the input ended with a semicolon) *)
SL31Run`showQ[res_] := res =!= Null;

SL31Run`cellLog = {};
(* a progress file, written before every cell, shows where a long or stuck evaluation is *)
progressFile = StringDrop[logFile, -4] <> ".progress.txt";
SL31Run`progress[line_String] := Module[{st = OpenAppend[progressFile]}, WriteString[st, line <> "\n"]; Close[st]];
Quiet[DeleteFile[progressFile]];
(* the longest time one cell may take, in seconds, before it is stopped *)
cellTimeLimit = 1800;
t0 = AbsoluteTime[];
summary = UsingFrontEnd[
  Module[{nb, cells, n = 0, c, cell, style, content, held, res, prints, msgs, t, newCells, label},
    nb = NotebookOpen[inNB, Visible -> False];
    If[Head[nb] =!= NotebookObject, Return["could not open " <> inNB, Module]];
    cells = Cells[nb];
    Do[
      cell = NotebookRead[c];
      style = If[Length[cell] >= 2, cell[[2]], ""];
      If[MemberQ[{"Input", "Code"}, style],
        n++;
        content = cell[[1]];
        SetOptions[c, CellLabel -> "In[" <> ToString[n] <> "]:="];
        SL31Run`cellMessages = {};
        prints = {};
        SL31Run`progress[DateString[{"Hour", ":", "Minute", ":", "Second"}] <> "  cell " <> ToString[n] <> "  " <>
          StringTake[StringReplace[ToString[content, InputForm], "\n" -> " "], UpTo[100]]];
        held = Check[SL31Run`toHeld[content], $Failed];
        (* a cell may hold several expressions on separate lines: evaluate them in turn, as the
           front end does, and show the result of every one that is not Null *)
        {t, res} = If[held === $Failed,
          {0., {$Failed}},
          AbsoluteTiming[
            Block[{Print = (AppendTo[prints, {##}]; Null) &, $Line = n},
              TimeConstrained[
                List @@ Map[Function[e, ReleaseHold[e], HoldAllComplete], Map[HoldComplete, held]],
                cellTimeLimit, {"stopped after " <> ToString[cellTimeLimit] <> " seconds"}]]]];
        msgs = SL31Run`cellMessages;
        newCells = Join[
          Map[Cell[BoxData[ToBoxes[Row[#], StandardForm]], "Print"] &, prints],
          Map[Cell[SL31Run`messageText[#], "Message", "MSG"] &, Select[msgs, Last[#] === True &]],
          Map[Cell[BoxData[ToBoxes[#, StandardForm]], "Output", CellLabel -> "Out[" <> ToString[n] <> "]="] &,
            Select[res, SL31Run`showQ]]];
        If[newCells =!= {},
          SelectionMove[c, After, Cell];
          NotebookWrite[nb, newCells]];
        AppendTo[SL31Run`cellLog, <|"cell" -> n, "seconds" -> t,
          "messages" -> Map[SL31Run`messageText, msgs],
          "parsed" -> (held =!= $Failed)|>];
      ],
      {c, cells}];
    NotebookSave[nb, outNB];
    Export[StringDrop[outNB, -3] <> ".pdf", nb];
    NotebookClose[nb];
    <|"inputCells" -> n|>
  ]
];
seconds = AbsoluteTime[] - t0;
testSym = Quiet[ToExpression["SL31`Test`$Results", InputForm, HoldComplete]];
tests = If[MatchQ[testSym, HoldComplete[_Symbol]] && ValueQ @@ testSym, ReleaseHold[testSym], Missing["no test registry"]];
printedMessages = Select[SL31Run`allMessages, Last[#] === True &];
logText = StringRiffle[Flatten[{
   "run_notebook.wls log",
   "input:   " <> inNB, "output:  " <> outNB,
   "version: " <> $Version,
   "date:    " <> DateString[],
   "seconds: " <> ToString[NumberForm[seconds, {8, 1}]],
   "summary: " <> ToString[summary, InputForm],
   "printed messages: " <> ToString[Length[printedMessages]] <> " (all messages: " <> ToString[Length[SL31Run`allMessages]] <> ")",
   "--- printed messages ---",
   Map[SL31Run`messageText, printedMessages],
   "--- cells (number, seconds, messages) ---",
   Map[ToString[#["cell"]] <> "  " <> ToString[NumberForm[#["seconds"], {6, 3}]] <> "  " <>
        If[#["parsed"], "", "NOT PARSED  "] <> StringRiffle[#["messages"], " | "] &, SL31Run`cellLog],
   "--- test registry ---",
   If[ListQ[tests], Map[ToString[#, InputForm] &, tests], ToString[tests]]
  }], "\n"];
Export[logFile, logText, "Text"];
Exit[0];
