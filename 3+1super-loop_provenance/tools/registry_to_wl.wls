#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   registry_to_wl.wls     (PROJECT spacetimes, notebook 3+1super-loop.nb)
   Converts the registry of the symbols of the super-loop revision (a CSV file with the columns
   group, symbol, names_in_files, meaning, origin) into a Wolfram Language data file, applying the
   restated meanings of meaning_edits.wl, so that the notebook can hold the complete registry.
   Usage:  wolframscript -file registry_to_wl.wls  <symbols_ll.csv>  <meaning_edits.wl>  <out.wl>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
{csv, edits, out} = args[[1 ;; 3]];
Get[edits];
rows = Import[csv, "CSV"];
start = First[FirstPosition[rows, {"group", "symbol", "names_in_files", "meaning", "origin"}]];
rows = Select[Drop[rows, start], Length[#] == 5 &];
(* an empty field of the CSV file is imported as Missing["NotAvailable"] (and a field that looks
   like a number as a number): every field is turned into a string, an empty field into "" *)
field[x_String] := x;
field[_Missing] := "";
field[x_] := ToString[x];
data = MapIndexed[
   Function[{r, k}, <|"entry" -> First[k], "group" -> field[r[[1]]], "symbol" -> field[r[[2]]],
      "names" -> StringSplit[field[r[[3]]]],
      "meaning" -> Lookup[SL31MeaningEdits, First[k], field[r[[4]]]],
      "meaningEdited" -> KeyExistsQ[SL31MeaningEdits, First[k]],
      "origin" -> field[r[[5]]]|>], rows];
text = "(* The registry of the symbols of the super-loop revision: " <> ToString[Length[data]] <>
   " entries (group, TeX of the symbols, plain-text names, meaning, origin). *)\nSL31Registry = {\n" <>
   StringRiffle[Map[ToString[#, InputForm, CharacterEncoding -> "ASCII"] &, data], ",\n"] <> "\n};\n";
Export[out, text, "Text"];
Print[Length[data], " entries written to ", out];
