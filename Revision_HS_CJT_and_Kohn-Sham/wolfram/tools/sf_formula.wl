(* ::Package:: *)
(* ------------------------------------------------------------------------------------------------
   The reader of formulas of the notebook superfluid.nb (folder Revision_HS_CJT_and_Kohn-Sham of PROJECT
   spacetimes), in the context SFNB` .  It was first written for the notebook 3+1super-loop.nb of the
   project (context SL31`) and is copied here with the names of the context changed, so that this folder
   holds every program that builds its notebook.
   It converts the TeX of a complete formula (as it stands between dollar signs in the texts of
   this notebook and in the meanings of the registry of the symbols) into two-dimensional
   Wolfram boxes for display.

   WHY THE NOTEBOOK HAS ITS OWN READER OF FORMULAS
   The TeX importer of the Wolfram Language, ImportString["$...$", "TeX"], reads a fraction
   whose arguments are written WITHOUT braces wrongly.  In TeX an argument without braces is the
   single token that follows: \tfrac12 x is "one half times x".  The importer instead takes
   everything up to the end of the group as the denominator, and everything before the fraction
   as part of the numerator: it shows \tfrac12 x as 1/(2x), and a=b+\tfrac12 x as the fraction
   with the numerator a=b+1 and the denominator 2x.  It also shows the command \hbar as nothing,
   and it loses the alphabets of letters (a bold b, a sans-serif N and the double-struck R
   become plain letters).  A formula that is shown wrongly teaches a wrong statement, so this
   notebook converts every formula itself, with the rules of TeX, and tests the conversion
   (test S-15).

   THE METHOD (recursive descent, as in the reader of symbols of section I.3)
     1. The TeX is cut into tokens: a command (\frac, \Delta), a control symbol (a backslash
        followed by one character that is not a letter), white space, or one other character.
        Digits stay single tokens, because an argument without braces is ONE token: \tfrac12
        has the numerator 1 and the denominator 2.
     2. A sequence of atoms is read from left to right.  An atom is a letter, a digit, a
        character of a command, a group in braces, a part between matching delimiters such as
        ( ... ), or a command together with its arguments (\frac{a}{b}, \sqrt{a}, \vec d,
        \mathbf b, \begin{pmatrix} ... \end{pmatrix}); after the atom come its primes, its
        superscript and its subscript.
     3. Every letter carries its alphabet: an italic letter of a variable, upright text ({\rm ..},
        \mathrm, \text), bold (\mathbf), sans-serif (\mathsf), double-struck (\mathbb), script
        (\mathcal) or Gothic (\mathfrak).  Neighbouring digits form one number and neighbouring
        letters of one alphabet one string; a number that directly follows a letter is joined
        to the letter without a space (the coordinate x5 is one name, not a product).
     4. A sign that stands first, or after an operator, a relation, an opening delimiter or a
        comma, is a prefix sign and is joined to the item that follows it; the name of a
        function (cos, exp) and a large operator (the summation sign) are separated from a
        neighbouring ordinary item by a thin space, as TeX separates them.
   A command that the reader does not know, a missing argument or an unmatched brace is NOT
   passed over in silence: SFNBTeXFormulaProblems[tex] lists every such problem.
   ------------------------------------------------------------------------------------------------ *)
BeginPackage["SFNB`"];

SFNBTeXFormula::usage = "SFNBTeXFormula[tex] converts the TeX of a formula into two-dimensional Wolfram boxes for display: fractions, roots, sums and integrals with their limits, accents, matrices, delimiters, and every letter in its alphabet (italic, upright, bold, sans-serif, double-struck, script, Gothic).";
SFNBTeXFormulaProblems::usage = "SFNBTeXFormulaProblems[tex] lists what the reader of formulas could not read in tex: unknown commands, missing arguments, unmatched braces. The empty list means that the whole formula was read.";
SFNBFormulaCell::usage = "SFNBFormulaCell[tex] returns the inline cell that shows the formula tex inside a text.";
SFNBTextData::usage = "SFNBTextData[text] converts a text with formulas between dollar signs, code between backquotes and bold text between double asterisks into the content of a text cell.";
SFNBTextFormulas::usage = "SFNBTextFormulas[text] lists the TeX of the formulas that SFNBTextData finds in text.";
SFNBTextBoxes::usage = "SFNBTextBoxes[text, options] returns the boxes of an inline text cell that shows text (converted by SFNBTextData); the options are cell options such as FontSize.";
SFNBFormulaReading::usage = "SFNBFormulaReading[boxes] writes boxes on one line, so that two readings of a formula can be compared: a fraction as frac{a}{b}, a root as sqrt{a}, scripts as _{..} and ^{..}, an accent as over{a}{accent}, a matrix as grid[a&b;c&d], and a bold or sans-serif letter as <bf:b> or <sf:N>; spaces, the italic and upright styles and the grouping of rows are left out.";

Begin["`Private`"];

