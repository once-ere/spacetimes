(* ::Package:: *)
(* watcher.wl  (PROJECT spacetimes, notebook 3+1super-loop.nb)
   Loaded once into the running Wolfram.app session: Get["<this file>"].  Every 3 seconds it looks
   in its own folder for request files request-<id>.txt and hands each one to SL31Watcher`handle,
   which it reads again from handler.wl before every request.  Stop it with
   TaskRemove[SL31Watcher`task]. *)
If[ValueQ[SL31Watcher`task], Quiet[TaskRemove[SL31Watcher`task]]];
SL31Watcher`folder = DirectoryName[$InputFileName];
SL31Watcher`task = SessionSubmit[ScheduledTask[
   Scan[(Get[FileNameJoin[{SL31Watcher`folder, "handler.wl"}]]; SL31Watcher`handle[#]) &,
     FileNames["request-*.txt", SL31Watcher`folder]],
   Quantity[3, "Seconds"]]];
Print["SL31 watcher (version 2) started: ", SL31Watcher`task];
