(* ::Package:: *)
(* guirun.wl  (PROJECT spacetimes, folder Revision_HS_CJT_and_Kohn-Sham, notebook superfluid.nb; first written
   for the notebook 3+1super-loop.nb of the project and copied here with the names changed from SL31 to SFNB)
   Evaluates a notebook in the front end of the running Wolfram.app, exactly as the menu command
   Evaluation > Evaluate Notebook does, but in a SEPARATE, FRESH kernel named "SFNBRun".  The work is
   done in three calls, each of which returns at once:
     SFNBGUI`start[in.nb, tag]  opens in.nb (visible), sets its evaluator to SFNBRun, and starts the
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
     SFNBGUI`go[]               when out/<tag>.ready.txt exists: starts the evaluation of all cells of
                                in.nb and queues after it a hidden one-cell notebook that writes the
                                test registry of the kernel to out/<tag>.results.wl and then quits the
                                kernel (so that the next run starts again with a fresh kernel);
     SFNBGUI`status[]           while the results file does not exist: reports the last entry of the
                                progress file, WITHOUT any call to the front end (the front end is busy
                                with the evaluation); once the results file exists it saves the
                                evaluated notebook to out/<tag>.nb, exports out/<tag>.pdf, writes
                                out/<tag>.log.txt (all printed lines and messages) and reports.
   The progress file and the results file are ordinary text files, so a run can also be followed
   without any call into Wolfram.app.
   The kernel SFNBRun is declared for this front-end session only ($FrontEndSession), so the
   preferences of Wolfram.app are not changed. *)
SFNBGUI`dir = "/Users/nsh/Developer/github/spacetimes/.git-transfer/mma/sfnb";
SFNBGUI`kernel = "SFNBRun";

SFNBGUI`ensureKernel[] := Module[{ev = EvaluatorNames /. Options[$FrontEndSession, EvaluatorNames]},
   If[! MemberQ[Keys[ev], SFNBGUI`kernel],
     SetOptions[$FrontEndSession, EvaluatorNames -> Append[ev, SFNBGUI`kernel -> {"AutoStartOnLaunch" -> False}]]];
   Keys[EvaluatorNames /. Options[$FrontEndSession, EvaluatorNames]]];

SFNBGUI`cellText[c_CellObject] := Quiet[Check[
    First[FrontEndExecute[FrontEnd`ExportPacket[NotebookRead[c], "PlainText"]]], ToString[NotebookRead[c], InputForm]]];