(* ---------- 0. The characters of the TeX commands of letters and signs ----------
   (in the notebook 3+1super-loop.nb this table belongs to its reader of symbols; this notebook has no reader of
   symbols, so the table stands here) *)
cmd = <|
  "\\alpha" -> "\[Alpha]", "\\beta" -> "\[Beta]", "\\gamma" -> "\[Gamma]", "\\delta" -> "\[Delta]",
  "\\epsilon" -> "\[Epsilon]", "\\varepsilon" -> "\[CurlyEpsilon]", "\\zeta" -> "\[Zeta]", "\\eta" -> "\[Eta]",
  "\\theta" -> "\[Theta]", "\\vartheta" -> "\[CurlyTheta]", "\\iota" -> "\[Iota]", "\\kappa" -> "\[Kappa]",
  "\\varkappa" -> "\[CurlyKappa]", "\\lambda" -> "\[Lambda]", "\\mu" -> "\[Mu]", "\\nu" -> "\[Nu]",
  "\\xi" -> "\[Xi]", "\\pi" -> "\[Pi]", "\\varpi" -> "\[CurlyPi]", "\\rho" -> "\[Rho]",
  "\\varrho" -> "\[CurlyRho]", "\\sigma" -> "\[Sigma]", "\\varsigma" -> "\[FinalSigma]", "\\tau" -> "\[Tau]",
  "\\upsilon" -> "\[Upsilon]", "\\phi" -> "\[Phi]", "\\varphi" -> "\[CurlyPhi]", "\\chi" -> "\[Chi]",
  "\\psi" -> "\[Psi]", "\\omega" -> "\[Omega]",
  "\\Gamma" -> "\[CapitalGamma]", "\\Delta" -> "\[CapitalDelta]", "\\Theta" -> "\[CapitalTheta]",
  "\\Lambda" -> "\[CapitalLambda]", "\\Xi" -> "\[CapitalXi]", "\\Pi" -> "\[CapitalPi]",
  "\\Sigma" -> "\[CapitalSigma]", "\\Upsilon" -> "\[CapitalUpsilon]", "\\Phi" -> "\[CapitalPhi]",
  "\\Psi" -> "\[CapitalPsi]", "\\Omega" -> "\[CapitalOmega]",
  "\\ell" -> "\[ScriptL]", "\\nabla" -> "\[Del]", "\\partial" -> "\[PartialD]", "\\infty" -> "\[Infinity]",
  "\\pm" -> "\[PlusMinus]", "\\mp" -> "\[MinusPlus]", "\\dagger" -> "\[Dagger]",
  "\\uparrow" -> "\[UpArrow]", "\\downarrow" -> "\[DownArrow]", "\\cdot" -> "\[CenterDot]",
  "\\dots" -> "\[Ellipsis]", "\\ldots" -> "\[Ellipsis]", "\\times" -> "\[Times]",
  "\\otimes" -> "\[CircleTimes]", "\\oplus" -> "\[CirclePlus]", "\\circ" -> "\[SmallCircle]",
  "\\langle" -> "\[LeftAngleBracket]", "\\rangle" -> "\[RightAngleBracket]",
  "\\lvert" -> "|", "\\rvert" -> "|", "\\vert" -> "|",
  "\\deg" -> "deg", "\\det" -> "det", "\\exp" -> "exp", "\\ln" -> "ln", "\\sum" -> "\[Sum]",
  "\\int" -> "\[Integral]"|>;

(* ---------- 1. Tokens ---------- *)
formulaTokens[s_String] := StringCases[s, {
    RegularExpression["\\\\[A-Za-z]+"],     (* a command such as \frac or \Delta *)
    RegularExpression["(?s)\\\\."],         (* a control symbol: a backslash and one character that is not a
                                               letter (the spaces of TeX, the double backslash that ends
                                               a row of a matrix, the braces written as characters) *)
    RegularExpression["\\s+"],              (* white space: skipped, except inside \text{...} *)
    RegularExpression["(?s)."]}];           (* any other single character *)

(* ---------- 2. Tables ---------- *)
(* the characters of the commands: those of the table cmd of part 0 (Greek letters, \ell, \nabla,
   \partial, \infty, \pm, \dagger, arrows, \cdot, \times, ...) and, in addition, the delimiters,
   relations and other signs of complete formulas *)
