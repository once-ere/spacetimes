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
     - turns $...$ in text into inline formulas (TeX converted by the Wolfram TeX importer; the
       alphabets of bold, sans-serif, double-struck, script and Gothic letters are kept),
       `...` (or ``...`` for code that contains the character `) into code font and **...** into
       bold;
     - inserts the content of the file named by a line  (* @@INCLUDE file@@ *)  (relative to
       the source) into the code chunk in which it stands, so that the notebook is complete;
     - replaces a line  (* @@INCLUDECELLS file@@ *)  by a whole file with its own cell markers
       (the chapters of Part II).
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
chunks = StringCases[text,
   RegularExpression["(?ms)^\\(\\* ::(\\w+)(?:::\\w+)?:: \\*\\)[ \\t]*\\n(.*?)(?=^\\(\\* ::\\w+(?:::\\w+)?:: \\*\\)|\\z)"] :> {"$1", "$2"}];

textStyles = {"Title", "Subtitle", "Subsubtitle", "Chapter", "Section", "Subsection", "Subsubsection",
   "Text", "Item", "Subitem", "ItemNumbered", "SubitemNumbered", "ItemParagraph", "Program"};

(* the text of a comment chunk: strip one (* ... *) pair *)
commentText[body_String] := StringTrim[StringReplace[StringTrim[body],
    RegularExpression["(?s)^\\(\\*(.*)\\*\\)$"] :> "$1"]];

(* The alphabets of letters in text formulas.  The Wolfram TeX importer loses them: \mathbf b gives
   StyleBox["b", FontSlant -> "Bold"] (an invalid value, shown as a plain letter), \mathsf N and \mathbb R give
   plain letters, \mathcal L and \mathfrak g give italic letters.  A bold b and a plain b, or the set R of the real
   numbers and a letter R, would then look the same.  Therefore every styled letter is first replaced by a
   placeholder word inside \mathrm{...}, which the importer keeps as ONE string (for example \mathbb R becomes
   \mathrm{QZbbQRZQ}), and after the import the placeholder is replaced by the box that the converter of the
   notebook (sl31_core.wl) uses for the same letter: double-struck, script and Gothic characters, and bold or
   sans-serif upright letters.  The importer also writes \varepsilon as the lunate epsilon (the character of
   \epsilon, used for the Levi-Civita symbol); \varepsilon is therefore protected in the same way and becomes the
   curly epsilon, as in the converter of the notebook. *)
SL31Build`alphaCode = <|"mathbf" -> "bf", "mathsf" -> "sf", "mathbb" -> "bb", "mathcal" -> "cal", "mathfrak" -> "frak"|>;
SL31Build`protectAlphabets[tex_String] := StringReplace[StringReplace[tex,
    RegularExpression["\\\\(mathbf|mathsf|mathbb|mathcal|mathfrak)\\s*(?:\\{([A-Za-z0-9]+)\\}|([A-Za-z0-9]))"] :>
      "\\mathrm{QZ" <> SL31Build`alphaCode["$1"] <> "Q" <> "$2" <> "$3" <> "ZQ}"],
   RegularExpression["\\\\varepsilon(?![A-Za-z])"] -> "\\mathrm{QZgrQvarepsilonZQ}"];
(* one character of a special alphabet: capital and small letters have their own character names; digits stay *)
SL31Build`special[kind_String, ch_String] := Which[
   DigitQ[ch], ch,
   UpperCaseQ[ch], ToExpression["\"\\[" <> kind <> "Capital" <> ch <> "]\""],
   True, ToExpression["\"\\[" <> kind <> ToUpperCase[ch] <> "]\""]];
SL31Build`alphaBox["bf", x_] := StyleBox[x, FontWeight -> "Bold", FontSlant -> "Plain"];
SL31Build`alphaBox["sf", x_] := StyleBox[x, FontFamily -> "Helvetica", FontSlant -> "Plain"];
SL31Build`alphaBox["bb", x_] := StyleBox[StringJoin[SL31Build`special["DoubleStruck", #] & /@ Characters[x]], FontSlant -> "Plain"];
SL31Build`alphaBox["cal", x_] := StyleBox[StringJoin[SL31Build`special["Script", #] & /@ Characters[x]], FontSlant -> "Plain"];
SL31Build`alphaBox["frak", x_] := StyleBox[StringJoin[SL31Build`special["Gothic", #] & /@ Characters[x]], FontSlant -> "Plain"];
SL31Build`alphaBox["gr", "varepsilon"] := "\[CurlyEpsilon]";
SL31Build`placeholderRE = RegularExpression["QZ(bf|sf|bb|cal|frak|gr)Q([A-Za-z0-9]+)ZQ"];
SL31Build`placeholderBox[s_String] := SL31Build`alphaBox @@ First[StringCases[s, SL31Build`placeholderRE :> {"$1", "$2"}]];
SL31Build`restoreAlphabets[boxes_] := boxes /. {
    StyleBox[s_String /; StringMatchQ[s, SL31Build`placeholderRE], ___] :> SL31Build`placeholderBox[s],
    s_String /; StringMatchQ[s, SL31Build`placeholderRE] :> SL31Build`placeholderBox[s]};

(* inline TeX -> an InlineFormula cell (Wolfram TeX importer, with the alphabets kept); on failure keep the TeX *)
SL31Build`inline[tex_String] := Module[{nb = Quiet[ImportString["$" <> SL31Build`protectAlphabets[tex] <> "$", "TeX"]], c},
   c = Cases[nb, Cell[BoxData[f_FormBox], "InlineFormula", ___], Infinity];
   If[c === {}, AppendTo[SL31Build`warnings, "TeX not converted: " <> tex]; "$" <> tex <> "$",
     Cell[BoxData[SL31Build`restoreAlphabets[First[c]]], "InlineFormula"]]];
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
displayCell[t_String] := Module[{nb = Quiet[ImportString["$$" <> SL31Build`protectAlphabets[t] <> "$$", "TeX"]], c},
   c = Cases[nb, Cell[BoxData[f_FormBox], ___], Infinity];
   If[c === {}, AppendTo[SL31Build`warnings, "display TeX not converted: " <> t];
     Cell["$$" <> t <> "$$", "Text"],
     Cell[BoxData[SL31Build`restoreAlphabets[First[c]]], "DisplayFormula"]]];

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
