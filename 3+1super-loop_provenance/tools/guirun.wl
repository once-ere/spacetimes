(* ::Package:: *)
(* guirun.wl  (PROJECT spacetimes, notebook 3+1super-loop.nb)
   Evaluates a notebook in the front end of the running Wolfram.app, exactly as the menu command
   Evaluation > Evaluate Notebook does, but in a SEPARATE, FRESH kernel named "SL31Run".  The work is
   done in three calls, each of which returns at once:
     SL31GUI`start[in.nb, tag]  opens in.nb (visible), sets its evaluator to SL31Run, and starts the
                                kernel by evaluating a hidden one-cell notebook in it.  That cell
                                (1) sets $Post in the kernel, so that the kernel appends, after every
                                input line, the number of the line and the time to
                                out/<tag>.progress.txt ($Post returns every result unchanged);
                                (2) writes out/<tag>.ready.txt, the sign that the kernel runs and
                                answers; (3) sets $Line to 0, so that the first cell of in.nb is In[1].
                                Nothing else is queued in this call: in the run of 2026-11-18, 12:19
                                (Pacific time), in which the evaluations of two notebooks were queued
                                in the same call that started the kernel, the front end stopped
                                responding and the kernel never received an input;
     SL31GUI`go[]               when out/<tag>.ready.txt exists: starts the evaluation of all cells of
                                in.nb and queues after it a hidden one-cell notebook that writes the
                                test registry of the kernel to out/<tag>.results.wl and then quits the
                                kernel (so that the next run starts again with a fresh kernel);
     SL31GUI`status[]           while the results file does not exist: reports the last entry of the
                                progress file, WITHOUT any call to the front end (the front end is busy
                                with the evaluation); once the results file exists it saves the
                                evaluated notebook to out/<tag>.nb, exports out/<tag>.pdf, writes
                                out/<tag>.log.txt (all printed lines and messages) and reports.
   The progress file and the results file are ordinary text files, so a run can also be followed
   without any call into Wolfram.app.
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

SL31GUI`start[inNB_String, tag_String] := Module[{nb, resultFile, outNB, progressFile, readyFile, pre},
   SL31GUI`ensureKernel[];
   resultFile = FileNameJoin[{SL31GUI`dir, "out", tag <> ".results.wl"}];
   outNB = FileNameJoin[{SL31GUI`dir, "out", tag <> ".nb"}];
   progressFile = FileNameJoin[{SL31GUI`dir, "out", tag <> ".progress.txt"}];
   readyFile = FileNameJoin[{SL31GUI`dir, "out", tag <> ".ready.txt"}];
   Scan[Quiet[DeleteFile[#]] &, {resultFile, progressFile, readyFile}];
   (* close windows left open by an earlier run of this notebook *)
   Scan[NotebookClose, Select[Notebooks[], MemberQ[{inNB, outNB}, Quiet[NotebookFileName[#]]] &]];
   nb = NotebookOpen[inNB];
   SetOptions[nb, Evaluator -> SL31GUI`kernel];
   pre = With[{pf = progressFile, rf = readyFile},
     NotebookPut[Notebook[{Cell[BoxData[MakeBoxes[
         $Post = Function[result, (PutAppend[{$Line, DateString[{"Hour", ":", "Minute", ":", "Second"}]}, pf]; result)];
         Put[<|"ready" -> DateString[], "kernel" -> {$KernelID, $ProcessID}|>, rf];
         $Line = 0;, StandardForm]], "Input"]},
       Visible -> False, Evaluator -> SL31GUI`kernel]]];
   SL31GUI`job = <|"in" -> inNB, "tag" -> tag, "nb" -> nb, "pre" -> pre, "results" -> resultFile,
      "progress" -> progressFile, "ready" -> readyFile,
      "out" -> outNB, "pdf" -> FileNameJoin[{SL31GUI`dir, "out", tag <> ".pdf"}],
      "log" -> FileNameJoin[{SL31GUI`dir, "out", tag <> ".log.txt"}],
      "started" -> AbsoluteTime[], "state" -> "kernel starting"|>;
   FrontEndTokenExecute[pre, "EvaluateNotebook"];
   {"kernel starting", tag}];

SL31GUI`go[] := Module[{j = SL31GUI`job, marker},
   If[! AssociationQ[j], Return["no job", Module]];
   If[j["state"] =!= "kernel starting", Return[{"not waiting for the kernel", j["state"]}, Module]];
   If[! FileExistsQ[j["ready"]], Return["kernel not ready", Module]];
   marker = With[{rf = j["results"]},
     NotebookPut[Notebook[{Cell[BoxData[MakeBoxes[
         Put[<|"finished" -> DateString[], "kernel" -> {$KernelID, $ProcessID},
              "tests" -> If[ValueQ[SL31`Test`$Results], SL31`Test`$Results, {}],
              "line" -> $Line, "seconds" -> SessionTime[]|>, rf];
         Quit[], StandardForm]], "Input"]}, Visible -> False, Evaluator -> SL31GUI`kernel]]];
   SL31GUI`job["marker"] = marker;
   SL31GUI`job["state"] = "running";
   SL31GUI`job["started"] = AbsoluteTime[];
   FrontEndTokenExecute[j["nb"], "EvaluateNotebook"];
   FrontEndTokenExecute[marker, "EvaluateNotebook"];
   {"started", j["tag"]}];

(* the last entry {line, time} of the progress file, read without the front end *)
SL31GUI`lastProgress[file_String] := If[FileExistsQ[file],
   With[{l = Quiet[ReadList[file]]}, If[ListQ[l] && l =!= {}, Last[l], "no entry"]], "no file"];

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
     Return[<|"state" -> j["state"], "seconds" -> Round[AbsoluteTime[] - j["started"]],
        "kernelReady" -> FileExistsQ[j["ready"]], "lastLine" -> SL31GUI`lastProgress[j["progress"]]|>, Module]];
   (* finished: save, export, log *)
   res = Get[j["results"]];
   SetOptions[nb, Evaluator -> Inherited];
   NotebookSave[nb, j["out"]];
   Export[j["pdf"], nb];
   Quiet[NotebookClose[j["marker"]]];
   Quiet[NotebookClose[j["pre"]]];
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