formulaChar = Join[cmd, <|
    "\\lvert" -> "\[LeftBracketingBar]", "\\rvert" -> "\[RightBracketingBar]",
    "\\lVert" -> "\[LeftDoubleBracketingBar]", "\\rVert" -> "\[RightDoubleBracketingBar]",
    "\\{" -> "{", "\\}" -> "}", "\\|" -> "\[DoubleVerticalBar]", "\\vert" -> "|", "\\mid" -> "|",
    "\\lfloor" -> "\[LeftFloor]", "\\rfloor" -> "\[RightFloor]", "\\lceil" -> "\[LeftCeiling]", "\\rceil" -> "\[RightCeiling]",
    "\\le" -> "\[LessEqual]", "\\leq" -> "\[LessEqual]", "\\ge" -> "\[GreaterEqual]", "\\geq" -> "\[GreaterEqual]",
    "\\neq" -> "\[NotEqual]", "\\ne" -> "\[NotEqual]", "\\ll" -> "\[LessLess]", "\\gg" -> "\[GreaterGreater]",
    "\\approx" -> "\[TildeTilde]", "\\simeq" -> "\[TildeEqual]", "\\sim" -> "\[Tilde]", "\\equiv" -> "\[Congruent]",
    "\\propto" -> "\[Proportional]", "\\in" -> "\[Element]", "\\notin" -> "\[NotElement]",
    "\\subset" -> "\[Subset]", "\\supset" -> "\[Superset]", "\\subseteq" -> "\[SubsetEqual]", "\\supseteq" -> "\[SupersetEqual]",
    "\\cup" -> "\[Union]", "\\cap" -> "\[Intersection]",
    "\\to" -> "\[RightArrow]", "\\rightarrow" -> "\[RightArrow]", "\\leftarrow" -> "\[LeftArrow]",
    "\\leftrightarrow" -> "\[LeftRightArrow]", "\\mapsto" -> "\[RightTeeArrow]",
    "\\Rightarrow" -> "\[DoubleRightArrow]", "\\Leftarrow" -> "\[DoubleLeftArrow]", "\\Leftrightarrow" -> "\[DoubleLeftRightArrow]",
    "\\hbar" -> "\[HBar]", "\\cdots" -> "\[CenterEllipsis]", "\\prime" -> "\[Prime]", "\\ast" -> "*", "\\star" -> "\[Star]",
    "\\wedge" -> "\[Wedge]", "\\vee" -> "\[Vee]", "\\forall" -> "\[ForAll]", "\\exists" -> "\[Exists]",
    "\\emptyset" -> "\[EmptySet]", "\\perp" -> "\[UpTee]", "\\parallel" -> "\[DoubleVerticalBar]",
    "\\%" -> "%", "\\&" -> "&", "\\#" -> "#", "\\_" -> "_", "\\$" -> "$"|>];

(* an opening delimiter and the delimiter that closes it *)
formulaDelims = <|"(" -> ")", "[" -> "]", "\\{" -> "\\}", "\\lvert" -> "\\rvert", "\\lVert" -> "\\rVert",
    "\\langle" -> "\\rangle", "\\lfloor" -> "\\rfloor", "\\lceil" -> "\\rceil"|>;
formulaClosers = Values[formulaDelims];
formulaDelimChar[k_String] := Lookup[formulaChar, k, k];

(* the class of a character, which decides the spacing: a binary operator, a relation or a
   punctuation mark; every other item is an ordinary item *)
formulaClass = Join[
    AssociationMap["bin" &, {"+", "-", "*", "\[PlusMinus]", "\[MinusPlus]", "\[Times]", "\[CenterDot]", "\[CircleTimes]",
       "\[CirclePlus]", "\[SmallCircle]", "\[Wedge]", "\[Vee]", "\[Union]", "\[Intersection]", "\[Star]"}],
    AssociationMap["rel" &, {"<", ">", ":", "\[LongEqual]", "\[LessEqual]", "\[GreaterEqual]", "\[NotEqual]", "\[LessLess]",
       "\[GreaterGreater]", "\[TildeTilde]", "\[TildeEqual]", "\[Tilde]", "\[Congruent]", "\[Proportional]", "\[Element]",
       "\[NotElement]", "\[Subset]", "\[Superset]", "\[SubsetEqual]", "\[SupersetEqual]", "\[RightArrow]", "\[LeftArrow]",
       "\[LeftRightArrow]", "\[RightTeeArrow]", "\[DoubleRightArrow]", "\[DoubleLeftArrow]", "\[DoubleLeftRightArrow]"}],
    AssociationMap["punct" &, {",", ";"}]];
formulaSigns = {"+", "-", "\[PlusMinus]", "\[MinusPlus]"};

(* commands that only choose the size of a delimiter or the style of a formula: they do not change
   what the formula says and are left out *)
formulaIgnored = {"\\big", "\\Big", "\\bigg", "\\Bigg", "\\bigl", "\\bigr", "\\Bigl", "\\Bigr", "\\biggl", "\\biggr",
    "\\Biggl", "\\Biggr", "\\bigm", "\\Bigm", "\\left", "\\right", "\\displaystyle", "\\textstyle", "\\scriptstyle",
    "\\limits", "\\nolimits", "\\nonumber", "\\strut", "\\mathstrut"};
(* the spaces of TeX; the negative thin space (a backslash and an exclamation mark) becomes the
   empty string, which is removed at the end but keeps the automatic thin space of rule 4 from
   being inserted at its place *)
