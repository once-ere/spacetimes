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
     - turns $...$ in text into inline formulas, `...` (or ``...`` for code that contains the
       character `) into code font and **...** into bold, with the notebook's own reader of
       formulas (SL31TextData of notebook_source/sl31_formula.wl; the TeX importer of the
       Wolfram Language is not used, because it reads a fraction with arguments without braces
       wrongly and loses the alphabets of letters); every problem that the reader reports for a
       formula (an unknown command, a missing argument, an unmatched brace) becomes a warning;
     - inserts the content of the file named by a line  (* @@INCLUDE file@@ *)  (relative to
       the source) into the code chunk in which it stands, so that the notebook is complete;
     - replaces a line  (* @@INCLUDECELLS file@@ *)  by a whole file with its own cell markers
       (the chapters of Part II);
     - makes an input cell that is marked  (* ::Input::Closed:: *)  a closed cell (cell option
       CellOpen -> False): the notebook evaluates a closed cell like every other cell, but shows
       only a thin cell bracket for it and leaves it out of a printed copy.  This is used for the
       cells that hold the numbers of the data files of the original programs: they keep the
       notebook complete without filling many pages with numbers.
   USAGE
     wolframscript -file build_notebook.wls  <source.wl>  <output.nb>  <package.wl> [<package.wl> ...]
   The packages (sl31_core.wl, then sl31_formula.wl) define SL31`Build`symbolBoxes[tex, rules],
   used for LL[...], and the reader of formulas, used for the text cells.
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 3, Print["usage: build_notebook.wls source.wl out.nb package.wl [package.wl ...]"]; Exit[2]];
{src, outNB} = args[[1 ;; 2]];
srcDir = DirectoryName[src];
(* the symbol machinery (it loads the Notation package, which asks the front end for settings) and
   the reader of formulas *)
Scan[UsingFrontEnd[Get[#]] &, Drop[args, 2]];
boxesDefinedQ = Length[DownValues[SL31`Build`symbolBoxes]] > 0;
If[Length[DownValues[SL31`SL31TextData]] == 0,
  Print["the packages do not define the reader of formulas (SL31TextData)"]; Exit[2]];

(* ---- 1. read the source and resolve the includes ---- *)
text = ReadString[src];
(* a line  (* @@INCLUDECELLS file@@ *)  is replaced by the whole file, with its own cell markers (used for
   the chapters of Part II, each kept in its own source file) *)
text = StringReplace[text,
   RegularExpression["(?m)^\\(\\* @@INCLUDECELLS ([^@]+)@@ \\*\\)[ \\t]*$"] :> ReadString[FileNameJoin[{srcDir, "$1"}]]];
(* an included file is inserted without its own cell markers (lines such as (* ::Package:: *)) *)
text = StringReplace[text,
   RegularExpression["\\(\\* @@INCLUDE ([^@]+)@@ \\*\\)"] :>
     StringReplace[ReadString[FileNameJoin[{srcDir, "$1"}]], RegularExpression["(?m)^\\(\\* ::\\w+:: \\*\\)[ \\t]*\\n"] -> ""]];

(* ---- 1b. the components of families typed in the source ----
   every reference LL["tex", "index" -> "value", ...] with at least one index value names a component of a
   registered family; the line  (* @@COMPONENTS@@ *)  is replaced by the list of all of them, as
   {"tex", "index" -> "value", ...}, so that a cell of the notebook can create them before they are read *)
componentRE = RegularExpression["LL\\[(\"(?:[^\"\\\\]|\\\\.)*\"(?:\\s*,\\s*\"(?:[^\"\\\\]|\\\\.)*\"\\s*->\\s*\"(?:[^\"\\\\]|\\\\.)*\")+)\\]"];
SL31Build`components = DeleteDuplicates[StringCases[text, componentRE :> "$1"]];
text = StringReplace[text, "(* @@COMPONENTS@@ *)" -> "{" <> StringRiffle[Map["{" <> # <> "}" &, SL31Build`components], ", "] <> "}"];
Print["components: ", Length[SL31Build`components]];

(* ---- 2. split into (style, body) chunks ---- *)
(* a chunk is {style, tag, body}; the tag is the second word of a marker such as (* ::Input::Closed:: *), or the
   empty string *)
chunks = StringCases[text,
   RegularExpression["(?ms)^\\(\\* ::(\\w+)(?:::(\\w+))?:: \\*\\)[ \\t]*\\n(.*?)(?=^\\(\\* ::\\w+(?:::\\w+)?:: \\*\\)|\\z)"] :> {"$1", "$2", "$3"}];

textStyles = {"Title", "Subtitle", "Subsubtitle", "Chapter", "Section", "Subsection", "Subsubsection",
   "Text", "Item", "Subitem", "ItemNumbered", "SubitemNumbered", "ItemParagraph", "Program"};

(* the text of a comment chunk: strip one (* ... *) pair *)
commentText[body_String] := StringTrim[StringReplace[StringTrim[body],
    RegularExpression["(?s)^\\(\\*(.*)\\*\\)$"] :> "$1"]];

(* text with $math$, `code` and **bold** -> TextData, by the notebook's own reader of formulas; every
   problem that the reader reports for a formula becomes a warning of the build *)
SL31Build`warnings = {};
SL31Build`formulaCount = 0;
SL31Build`check[tex_String] := (SL31Build`formulaCount++;
   Scan[AppendTo[SL31Build`warnings, "formula " <> tex <> ": " <> #] &, SL31`SL31TeXFormulaProblems[tex]]);
textData[t_String] := (Scan[SL31Build`check, SL31`SL31TextFormulas[t]]; SL31`SL31TextData[t]);

(* display formula: the whole body is TeX.  It is set in the Times typeface like the formulas in a text, and one
   quarter larger than the style DisplayFormula sets it, which gives the size of the formulas in a text cell *)
displayCell[t_String] := (SL31Build`check[t];
   Cell[BoxData[FormBox[SL31`SL31TeXFormula[t], TraditionalForm]], "DisplayFormula", FontFamily -> "Times",
     FontSize -> 1.25 Inherited]);

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

SL31Build`closedCount = 0;
cellOf[{"Package", _, _}] := Nothing;
cellOf[{style_, tag_, body_}] := Which[
   (* a closed input cell: the same boxes, with the option CellOpen -> False *)
   style === "Input" && tag === "Closed", (SL31Build`closedCount++; Append[codeCell[StringTrim[body, "\n" ..]], CellOpen -> False]),
   tag =!= "", (AppendTo[SL31Build`warnings, "unknown tag " <> tag <> " of a cell of the style " <> style]; Cell[commentText[body], "Text"]),
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
Print["closed input cells: ", SL31Build`closedCount];
Print["formulas in text cells: ", SL31Build`formulaCount];
Print["warnings: ", Length[SL31Build`warnings]];
Scan[Print, SL31Build`warnings];
Exit[0];
