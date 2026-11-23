#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   build_notebook.wls      (PROJECT spacetimes, folder Revision_Lord_Gravity, notebook lord_gravity.nb;
                            first written for the notebook 3+1super-loop.nb of the project, copied to the
                            folder Revision_SM_Higgs for its notebook superfluid.nb (contexts SFNB), and
                            copied here with the names of the contexts changed from SFNB to LGNB)

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
       formulas (LGNBTextData of wolfram/tools/lg_formula.wl; the TeX importer of the
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
   The package lg_formula.wl defines the reader of formulas, used for the text cells.  (A package that defines
   LGNB`Build`symbolBoxes[tex, rules] would give two-dimensional boxes for references LL[...] in the code; the
   notebook lord_gravity.nb has no such references and loads no such package.)
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 3, Print["usage: build_notebook.wls source.wl out.nb package.wl [package.wl ...]"]; Exit[2]];
{src, outNB} = args[[1 ;; 2]];
srcDir = DirectoryName[src];
(* the symbol machinery (it loads the Notation package, which asks the front end for settings) and
   the reader of formulas *)
Scan[UsingFrontEnd[Get[#]] &, Drop[args, 2]];
boxesDefinedQ = Length[DownValues[LGNB`Build`symbolBoxes]] > 0;
If[Length[DownValues[LGNB`LGNBTextData]] == 0,
  Print["the packages do not define the reader of formulas (LGNBTextData)"]; Exit[2]];

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
LGNBBuild`components = DeleteDuplicates[StringCases[text, componentRE :> "$1"]];
text = StringReplace[text, "(* @@COMPONENTS@@ *)" -> "{" <> StringRiffle[Map["{" <> # <> "}" &, LGNBBuild`components], ", "] <> "}"];
Print["components: ", Length[LGNBBuild`components]];

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
LGNBBuild`warnings = {};
LGNBBuild`formulaCount = 0;
LGNBBuild`check[tex_String] := (LGNBBuild`formulaCount++;
   Scan[AppendTo[LGNBBuild`warnings, "formula " <> tex <> ": " <> #] &, LGNB`LGNBTeXFormulaProblems[tex]]);
textData[t_String] := (Scan[LGNBBuild`check, LGNB`LGNBTextFormulas[t]]; LGNB`LGNBTextData[t]);

(* display formula: the whole body is TeX.  It is set in the Times typeface like the formulas in a text, and one
   quarter larger than the style DisplayFormula sets it, which gives the size of the formulas in a text cell *)
displayCell[t_String] := (LGNBBuild`check[t];
   Cell[BoxData[FormBox[LGNB`LGNBTeXFormula[t], TraditionalForm]], "DisplayFormula", FontFamily -> "Times",
     FontSize -> 1.25 Inherited]);

(* code: replace LL["..."] references by placeholders, parse with the front end, substitute boxes *)
LGNBBuild`placeholders = <||>;
codeCell[code_String] := Module[{k = 0, rep, parsed, boxes},
   rep = StringReplace[code,
     RegularExpression["LL\\[(\"(?:[^\"\\\\]|\\\\.)*\"(?:\\s*,\\s*\"(?:[^\"\\\\]|\\\\.)*\"\\s*->\\s*\"(?:[^\"\\\\]|\\\\.)*\")*)\\]"] :>
       Module[{id = "SLPH" <> ToString[++k] <> "X", argsExpr = ToExpression["{" <> "$1" <> "}"]},
         LGNBBuild`placeholders[id] = If[boxesDefinedQ,
           LGNB`Build`symbolBoxes @@ argsExpr, $Failed];
         If[LGNBBuild`placeholders[id] === $Failed,
           AppendTo[LGNBBuild`warnings, "no boxes for LL[" <> "$1" <> "]"]; "LL[" <> "$1" <> "]", id]]];
   parsed = FrontEndExecute[FrontEnd`UndocumentedTestFEParserPacket[rep, False]];
   boxes = First[parsed];
   boxes = boxes /. s_String /; KeyExistsQ[LGNBBuild`placeholders, s] :> LGNBBuild`placeholders[s];
   Cell[boxes, "Input"]];

LGNBBuild`closedCount = 0;
cellOf[{"Package", _, _}] := Nothing;
cellOf[{style_, tag_, body_}] := Which[
   (* a closed input cell: the same boxes, with the option CellOpen -> False *)
   style === "Input" && tag === "Closed", (LGNBBuild`closedCount++; Append[codeCell[StringTrim[body, "\n" ..]], CellOpen -> False]),
   tag =!= "", (AppendTo[LGNBBuild`warnings, "unknown tag " <> tag <> " of a cell of the style " <> style]; Cell[commentText[body], "Text"]),
   style === "Input" || style === "Code", codeCell[StringTrim[body, "\n" ..]],
   style === "DisplayFormula", displayCell[commentText[body]],
   MemberQ[textStyles, style], Cell[textData[commentText[body]], style],
   True, (AppendTo[LGNBBuild`warnings, "unknown style " <> style]; Cell[commentText[body], "Text"])];

cells = UsingFrontEnd[cellOf /@ chunks];
nb = Notebook[cells, StyleDefinitions -> "Default.nb", WindowSize -> {1000, 900}];
(* the notebook expression is written by the kernel itself, NOT through the front end: when the
   front end writes a notebook file it reads the boxes of every input cell again, and a box that
   looks like an operator (a partial-derivative sign with a subscript, for example) makes it regroup
   the whole line, so that the typed symbols would no longer be the symbolized boxes *)
Export[outNB, nb, "NB"];
Print["cells: ", Length[cells]];
Print["closed input cells: ", LGNBBuild`closedCount];
Print["formulas in text cells: ", LGNBBuild`formulaCount];
Print["warnings: ", Length[LGNBBuild`warnings]];
Scan[Print, LGNBBuild`warnings];
Exit[0];
