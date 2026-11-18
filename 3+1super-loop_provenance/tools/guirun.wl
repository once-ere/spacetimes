(* ::Package:: *)
(* guirun.wl  (PROJECT spacetimes, notebook 3+1super-loop.nb)
   Evaluates a notebook in the front end of the running Wolfram.app, exactly as the menu command
   Evaluation > Evaluate Notebook does, but in a SEPARATE, FRESH kernel named "SL31Run":
     SL31GUI`start[in.nb, tag]  opens in.nb (visible), sets its evaluator to SL31Run, starts the
                                evaluation of all its cells, and queues after it a hidden one-cell
                                notebook that writes the test registry of the kernel to
                                out/<tag>.results.wl and then quits the kernel (so that the next
                                run starts again with a fresh kernel);
     SL31GUI`status[]           reports the progress; once the results file exists it saves the
                                evaluated notebook to out/<tag>.nb, exports out/<tag>.pdf, writes
                                out/<tag>.log.txt (all printed lines and messages) and reports.
   The kernel SL31Run is declared for this front-end session only ($FrontEndSession), so the
   preferences of Wolfram.app are not changed. *)
SL31GUI`dir = "/Users/nsh/Developer/github/spacetimes/.git-transfer/mma";
SL31GUI`kernel = "SL31Run";

SL31GUI`ensureKernel[] := Module[{ev = EvaluatorNames /. Options[$FrontEndSession, EvaluatorNames]},
   If[! MemberQ[Keys[ev], SL31GUI`kernel],
     SetOptions[$FrontEndSession, EvaluatorNames -> Append[ev, SL31GUI`kernel -> {"AutoStartOnLaunch" -> False}]]];
   Keys[EvaluatorNames /. Options[$FrontEndSession, EvaluatorNames]]];

SL31GUI`cellText[c_CellObject] := Quiet[Check[
    First[FrontEndExecute[FrontEnd`ExportPacket[NotebookRead[c], "PlainText"]]], ToString[NotebookRead[c], InputForm]]];

SL31GUI`label[c_CellObject] := Replace[CellLabel /. Options[c, CellLabel], {l_String :> StringReplace[l, {"During evaluation of " -> "during ", "\n" -> "", "\\" -> ""}], _ -> "(no label)"}];

SL31GUI`start[inNB_String, tag_String] := Module[{nb, marker, resultFile, outNB},
   SL31GUI`ensureKernel[];
   resultFile = FileNameJoin[{SL31GUI`dir, "out", tag <> ".results.wl"}];
   outNB = FileNameJoin[{SL31GUI`dir, "out", tag <> ".nb"}];
   Quiet[DeleteFile[resultFile]];
   (* close windows left open by an earlier run of this notebook *)
   Scan[NotebookClose, Select[Notebooks[], MemberQ[{inNB, outNB}, Quiet[NotebookFileName[#]]] &]];
   nb = NotebookOpen[inNB];
   SetOptions[nb, Evaluator -> SL31GUI`kernel];
   marker = With[{rf = resultFile},
     NotebookPut[Notebook[{Cell[BoxData[MakeBoxes[
         Put[<|"finished" -> DateString[], "kernel" -> {$KernelID, $ProcessID},
              "tests" -> If[ValueQ[SL31`Test`$Results], SL31`Test`$Results, {}],
              "line" -> $Line, "seconds" -> SessionTime[]|>, rf];
         Quit[], StandardForm]], "Input"]}, Visible -> False, Evaluator -> SL31GUI`kernel]]];
   SL31GUI`job = <|"in" -> inNB, "tag" -> tag, "nb" -> nb, "marker" -> marker, "results" -> resultFile,
      "out" -> outNB, "pdf" -> FileNameJoin[{SL31GUI`dir, "out", tag <> ".pdf"}],
      "log" -> FileNameJoin[{SL31GUI`dir, "out", tag <> ".log.txt"}],
      "started" -> AbsoluteTime[], "state" -> "running"|>;
   FrontEndTokenExecute[nb, "EvaluateNotebook"];
   FrontEndTokenExecute[marker, "EvaluateNotebook"];
   {"started", tag, Length[Cells[nb, CellStyle -> {"Input", "Code"}]]}];

SL31GUI`counts[nb_] := <|
   "inputs" -> Length[Cells[nb, CellStyle -> {"Input", "Code"}]],
   "evaluated" -> Count[Cells[nb, CellStyle -> {"Input", "Code"}], c_ /; StringQ[CellLabel /. Options[c, CellLabel]] && StringStartsQ[CellLabel /. Options[c, CellLabel], "In["]],
   "outputs" -> Length[Cells[nb, CellStyle -> "Output"]],
   "prints" -> Length[Cells[nb, CellStyle -> "Print"]],
   "messages" -> Length[Cells[nb, CellStyle -> "Message"]]|>;

SL31GUI`status[] := Module[{j = SL31GUI`job, nb, res, msgs, prints, inputs, log},
   If[! AssociationQ[j], Return["no job", Module]];
   nb = j["nb"];
   If[j["state"] === "finished", Return[j["summary"], Module]];
   If[! FileExistsQ[j["results"]],
     Return[Join[<|"state" -> "running", "seconds" -> Round[AbsoluteTime[] - j["started"]]|>, SL31GUI`counts[nb]], Module]];
   (* finished: save, export, log *)
   res = Get[j["results"]];
   SetOptions[nb, Evaluator -> Inherited];
   NotebookSave[nb, j["out"]];
   Export[j["pdf"], nb];
   Quiet[NotebookClose[j["marker"]]];
   (* every message and printed line, with the label of the input that produced it *)
   msgs = Map[SL31GUI`label[#] <> "  " <> SL31GUI`cellText[#] &, Cells[nb, CellStyle -> "Message"]];
   prints = Map[SL31GUI`label[#] <> "  " <> SL31GUI`cellText[#] &, Cells[nb, CellStyle -> "Print"]];
   inputs = Map[SL31GUI`label[#] <> "  " <> StringTake[StringReplace[SL31GUI`cellText[#], "\n" -> " "], UpTo[90]] &,
     Cells[nb, CellStyle -> {"Input", "Code"}]];
   log = StringRiffle[Flatten[{
      "evaluated in the front end of " <> $Version, "notebook: " <> j["in"], "saved:    " <> j["out"],
      "seconds:  " <> ToString[Round[AbsoluteTime[] - j["started"]]],
      "kernel:   " <> ToString[res["kernel"]] <> "   last In line: " <> ToString[res["line"]],
      "counts:   " <> ToString[SL31GUI`counts[nb], InputForm],
      "--- messages (" <> ToString[Length[msgs]] <> ") ---", msgs,
      "--- printed lines (" <> ToString[Length[prints]] <> ") ---", prints,
      "--- input cells (label, beginning) ---", inputs,
      "--- tests ---", Map[ToString[#, InputForm] &, res["tests"]]}], "\n"];
   Export[j["log"], log, "Text"];
   SL31GUI`job["state"] = "finished";
   SL31GUI`job["summary"] = Join[<|"state" -> "finished", "seconds" -> Round[AbsoluteTime[] - j["started"]],
      "tests" -> Length[res["tests"]], "passed" -> Count[res["tests"], t_ /; TrueQ[t["pass"]]],
      "messageTexts" -> msgs|>, SL31GUI`counts[nb]];
   SL31GUI`job["summary"]];
