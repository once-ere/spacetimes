(* ::Package:: *)
(* handler.wl (version 4) -- read again before every request by watcher.wl, so that it can be
   changed while the watcher runs.  A request file request-<id>.txt holds either
     three lines: input notebook, output notebook, log file   (run_notebook.wls is run), or
     one line:    wls <script.wls>|<arg>|<arg> ...            (the script is run with its arguments
                                                               in a fresh kernel by wolframscript), or
     one line:    gui <file.wl>                               (the file is read and evaluated HERE,
                                                               in the kernel of the running Wolfram.app
                                                               session, so that it can use the front
                                                               end of Wolfram.app itself).
   Every path must lie inside /Users/nsh/Developer/github/spacetimes.  done-<id>.txt receives the
   exit code or result, the time and the output. *)
SL31Watcher`root = "/Users/nsh/Developer/github/spacetimes";
SL31Watcher`dir = FileNameJoin[{SL31Watcher`root, ".git-transfer", "mma"}];
SL31Watcher`ws = Replace[FileNames["wolframscript", FileNameJoin[{$InstallationDirectory, "MacOS"}]], {{f_, ___} :> f, {} -> "wolframscript"}];
SL31Watcher`okPath[p_String] := StringStartsQ[p, SL31Watcher`root <> "/"] && ! StringContainsQ[p, ".."];
SL31Watcher`handle[req_String] := Module[{id, lines, cmd, r, t0, taken, done},
  id = StringReplace[FileBaseName[req], "request-" -> ""];
  done = FileNameJoin[{SL31Watcher`dir, "done-" <> id <> ".txt"}];
  lines = Select[StringTrim /@ Import[req, "Lines"], # =!= "" &];
  taken = req <> ".taken";
  Quiet[DeleteFile[taken]];
  RenameFile[req, taken];
  (* a gui request: evaluate the file in this kernel and report result, messages and time *)
  If[Length[lines] >= 1 && StringStartsQ[lines[[1]], "gui "],
    With[{file = StringTrim[StringDrop[lines[[1]], 4]]},
      If[! SL31Watcher`okPath[file] || ! FileExistsQ[file],
        Export[done, "rejected: bad gui request " <> ToString[lines, InputForm], "Text"]; Return[Null, Module]];
      Module[{ed = EvaluationData[Get[file]]},
        Export[done, StringJoin["gui ", file,
          "\nsuccess ", ToString[ed["Success"]],
          "\nseconds ", ToString[Round[ed["AbsoluteTiming"], 0.01]],
          "\n--- messages ---\n", StringRiffle[ToString /@ ed["MessagesText"], "\n"],
          "\n--- result ---\n", StringTake[ToString[ed["Result"], InputForm, PageWidth -> Infinity], UpTo[20000]]], "Text"]]];
    Return[Null, Module]];
  cmd = Which[
    Length[lines] >= 1 && StringStartsQ[lines[[1]], "wls "],
      With[{w = StringSplit[StringDrop[lines[[1]], 4], "|"]}, If[AllTrue[w, SL31Watcher`okPath], Join[{SL31Watcher`ws, "-file"}, w], $Failed]],
    Length[lines] >= 3 && AllTrue[lines[[1 ;; 3]], SL31Watcher`okPath],
      {SL31Watcher`ws, "-file", FileNameJoin[{SL31Watcher`dir, "run_notebook.wls"}], lines[[1]], lines[[2]], lines[[3]]},
    True, $Failed];
  If[cmd === $Failed,
    Export[done, "rejected: bad request " <> ToString[lines, InputForm], "Text"]; Return[Null, Module]];
  (* start the process, read its output while it runs (so that a full pipe cannot stop it), and stop
     it after the time limit *)
  t0 = AbsoluteTime[];
  Module[{p = StartProcess[cmd], out = "", err = "", limit = 7200, status = "finished"},
    While[ProcessStatus[p] === "Running",
      out = out <> Replace[ReadString[ProcessConnection[p, "StandardOutput"], EndOfBuffer], Except[_String] -> ""];
      err = err <> Replace[ReadString[ProcessConnection[p, "StandardError"], EndOfBuffer], Except[_String] -> ""];
      If[AbsoluteTime[] - t0 > limit, KillProcess[p]; status = "killed after " <> ToString[limit] <> " s"; Break[]];
      (* a file named kill in the folder stops the running process *)
      If[FileExistsQ[FileNameJoin[{SL31Watcher`dir, "kill"}]], KillProcess[p]; status = "killed on request";
        RenameFile[FileNameJoin[{SL31Watcher`dir, "kill"}], FileNameJoin[{SL31Watcher`dir, "kill-" <> id <> ".done"}]]; Break[]];
      Pause[0.5]];
    out = out <> Replace[Quiet[ReadString[ProcessConnection[p, "StandardOutput"], EndOfBuffer]], Except[_String] -> ""];
    err = err <> Replace[Quiet[ReadString[ProcessConnection[p, "StandardError"], EndOfBuffer]], Except[_String] -> ""];
    Export[done, StringJoin["exit ", ToString[ProcessInformation[p, "ExitCode"]], "\nstatus ", status,
       "\nseconds ", ToString[Round[AbsoluteTime[] - t0, 0.1]],
       "\ncommand ", StringRiffle[cmd, " "], "\n--- stdout ---\n", out, "\n--- stderr ---\n", err], "Text"]];
];
