#!/usr/bin/env wolframscript
(* style_font_sizes.wls      (PROJECT spacetimes, notebook 3+1super-loop.nb, error E24 of the provenance)

   PURPOSE
     Prints the font family and the font size that the style sheet "Default.nb" of the front end gives to
     the cell styles Text, Item, DisplayFormula, InlineFormula and Output.  The notebook sets a formula
     inside a text 1.1 times as large as the text, and a display formula 1.25 times as large as the style
     DisplayFormula; this script shows that the two sizes are then nearly equal.

   METHOD
     An invisible notebook with the style sheet "Default.nb" is created through the front end; for each
     style, AbsoluteCurrentValue returns the resolved value of the option.

   USAGE
     wolframscript -file style_font_sizes.wls
*)
styles = {"Text", "Item", "DisplayFormula", "InlineFormula", "Output"};
Print["$Version = ", $Version];
UsingFrontEnd[
  Module[{nb, size, family},
   nb = CreateDocument[{}, Visible -> False, StyleDefinitions -> "Default.nb"];
   Do[
    size = AbsoluteCurrentValue[nb, {StyleDefinitions, st, FontSize}];
    family = AbsoluteCurrentValue[nb, {StyleDefinitions, st, FontFamily}];
    Print[st, ": font family ", family, ", font size ", size],
    {st, styles}];
   size = AbsoluteCurrentValue[nb, {StyleDefinitions, "Text", FontSize}];
   Print["a formula inside a text: 1.1 times the size of Text = ", 1.1 size];
   size = AbsoluteCurrentValue[nb, {StyleDefinitions, "DisplayFormula", FontSize}];
   Print["a display formula: 1.25 times the size of DisplayFormula = ", 1.25 size];
   NotebookClose[nb]]];
Print["done"];