SFNBGUI`label[c_CellObject] := Replace[CellLabel /. Options[c, CellLabel], {l_String :> StringReplace[l, {"During evaluation of " -> "during ", "\n" -> "", "\\" -> ""}], _ -> "(no label)"}];

SFNBGUI`start[inNB_String, tag_String] := Module[{nb, resultFile, outNB, progressFile, readyFile, pre},
   SFNBGUI`ensureKernel[];
   resultFile = FileNameJoin[{SFNBGUI`dir, "out", tag <> ".results.wl"}];
   outNB = FileNameJoin[{SFNBGUI`dir, "out", tag <> ".nb"}];
   progressFile = FileNameJoin[{SFNBGUI`dir, "out", tag <> ".progress.txt"}];
   readyFile = FileNameJoin[{SFNBGUI`dir, "out", tag <> ".ready.txt"}];
   Scan[Quiet[DeleteFile[#]] &, {resultFile, progressFile, readyFile}];
   (* close windows left open by an earlier run of this notebook *)
   Scan[NotebookClose, Select[Notebooks[], MemberQ[{inNB, outNB}, Quiet[NotebookFileName[#]]] &]];
   nb = NotebookOpen[inNB];
   SetOptions[nb, Evaluator -> SFNBGUI`kernel];
   pre = With[{pf = progressFile, rf = readyFile},
     NotebookPut[Notebook[{Cell[BoxData[MakeBoxes[
         $Post = Function[result, (PutAppend[{$Line, DateString[{"Hour", ":", "Minute", ":", "Second"}]}, pf]; result)];
         Put[<|"ready" -> DateString[], "kernel" -> {$KernelID, $ProcessID}|>, rf];
         $Line = 0;, StandardForm]], "Input"]},
       Visible -> False, Evaluator -> SFNBGUI`kernel]]];
   SFNBGUI`job = <|"in" -> inNB, "tag" -> tag, "nb" -> nb, "pre" -> pre, "results" -> resultFile,
      "progress" -> progressFile, "ready" -> readyFile,
      "out" -> outNB, "pdf" -> FileNameJoin[{SFNBGUI`dir, "out", tag <> ".pdf"}],
      "log" -> FileNameJoin[{SFNBGUI`dir, "out", tag <> ".log.txt"}],
      "started" -> AbsoluteTime[], "state" -> "kernel starting"|>;
   FrontEndTokenExecute[pre, "EvaluateNotebook"];
   {"kernel starting", tag}];

SFNBGUI`go[] := Module[{j = SFNBGUI`job, marker},
   If[! AssociationQ[j], Return["no job", Module]];
   If[j["state"] =!= "kernel starting", Return[{"not waiting for the kernel", j["state"]}, Module]];
   If[! FileExistsQ[j["ready"]], Return["kernel not ready", Module]];
   marker = With[{rf = j["results"]},
     NotebookPut[Notebook[{Cell[BoxData[MakeBoxes[
         Put[<|"finished" -> DateString[], "kernel" -> {$KernelID, $ProcessID},
              "tests" -> If[ValueQ[SFNB`Test`$Results], SFNB`Test`$Results, {}],
              "line" -> $Line, "seconds" -> SessionTime[]|>, rf];
         Quit[], StandardForm]], "Input"]}, Visible -> False, Evaluator -> SFNBGUI`kernel]]];
   SFNBGUI`job["marker"] = marker;
   SFNBGUI`job["state"] = "running";
   SFNBGUI`job["started"] = AbsoluteTime[];
   FrontEndTokenExecute[j["nb"], "EvaluateNotebook"];
   FrontEndTokenExecute[marker, "EvaluateNotebook"];
   {"started", j["tag"]}];

(* the last entry {line, time} of the progress file, read without the front end *)
SFNBGUI`lastProgress[file_String] := If[FileExistsQ[file],
   With[{l = Quiet[ReadList[file]]}, If[ListQ[l] && l =!= {}, Last[l], "no entry"]], "no file"];

SFNBGUI`counts[nb_] := <|
   "inputs" -> Length[Cells[nb, CellStyle -> {"Input", "Code"}]],
   "evaluated" -> Count[Cells[nb, CellStyle -> {"Input", "Code"}], c_ /; StringQ[CellLabel /. Options[c, CellLabel]] && StringStartsQ[CellLabel /. Options[c, CellLabel], "In["]],
   "outputs" -> Length[Cells[nb, CellStyle -> "Output"]],
   "prints" -> Length[Cells[nb, CellStyle -> "Print"]],
   "messages" -> Length[Cells[nb, CellStyle -> "Message"]]|>;

SFNBGUI`status[] := Module[{j = SFNBGUI`job, nb, res, msgs, prints, inputs, log},
   If[! AssociationQ[j], Return["no job", Module]];
   nb = j["nb"];
   If[j["state"] === "finished", Return[j["summary"], Module]];
   If[! FileExistsQ[j["results"]],
     Return[<|"state" -> j["state"], "seconds" -> Round[AbsoluteTime[] - j["started"]],
        "kernelReady" -> FileExistsQ[j["ready"]], "lastLine" -> SFNBGUI`lastProgress[j["progress"]]|>, Module]];
   (* finished: save, export, log *)
   res = Get[j["results"]];
   SetOptions[nb, Evaluator -> Inherited];
   NotebookSave[nb, j["out"]];
   Export[j["pdf"], nb];
   Quiet[NotebookClose[j["marker"]]];
   Quiet[NotebookClose[j["pre"]]];
   (* every message and printed line, with the label of the input that produced it *)
   msgs = Map[SFNBGUI`label[#] <> "  " <> SFNBGUI`cellText[#] &, Cells[nb, CellStyle -> "Message"]];
   prints = Map[SFNBGUI`label[#] <> "  " <> SFNBGUI`cellText[#] &, Cells[nb, CellStyle -> "Print"]];
   inputs = Map[SFNBGUI`label[#] <> "  " <> StringTake[StringReplace[SFNBGUI`cellText[#], "\n" -> " "], UpTo[90]] &,
     Cells[nb, CellStyle -> {"Input", "Code"}]];
   log = StringRiffle[Flatten[{
      "evaluated in the front end of " <> $Version, "notebook: " <> j["in"], "saved:    " <> j["out"],
      "seconds:  " <> ToString[Round[AbsoluteTime[] - j["started"]]],
      "kernel:   " <> ToString[res["kernel"]] <> "   last In line: " <> ToString[res["line"]],
      "counts:   " <> ToString[SFNBGUI`counts[nb], InputForm],
      "--- messages (" <> ToString[Length[msgs]] <> ") ---", msgs,
      "--- printed lines (" <> ToString[Length[prints]] <> ") ---", prints,
      "--- input cells (label, beginning) ---", inputs,
      "--- tests ---", Map[ToString[#, InputForm] &, res["tests"]]}], "\n"];
   Export[j["log"], log, "Text"];
   SFNBGUI`job["state"] = "finished";
   SFNBGUI`job["summary"] = Join[<|"state" -> "finished", "seconds" -> Round[AbsoluteTime[] - j["started"]],
      "tests" -> Length[res["tests"]], "passed" -> Count[res["tests"], t_ /; TrueQ[t["pass"]]],
      "messageTexts" -> msgs|>, SFNBGUI`counts[nb]];
   SFNBGUI`job["summary"]];