formulaSpace = <|"\\," -> "\[ThinSpace]", "\\:" -> "\[MediumSpace]", "\\;" -> "\[ThickSpace]",
    "\\ " -> " ", "~" -> " ", "\\!" -> "", "\\quad" -> "   ", "\\qquad" -> "      "|>;
(* names of functions, set upright; those of the second list take their subscript as a limit *)
formulaFunctions = {"\\sin", "\\cos", "\\tan", "\\cot", "\\sec", "\\csc", "\\sinh", "\\cosh", "\\tanh", "\\coth",
    "\\arcsin", "\\arccos", "\\arctan", "\\exp", "\\log", "\\ln", "\\lg", "\\det", "\\deg", "\\dim", "\\ker", "\\arg",
    "\\gcd", "\\hom", "\\lim", "\\max", "\\min", "\\sup", "\\inf", "\\Pr"};
formulaLimitNames = {"lim", "max", "min", "sup", "inf"};
(* large operators; the summation and the product sign take their scripts as limits *)
formulaBigOp = <|"\\sum" -> "\[Sum]", "\\prod" -> "\[Product]", "\\int" -> "\[Integral]", "\\oint" -> "\[ContourIntegral]"|>;
formulaLimitsQ[b_] := MemberQ[{"\[Sum]", "\[Product]"}, b] ||
    MatchQ[b, StyleBox[w_String /; MemberQ[formulaLimitNames, w], ___]];
(* the alphabets: a command with one argument, or a switch for the rest of the group *)
formulaStyle = <|"\\mathbf" -> "bf", "\\mathsf" -> "sf", "\\mathbb" -> "bb", "\\mathcal" -> "cal", "\\mathfrak" -> "frak",
    "\\mathrm" -> "rm", "\\mathit" -> "it", "\\boldsymbol" -> "bf", "\\textbf" -> "bf", "\\textrm" -> "rm",
    "\\textit" -> "it", "\\operatorname" -> "rm"|>;
formulaSwitch = <|"\\rm" -> "rm", "\\it" -> "it", "\\bf" -> "bf", "\\sf" -> "sf", "\\cal" -> "cal"|>;
(* accents: the argument with the accent character above it *)
formulaAccent = <|"\\vec" -> "\[RightVector]", "\\hat" -> "^", "\\widehat" -> "^", "\\tilde" -> "~", "\\widetilde" -> "~",
    "\\bar" -> "_", "\\overline" -> "_", "\\dot" -> ".", "\\ddot" -> ".."|>;

(* ---------- 3. Letters and numbers ---------- *)
(* While a sequence is read, a letter is kept as ch[alphabet, letter] and a digit as num[digit];
   formulaFin gives the final box.  An italic letter gets the style "TI" (text italic) and upright
   text the explicit upright slant: with explicit styles a letter is shown in the same way in a
   text cell and in the output of a program (where single letters would otherwise be set in
   italics automatically). *)
formulaSpecial[kind_String, c_String] := Which[
    DigitQ[c], c,
    UpperCaseQ[c], ToExpression["\"\\[" <> kind <> "Capital" <> c <> "]\""],
    True, ToExpression["\"\\[" <> kind <> ToUpperCase[c] <> "]\""]];
