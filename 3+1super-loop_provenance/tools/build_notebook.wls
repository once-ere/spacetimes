#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   build_notebook.wls      (PROJECT spacetimes, notebook 3+1super-loop.nb)

   PURPOSE
   Builds a notebook (.nb) from a source text in the Wolfram package format, in which every cell
   is introduced by a marker line
        (* ::Title:: *)   (* ::Section:: *)   (* ::Text:: *)   (* ::Input:: *)   ...
   followed, for text-like styles, by the text inside one comment (* ... *), and, for Input, by
   Wolfram Language code.  The builder
     - turns every Input chunk into a two-dimensional input cell: the code is parsed into boxes
       by the front end itself, which keeps comments, line breaks and indentation;
     - replaces every reference LL["tex"] or LL["tex", "index" -> "value", ...] in the code by
       the two-dimensional box of that registered symbol (or of its component), so that the
       notebook shows the symbols as they are printed in the documents;
     - turns $...$ in text into inline formulas (TeX converted by the Wolfram TeX importer),
       `...` (or ``...`` for code that contains the character `) into code font and **...** into
       bold;
     - inserts the content of the file named by a line  (* @@INCLUDE file@@ *)  (relative to
       the source) into the code chunk in which it stands, so that the notebook is complete.
   USAGE
     wolframscript -file build_notebook.wls  <source.wl>  <output.nb>  [<boxes-definitions.wl>]
   The optional third file defines SL31`Build`symbolBoxes[tex, rules], used for LL[...].
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 2, Print["usage: build_notebook.wls source.wl out.nb [boxes.wl]"]; Exit[2]];
{src, outNB} = args[[1 ;; 2]];
srcDir = DirectoryName[src];
(* the symbol machinery (it loads the Notation package, which asks the front end for settings) *)
If[Length[args] >= 3, UsingFrontEnd[Get[args[[3]]]]];
boxesDefinedQ = Length[DownValues[SL31`Build`symbolBoxes]] > 0;

(* ---- 1. read the source and resolve the includes ---- *)
text = ReadString[src];
(* an included file is inserted without its own cell markers (lines such as (* ::Package:: *)) *)
text = StringReplace[text,
   RegularExpression["\\(\\* @@INCLUDE ([^@]+)@@ \\*\\)"] :>
     StringReplace[ReadString[FileNameJoin[{srcDir, "$1"}]], RegularExpression["(?m)^\\(\\* ::\\w+:: \\*\\)[ \\t]*\\n"] -> ""]];

(* ---- 2. split into (style, body) chunks ---- *)
chunks = StringCases[text,
   RegularExpression["(?ms)^\\(\\* ::(\\w+)(?:::\\w+)?:: \\*\\)[ \\t]*\\n(.*?)(?=^\\(\\* ::\\w+(?:::\\w+)?:: \\*\\)|\\z)"] :> {"$1", "$2"}];

textStyles = {"Title", "Subtitle", "Subsubtitle", "Chapter", "Section", "Subsection", "Subsubsection",
   "Text", "Item", "Subitem", "ItemNumbered", "SubitemNumbered", "ItemParagraph", "Program"};

(* the text of a comment chunk: strip one (* ... *) pair *)
commentText[body_String] := StringTrim[StringReplace[StringTrim[body],
    RegularExpression["(?s)^\\(\\*(.*)\\*\\)$"] :> "$1"]];

(* inline TeX -> an InlineFormula cell (Wolfram TeX importer); on failure keep the TeX *)
SL31Build`inline[tex_String] := Module[{nb = Quiet[ImportString["$" <> tex <> "$", "TeX"]], c},
   c = Cases[nb, Cell[BoxData[f_FormBox], "InlineFormula", ___], Infinity];
   If[c === {}, AppendTo[SL31Build`warnings, "TeX not converted: " <> tex]; "$" <> tex <> "$",
     Cell[BoxData[First[c]], "InlineFormula"]]];
SL31Build`warnings = {};

(* text with $math$, `code` and **bold** -> TextData; StringReplace returns a StringExpression
   of strings and boxes when a replacement is not a string *)
textData[t_String] := Module[{r},
   r = StringReplace[t, {
      RegularExpression["\\$([^$]+)\\$"] :> SL31Build`inline["$1"],
      (* ``...`` marks code that itself contains the character ` (a context mark) *)
      RegularExpression["``(.+?)``"] :> StyleBox["$1", FontFamily -> "Source Code Pro"],
      RegularExpression["`([^`]+)`"] :> StyleBox["$1", FontFamily -> "Source Code Pro"],
      RegularExpression["\\*\\*([^*]+)\\*\\*"] :> StyleBox["$1", FontWeight -> "Bold"]}];
   TextData[If[Head[r] === StringExpression, List @@ r, {r}]]];

(* display formula: the whole body is TeX *)
displayCell[t_String] := Module[{nb = Quiet[ImportString["$$" <> t <> "$$", "TeX"]], c},
   c = Cases[nb, Cell[BoxData[f_FormBox], ___], Infinity];
   If[c === {}, AppendTo[SL31Build`warnings, "display TeX not converted: " <> t];
     Cell["$$" <> t <> "$$", "Text"],
     Cell[BoxData[First[c]], "DisplayFormula"]]];

(* code: replace LL["..."] references by placeholders, parse with the front end, substitute boxes *)
SL31Build`placeholders = <||>;
codeCell[code_String] := Module[{k = 0, rep, parsed, boxes},
   rep = StringReplace[code,
     RegularExpression["LL\\[(\"(?:[^\"\\\\]|\\\\.)*\"(?:\\s*,\\s*\"(?:[^\"\\\\]|\\\\.)*\"\\s*->\\s*\"(?:[^\"\\\\]|\\\\.)*\")*)\\]"] :>
       Module[{id = "SLPH" <> ToString[++k] <> "X", argsExpr = ToExpression["{" <> "$1" <> "}"]},
         SL31Build`placeholders[id] = If[boxesDefinedQ,
           SL31`Build`symbolBoxes @@ argsExpr, $Failed];
         If[SL31Build`placeholders[id] === $Failed,
           AppendTo[SL31Build`warnings, "no boxes for LL[" <> "$1" <> "]"]; "LL[" <> "$1" <> "]", id]]];
   parsed = FrontEndExecute[FrontEnd`UndocumentedTestFEParserPacket[rep, False]];
   boxes = First[parsed];
   boxes = boxes /. s_String /; KeyExistsQ[SL31Build`placeholders, s] :> SL31Build`placeholders[s];
   Cell[boxes, "Input"]];

cellOf[{"Package", _}] := Nothing;
cellOf[{style_, body_}] := Which[
   style === "Input" || style === "Code", codeCell[StringTrim[body, "\n" ..]],
   style === "DisplayFormula", displayCell[commentText[body]],
   MemberQ[textStyles, style], Cell[textData[commentText[body]], style],
   True, (AppendTo[SL31Build`warnings, "unknown style " <> style]; Cell[commentText[body], "Text"])];

cells = UsingFrontEnd[cellOf /@ chunks];
nb = Notebook[cells, StyleDefinitions -> "Default.nb", WindowSize -> {1000, 900}];
(* the notebook expression is written by the kernel itself, NOT through the front end: when the
   front end writes a notebook file it reads the boxes of every input cell again, and a box that
   looks like an operator (a partial-derivative sign with a subscript, for example) makes it regroup
   the whole line, so that the typed symbols would no longer be the symbolized boxes *)
Export[outNB, nb, "NB"];
Print["cells: ", Length[cells]];
Print["warnings: ", Length[SL31Build`warnings]];
Scan[Print, SL31Build`warnings];
Exit[0];