formulaFin[ch["it", c_String]] := StyleBox[c, "TI"];
formulaFin[ch["rm", w_String]] := StyleBox[w, FontSlant -> "Plain"];
formulaFin[ch["bf", w_String]] := StyleBox[w, FontWeight -> "Bold", FontSlant -> "Plain"];
formulaFin[ch["sf", w_String]] := StyleBox[w, FontFamily -> "Helvetica", FontSlant -> "Plain"];
formulaFin[ch["bb", w_String]] := StyleBox[StringJoin[formulaSpecial["DoubleStruck", #] & /@ Characters[w]], FontSlant -> "Plain"];
formulaFin[ch["cal", w_String]] := StyleBox[StringJoin[formulaSpecial["Script", #] & /@ Characters[w]], FontSlant -> "Plain"];
formulaFin[ch["frak", w_String]] := StyleBox[StringJoin[formulaSpecial["Gothic", #] & /@ Characters[w]], FontSlant -> "Plain"];
formulaFin[num[s_String]] := s;
(* a letter with the number that directly follows it, the name of one quantity such as the
   coordinate x5: the character InvisibleSpace, which has no width, stands between the two, so
   that the front end does not put the thin space of a product there *)
formulaFin[named[a_, b_]] := RowBox[{formulaFin[a], "\[InvisibleSpace]", formulaFin[b]}];
formulaFin[x_] := x;

formulaSingle[boxes_List] := Which[boxes === {}, "", Length[boxes] == 1, First[boxes], True, RowBox[boxes]];

(* ---------- 4. From the items of a sequence to its boxes ---------- *)
(* an item is {class, content}; the classes are "ord" (an ordinary item), "grp" (a part between
   matching delimiters), "op" (the name of a function or a large operator), "bin", "rel",
   "punct", "open" and "close" (a delimiter without its partner) and "space" *)
formulaFinish[items0_List] := Module[{items, out = {}, res = {}, k = 1, last = None, a, b},
   (* neighbouring letters of one alphabet form one string, so that they are set without a space
      between them, as TeX sets them (upright letters form a word; italic letters are the letters
      of a product or of a name such as SU); neighbouring digits, and digits around a decimal
      point, form one number *)
   items = items0 //. {
      {x___, {c1_, ch[al_, u_]}, {_, ch[al_, v_]}, y___} :> {x, {c1, ch[al, u <> v]}, y},
      {x___, {_, num[u_]}, {_, num[v_]}, y___} :> {x, {"ord", num[u <> v]}, y},
      {x___, {_, num[u_]}, {_, "."}, {_, num[v_]}, y___} :> {x, {"ord", num[u <> "." <> v]}, y}};
   (* a number that directly follows a letter belongs to the name of a quantity (the coordinates
      x5, x6, x7 of the main project); TeX sets the two without a space.  A number BEFORE a letter
      is a factor of a product and keeps the thin space that the front end puts between factors. *)
   items = items //. {x___, {c1_, letter : ch[_, _]}, {_, number : num[_]}, y___} :> {x, {c1, named[letter, number]}, y};
   items = Map[{#[[1]], formulaFin[#[[2]]]} &, items];
   (* prefix signs *)
   While[k <= Length[items],
     a = items[[k]];
     If[a[[1]] === "bin" && MemberQ[formulaSigns, a[[2]]] && MemberQ[{None, "bin", "rel", "open", "punct", "op"}, last],
       If[k < Length[items] && MemberQ[{"ord", "grp", "op"}, items[[k + 1, 1]]],
         b = items[[k + 1]];
         AppendTo[out, {If[b[[1]] === "op", "op", "ord"], RowBox[{a[[2]], b[[2]]}]}]; k += 2,
         AppendTo[out, {"ord", a[[2]]}]; k++];
       last = "ord",
       AppendTo[out, a]; If[a[[1]] =!= "space", last = a[[1]]]; k++]];
   (* thin spaces beside the names of functions and the large operators *)
   Do[
     If[j > 1 && ((out[[j, 1]] === "op" && MemberQ[{"ord", "grp", "op", "close"}, out[[j - 1, 1]]]) ||
          (out[[j - 1, 1]] === "op" && out[[j, 1]] === "ord")),
       AppendTo[res, "\[ThinSpace]"]];
     AppendTo[res, out[[j, 2]]],
     {j, Length[out]}];
   DeleteCases[res, ""]];
formulaBox[items_List] := formulaSingle[formulaFinish[items]];

(* ---------- 5. The parser ---------- *)
(* formulaParse[tex] returns {boxes, problems} *)
formulaParse[tex_String] := Module[
  {t = formulaTokens[tex], n, i = 1, problems = {}, note, skip, peek, next, expect, matched, parseSeq, atom, arg,
   scripts, rawText, envName, envir, boxes},
  n = Length[t];
  note[s_String] := AppendTo[problems, s];
  skip[] := While[i <= n && StringMatchQ[t[[i]], RegularExpression["\\s+"]], i++];
  peek[] := (skip[]; If[i <= n, t[[i]], None]);
  next[] := (skip[]; If[i <= n, i++; t[[i - 1]], None]);
  expect[tok_String] := If[peek[] === tok, next[], note["missing " <> tok]];

  (* is the opening delimiter that was just read closed by its partner within the same group?
     (if not, as in a half-open interval, which opens with a square bracket and closes with a
     round one, the delimiter is an ordinary character) *)
  matched[open_String, close_String] := Module[{j = i, depth = 0, brace = 0, env = 0, tok, found = False, go = True},
    While[go && j <= n,
      tok = t[[j]];
      Which[
        tok === "{", brace++,
        tok === "}", If[brace == 0, go = False, brace--],
        tok === "\\begin", env++,
        tok === "\\end", If[env == 0, go = False, env--],
        (tok === "&" || tok === "\\\\") && brace == 0 && env == 0, go = False,
        brace == 0 && env == 0 && tok === close, If[depth == 0, found = True; go = False, depth--],
        brace == 0 && env == 0 && tok === open, depth++];
      j++];
    found];

  (* a sequence of atoms in the alphabet al0, up to one of the tokens of stops or the end; a
     switch such as \rm changes the alphabet for the rest of the sequence *)
  parseSeq[al0_String, stops_List] := Module[{al = al0, items = {}, tok},
    While[(tok = peek[]) =!= None && ! MemberQ[stops, tok],
      If[KeyExistsQ[formulaSwitch, tok],
        next[]; al = formulaSwitch[tok],
        items = Join[items, atom[al, True]]]];
    items];

  (* an argument: a group in braces, or else ONE token (with the arguments of that token, if it
     is a command, but without scripts) *)
  arg[al_String] := Module[{tok = peek[], r},
    Which[
      tok === None, note["an argument is missing at the end of the formula"]; {},
      tok === "{", next[]; r = parseSeq[al, {"}"}]; expect["}"]; r,
      True, r = atom[al, False]; If[r === {}, note["the argument " <> tok <> " is empty"]]; r]];

  (* the primes, the superscript and the subscript after an item *)
  scripts[al_String, {cls_String, content_}] := Module[{sub = None, sup = None, primes = "", go = True, tok, b},
    While[go,
      tok = peek[];
      Which[
        tok === "'", next[]; primes = primes <> "\[Prime]",
        tok === "^", next[]; If[sup =!= None, note["two superscripts on one item"]]; sup = formulaBox[arg[al]],
        tok === "_", next[]; If[sub =!= None, note["two subscripts on one item"]]; sub = formulaBox[arg[al]],
        True, go = False]];
    If[primes =!= "", sup = If[sup === None, primes, RowBox[{primes, sup}]]];
    If[sub === None && sup === None,
      {cls, content},
      b = formulaFin[content];
      {cls, Which[
         formulaLimitsQ[b] && sup === None, UnderscriptBox[b, sub, LimitsPositioning -> True],
         formulaLimitsQ[b] && sub === None, OverscriptBox[b, sup, LimitsPositioning -> True],
         formulaLimitsQ[b], UnderoverscriptBox[b, sub, sup, LimitsPositioning -> True],
         sup === None, SubscriptBox[b, sub],
         sub === None, SuperscriptBox[b, sup],
         True, SubsuperscriptBox[b, sub, sup]]}]];

  (* the text of \text{...}: every character as it is written, white space as one space *)
  rawText[] := Module[{s = "", depth = 0, go = True, tok},
    If[peek[] =!= "{",
      note["\\text is not followed by {"],
      next[];
      While[go && i <= n,
        tok = t[[i]]; i++;
        Which[
          tok === "{", depth++; s = s <> tok,
          tok === "}", If[depth == 0, go = False, depth--; s = s <> tok],
          StringMatchQ[tok, RegularExpression["\\s+"]], s = s <> " ",
          True, s = s <> tok]];
      If[go, note["\\text{ is not closed"]]];
    s];

  (* the name of an environment, {pmatrix} *)
  envName[] := Module[{s = "", tok},
    If[peek[] =!= "{",
      note["\\begin or \\end is not followed by {"],
      next[];
      While[(tok = peek[]) =!= None && tok =!= "}", s = s <> next[]];
      expect["}"]];
    s];
  (* a matrix: cells separated by &, rows by a double backslash *)
  envir[al_String] := Module[{name = envName[], rows = {}, row = {}, cell, go = True, tok, width, grid},
    While[go,
      cell = formulaBox[parseSeq[al, {"&", "\\\\", "\\end"}]];
      tok = next[];
      Which[
        tok === "&", AppendTo[row, cell],
        tok === "\\\\", AppendTo[row, cell]; AppendTo[rows, row]; row = {},
        tok === "\\end",
          AppendTo[row, cell]; If[row =!= {""} || rows === {}, AppendTo[rows, row]];
          If[envName[] =!= name, note["\\end does not close \\begin{" <> name <> "}"]]; go = False,
        True,
          note["\\begin{" <> name <> "} is not closed"]; AppendTo[row, cell]; AppendTo[rows, row]; go = False]];
    width = Max[Length /@ rows];
    grid = GridBox[PadRight[#, width, ""] & /@ rows, RowSpacings -> 1, ColumnSpacings -> 1,
      RowAlignments -> Baseline, ColumnAlignments -> Center];
    Switch[name,
      "pmatrix", RowBox[{"(", "\[NoBreak]", grid, "\[NoBreak]", ")"}],
      "bmatrix", RowBox[{"[", "\[NoBreak]", grid, "\[NoBreak]", "]"}],
      "vmatrix", RowBox[{"\[LeftBracketingBar]", "\[NoBreak]", grid, "\[NoBreak]", "\[RightBracketingBar]"}],
      "matrix", grid,
      _, note["unknown environment " <> name]; grid]];

  (* one atom; with its scripts if s is True.  The result is a list of items (empty for a
     command that is left out) *)
  atom[al_String, s_] := Module[{tok = next[], r, a, b, sc},
    sc[item_] := If[s, scripts[al, item], item];
    Which[
      tok === None, {},
      (* a group in braces *)
      tok === "{", r = formulaBox[parseSeq[al, {"}"}]]; expect["}"]; {sc[{"ord", r}]},
      tok === "}", note["a closing brace without an opening brace"]; {},
      tok === "&" || tok === "\\\\", note[tok <> " outside a matrix"]; {},
      tok === "\\end", note["\\end without \\begin"]; envName[]; {},
      (* scripts without an item before them: they are attached to an empty item *)
      tok === "^" || tok === "_" || tok === "'", i--; {scripts[al, {"ord", ""}]},
      (* fractions and roots *)
      MemberQ[{"\\frac", "\\tfrac", "\\dfrac"}, tok],
        a = formulaBox[arg[al]]; b = formulaBox[arg[al]]; {sc[{"ord", FractionBox[a, b]}]},
      tok === "\\sqrt",
        If[peek[] === "[",
          next[]; b = formulaBox[parseSeq[al, {"]"}]]; expect["]"]; a = formulaBox[arg[al]]; {sc[{"ord", RadicalBox[a, b]}]},
          a = formulaBox[arg[al]]; {sc[{"ord", SqrtBox[a]}]}],
      (* alphabets, accents, text, matrices *)
      KeyExistsQ[formulaStyle, tok], {sc[{"ord", formulaBox[arg[formulaStyle[tok]]]}]},
      KeyExistsQ[formulaAccent, tok], a = formulaBox[arg[al]]; {sc[{"ord", OverscriptBox[a, formulaAccent[tok]]}]},
      tok === "\\text" || tok === "\\mbox", {sc[{"ord", StyleBox[rawText[], FontSlant -> "Plain"]}]},
      tok === "\\begin", {sc[{"ord", envir[al]}]},
      (* commands that are left out, and spaces *)
      MemberQ[formulaIgnored, tok], If[MemberQ[{"\\left", "\\right"}, tok] && peek[] === ".", next[]]; {},
      KeyExistsQ[formulaSpace, tok], {{"space", formulaSpace[tok]}},
      (* names of functions and large operators *)
      MemberQ[formulaFunctions, tok], {sc[{"op", StyleBox[StringDrop[tok, 1], FontSlant -> "Plain"]}]},
      KeyExistsQ[formulaBigOp, tok], {sc[{"op", formulaBigOp[tok]}]},
      (* delimiters: a matching pair encloses a part of the formula *)
      KeyExistsQ[formulaDelims, tok],
        If[matched[tok, formulaDelims[tok]],
          r = formulaBox[parseSeq[al, {formulaDelims[tok]}]]; expect[formulaDelims[tok]];
          {sc[{"grp", RowBox[If[r === "", {formulaDelimChar[tok], formulaDelimChar[formulaDelims[tok]]},
               {formulaDelimChar[tok], r, formulaDelimChar[formulaDelims[tok]]}]]}]},
          {{"open", formulaDelimChar[tok]}}],
      MemberQ[formulaClosers, tok], {sc[{"close", formulaDelimChar[tok]}]},
      (* the character of a command *)
      KeyExistsQ[formulaChar, tok], {sc[{Lookup[formulaClass, formulaChar[tok], "ord"], formulaChar[tok]}]},
      StringStartsQ[tok, "\\"], note["unknown command " <> tok]; {sc[{"ord", tok}]},
      (* single characters *)
      LetterQ[tok], {sc[{"ord", ch[al, tok]}]},
      (* a digit: where a sequence is read, the whole number (further digits, and a decimal point that
         is followed by a digit), so that a script after it belongs to the number; as an argument
         without braces, the single digit *)
      DigitQ[tok],
        If[s && (al === "it" || al === "rm"),
          r = tok;
          While[i <= n && (DigitQ[t[[i]]] || (t[[i]] === "." && i < n && DigitQ[t[[i + 1]]])), r = r <> t[[i]]; i++];
          {sc[{"ord", num[r]}]},
          {sc[{"ord", If[al === "it" || al === "rm", num[tok], ch[al, tok]]}]}],
      tok === "=", {sc[{"rel", "\[LongEqual]"}]},
      True, {sc[{Lookup[formulaClass, tok, "ord"], tok}]}]];

  boxes = formulaBox[parseSeq["it", {}]];
  {boxes, problems}];

SFNBTeXFormula[tex_String] := First[formulaParse[tex]];
SFNBTeXFormulaProblems[tex_String] := Last[formulaParse[tex]];

(* ---------- 6. Formulas inside a text ---------- *)
(* The inline cell of a formula.  The formula is set in the Times typeface: the italic letters
   of variables (style "TI") and the upright letters of text written with \rm are then letters
   with serifs, and a sans-serif letter (set in Helvetica) differs visibly from an upright roman
   letter; the text around the formula is set in the sans-serif typeface of the text cells.
   The letters of Times are smaller than those of that typeface at the same font size, so the
   formula is set one tenth larger than the text around it. *)
SFNBFormulaCell[tex_String] := Cell[BoxData[FormBox[SFNBTeXFormula[tex], TraditionalForm]], "InlineFormula",
   FontFamily -> "Times", FontSize -> 1.1 Inherited];
(* text with $formula$, `code` (or ``code`` for code that contains a backquote) and **bold**.
   The four patterns are tried from left to right at every position, so that a dollar sign
   inside a code span does not begin a formula.  StringReplace returns a StringExpression of
   strings and boxes when a replacement is not a string. *)
SFNBTextData[text_String] := Module[{r},
   r = StringReplace[text, {
      RegularExpression["\\$([^$]+)\\$"] :> SFNBFormulaCell["$1"],
      RegularExpression["``(.+?)``"] :> StyleBox["$1", FontFamily -> "Source Code Pro"],
      RegularExpression["`([^`]+)`"] :> StyleBox["$1", FontFamily -> "Source Code Pro"],
      RegularExpression["\\*\\*([^*]+)\\*\\*"] :> StyleBox["$1", FontWeight -> "Bold"]}];
   TextData[If[Head[r] === StringExpression, List @@ r, {r}]]];
(* the formulas of a text, found with the same four patterns *)
SFNBTextFormulas[text_String] := StringCases[text, {
     RegularExpression["\\$([^$]+)\\$"] :> "$1",
     RegularExpression["``(.+?)``"] :> Nothing,
     RegularExpression["`([^`]+)`"] :> Nothing,
     RegularExpression["\\*\\*([^*]+)\\*\\*"] :> Nothing}];
SFNBTextBoxes[text_String, opts___] := Cell[SFNBTextData[text], "Text", opts];

(* ---------- 7. The reading of boxes on one line ---------- *)
formulaReadingChars = {" " -> "", "\[ThinSpace]" -> "", "\[MediumSpace]" -> "", "\[ThickSpace]" -> "", "\[NoBreak]" -> "",
    "\[InvisibleSpace]" -> "", "\[InvisibleTimes]" -> "", "\n" -> "", "\[LongEqual]" -> "=",
    "\[LeftBracketingBar]" -> "|", "\[RightBracketingBar]" -> "|",
    "\[LeftDoubleBracketingBar]" -> "||", "\[RightDoubleBracketingBar]" -> "||", "\[DoubleVerticalBar]" -> "||"};
SFNBFormulaReading[s_String] := StringReplace[s, formulaReadingChars];
SFNBFormulaReading[RowBox[l_List]] := StringJoin[SFNBFormulaReading /@ l];
SFNBFormulaReading[StyleBox[x_, opts___]] := Which[
    MemberQ[{opts}, FontWeight -> "Bold"], "<bf:" <> SFNBFormulaReading[x] <> ">",
    MemberQ[{opts}, FontFamily -> "Helvetica"], "<sf:" <> SFNBFormulaReading[x] <> ">",
    True, SFNBFormulaReading[x]];
SFNBFormulaReading[FractionBox[a_, b_, ___]] := "frac{" <> SFNBFormulaReading[a] <> "}{" <> SFNBFormulaReading[b] <> "}";
SFNBFormulaReading[SqrtBox[a_, ___]] := "sqrt{" <> SFNBFormulaReading[a] <> "}";
SFNBFormulaReading[RadicalBox[a_, b_, ___]] := "root{" <> SFNBFormulaReading[b] <> "}{" <> SFNBFormulaReading[a] <> "}";
SFNBFormulaReading[(SubscriptBox | UnderscriptBox)[a_, b_, ___]] := SFNBFormulaReading[a] <> "_{" <> SFNBFormulaReading[b] <> "}";
SFNBFormulaReading[SuperscriptBox[a_, b_, ___]] := SFNBFormulaReading[a] <> "^{" <> SFNBFormulaReading[b] <> "}";
SFNBFormulaReading[(SubsuperscriptBox | UnderoverscriptBox)[a_, b_, c_, ___]] :=
   SFNBFormulaReading[a] <> "_{" <> SFNBFormulaReading[b] <> "}^{" <> SFNBFormulaReading[c] <> "}";
(* the upper limit of a large operator is a superscript, every other overscript an accent *)
SFNBFormulaReading[OverscriptBox[a_, b_, LimitsPositioning -> True, ___]] := SFNBFormulaReading[a] <> "^{" <> SFNBFormulaReading[b] <> "}";
SFNBFormulaReading[OverscriptBox[a_, b_, ___]] := "over{" <> SFNBFormulaReading[a] <> "}{" <> SFNBFormulaReading[b] <> "}";
SFNBFormulaReading[GridBox[rows_List, ___]] := "grid[" <> StringRiffle[Map[StringRiffle[SFNBFormulaReading /@ #, "&"] &, rows], ";"] <> "]";
SFNBFormulaReading[(FormBox | TagBox | InterpretationBox)[x_, ___]] := SFNBFormulaReading[x];
SFNBFormulaReading[Cell[BoxData[x_], ___]] := SFNBFormulaReading[x];
SFNBFormulaReading[Cell[TextData[l_List], ___]] := StringJoin[SFNBFormulaReading /@ l];
SFNBFormulaReading[Cell[TextData[x_], ___]] := SFNBFormulaReading[x];
SFNBFormulaReading[Cell[x_String, ___]] := SFNBFormulaReading[x];
SFNBFormulaReading[x_] := "<?" <> ToString[Head[x]] <> ">";

End[];
EndPackage[];
