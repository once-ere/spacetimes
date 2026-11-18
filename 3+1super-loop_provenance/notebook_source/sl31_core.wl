(* ::Package:: *)
(* ------------------------------------------------------------------------------------------------
   The symbol machinery of the notebook 3+1super-loop.nb, in the context SL31` .
   It converts the TeX of a registered symbol into two-dimensional Wolfram boxes, splits the
   symbol field of a registry entry into single symbols, classifies them, and symbolizes them
   with the Notation package.
   ------------------------------------------------------------------------------------------------ *)
BeginPackage["SL31`", {"Notation`"}];

SL31TeXTokens::usage = "SL31TeXTokens[tex] splits the TeX string tex into tokens: commands such as \\Delta, braces, the scripts ^ and _, primes, and single characters.";
SL31TeXToBoxes::usage = "SL31TeXToBoxes[tex] converts the TeX of one registered symbol into the two-dimensional Wolfram boxes used in this notebook (the style of every letter is kept: double-struck, script, Gothic, sans-serif and bold letters stay different).";
SL31SplitTeX::usage = "SL31SplitTeX[tex] splits a TeX segment that lists several symbols, separated by commas outside braces, parentheses and brackets, into the single symbols; a list a, \\dots, b of numbered symbols is expanded.";
SL31Classify::usage = "SL31Classify[tex, boxes] returns the kind of a registered item: \"symbol\", \"family\", \"function\", \"letter\", \"index\", \"set\", \"group\", \"operator\", \"ket\", \"expression\".";
SL31Head::usage = "SL31Head[boxes] returns the boxes of the symbol part of a function symbol such as W(u): the boxes before the argument in parentheses.";
SL31IndexLetters::usage = "SL31IndexLetters lists the boxes that are running indices of a family of symbols: p, j, iota, k, n and their primed forms.";
SL31NameOf::usage = "SL31NameOf[boxes] is the name given to the symbol of boxes: built from the letters, the scripts and the styles, with every character that may not stand in a symbol name replaced by its character name between the separators \[UnderBracket].";
SL31Symbolize::usage = "SL31Symbolize[boxes] symbolizes boxes with the Notation package in StandardForm (WorkingForm -> StandardForm given explicitly) and returns the new symbol, held: HoldComplete[symbol].";
SL31SymbolOf::usage = "SL31SymbolOf[boxes] returns HoldComplete[s], where s is the expression that the boxes parse to in StandardForm (for a symbolized box: the atomic symbol).";
SL31SymbolBoxes::usage = "SL31SymbolBoxes[tex] returns the boxes that are symbolized for the registered TeX item tex (see the comments of its definition for the wrapping by kind).";
SL31Symbolized::usage = "SL31Symbolized lists the kinds of registered items that are symbolized.";
SL31Instance::usage = "SL31Instance[boxes, {index -> value, ...}] replaces running indices in the scripts of boxes by values (strings), symbolizes the resulting component box and returns it, held.";
LL::usage = "LL[tex] returns the symbolized symbol of the registered TeX symbol tex; LL[tex, index -> value, ...] returns the symbolized component with the given index values.";
SL31`Build`symbolBoxes::usage = "SL31`Build`symbolBoxes[tex, index -> value, ...] returns the boxes of the registered symbol tex or of its component; used by the notebook builder.";

Begin["`Private`"];

(* ---------- 1. Tokens ---------- *)
SL31TeXTokens[s_String] := StringCases[s, {
    RegularExpression["\\\\[A-Za-z]+"],      (* a command such as \Delta or \mathbf *)
    RegularExpression["\\\\[,;:! ]"],        (* a TeX spacing command: dropped later *)
    "''", "'",                               (* primes *)
    RegularExpression["\\s+"],               (* white space: dropped later *)
    RegularExpression["."]}];                (* any other single character *)

(* ---------- 2. Characters for the commands ---------- *)
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

(* the styled letters: every style gives a box that the Notation package does NOT strip, so that
   letters of different styles stay different symbols *)
styled["\\mathbb", x_String] := StringJoin[ToExpression["\"\\[DoubleStruckCapital" <> # <> "]\""] & /@ Characters[x]];
styled["\\mathcal", x_String] := StringJoin[ToExpression["\"\\[ScriptCapital" <> # <> "]\""] & /@ Characters[x]];
styled["\\mathfrak", x_String] := StringJoin[If[UpperCaseQ[#],
     ToExpression["\"\\[GothicCapital" <> # <> "]\""], ToExpression["\"\\[Gothic" <> ToUpperCase[#] <> "]\""]] & /@ Characters[x]];
styled["\\mathsf", x_] := TagBox[StyleBox[x, FontFamily -> "Helvetica", FontSlant -> "Plain"], "SL31SansSerif"];
styled["\\mathbf", x_] := TagBox[StyleBox[x, FontWeight -> "Bold", FontSlant -> "Plain"], "SL31Bold"];
styled["\\mathrm", x_] := x;
styleCommands = {"\\mathbb", "\\mathcal", "\\mathfrak", "\\mathsf", "\\mathbf", "\\mathrm"};
accent = <|"\\vec" -> "\[RightVector]", "\\hat" -> "^", "\\tilde" -> "~", "\\bar" -> "_"|>;

(* the boxes of a piece of roman text, as the front end makes them.  When a notebook cell is
   evaluated, the front end first reads every string of its boxes again as typed text and cuts it
   into tokens: names (a letter followed by letters and digits), numbers (digits), and single
   other characters; a part in parentheses becomes a RowBox of its own.  The string "int,F" is
   therefore sent to the kernel as RowBox[{"int", ",", "F"}], and "s(2)" as
   RowBox[{"s", RowBox[{"(", "2", ")"}]}].  The Notation package recognizes an input only if its
   boxes are EXACTLY the symbolized boxes, so the reader must build these same boxes; a single
   name such as "vac" or "LL" stays one string. *)
plainBoxes[s_String] := Module[{toks, j = 1, m, group, single},
  toks = StringCases[s, {
     w : (LetterCharacter ~~ (LetterCharacter | DigitCharacter) ...) :> w,
     d : (DigitCharacter ..) :> d,
     c : Except[WhitespaceCharacter] :> c}];
  m = Length[toks];
  single[items_List] := Which[items === {}, "", Length[items] == 1, First[items], True, RowBox[items]];
  (* the items up to a closing parenthesis or the end; a part in parentheses becomes one item *)
  group[] := Module[{items = {}},
    While[j <= m && toks[[j]] =!= ")",
      If[toks[[j]] === "(",
        j++;
        With[{inner = group[]}, If[j <= m, j++]; AppendTo[items, RowBox[{"(", single[inner], ")"}]]],
        AppendTo[items, toks[[j]]]; j++]];
    items];
  single[group[]]];

(* ---------- 3. The parser (recursive descent over the token list) ---------- *)
SL31TeXToBoxes[tex_String] := Module[
  {t, i = 1, n, peek, next, seq, atom, scriptArg, withScripts, plainText, merge, regroup},
  t = Select[SL31TeXTokens[tex],
    ! (StringMatchQ[#, RegularExpression["\\s+"]] || MemberQ[{"\\,", "\;", "\\:", "\\!", "\\ "}, #]) &];
  n = Length[t];
  peek[] := If[i <= n, t[[i]], None];
  next[] := (i++; t[[i - 1]]);
  (* the text of a group in roman type, {\rm ...}: its characters are joined and then cut into
     tokens exactly as the front end cuts typed text (see plainBoxes above) *)
  plainText[] := Module[{s = ""},
    While[i <= n && peek[] =!= "}",
      With[{k = next[]},
        s = s <> Which[
          k === "'", "\[Prime]", k === "''", "\[Prime]\[Prime]",
          KeyExistsQ[cmd, k], cmd[k],
          StringStartsQ[k, "\\"], StringDrop[k, 1],
          True, k]]];
    plainBoxes[s]];
  (* merge neighbouring digits into one number string *)
  merge[items_List] := Module[{r = SequenceReplace[items,
        {a_String, b_String} /; StringMatchQ[a, DigitCharacter ..] && StringMatchQ[b, DigitCharacter ..] :> a <> b]},
    If[r === items, r, merge[r]]];
  (* a script written after a closing parenthesis, as in (T_LL)^2, belongs to the whole group in
     parentheses: the front end stores (T_LL)^2 as SuperscriptBox[RowBox[{"(", T_LL, ")"}], "2"],
     and the reader builds the same box *)
  regroup[items_List] := Module[{out = {}, stack = {}},
    Do[Which[
        it === "(", AppendTo[out, it]; AppendTo[stack, Length[out]],
        it === ")", If[stack =!= {}, stack = Most[stack]]; AppendTo[out, it],
        MatchQ[it, (SubscriptBox | SuperscriptBox | SubsuperscriptBox)[")", __]] && stack =!= {},
          With[{j = Last[stack]},
            stack = Most[stack];
            out = Append[Take[out, j - 1], ReplacePart[it, 1 -> RowBox[Append[Drop[out, j - 1], ")"]]]]],
        True, AppendTo[out, it]],
      {it, items}];
    out];
  (* a sequence of atoms up to "}" or the end *)
  seq[] := Module[{items = {}, a},
    While[i <= n && peek[] =!= "}",
      If[peek[] === "\\rm",
        next[]; AppendTo[items, plainText[]],
        a = atom[]; If[a =!= Null, AppendTo[items, a]]]];
    items = regroup[merge[items]];
    Which[items === {}, "", Length[items] == 1, First[items], True, RowBox[items]]];
  (* the argument of a script, an accent or a style: a group or a single token *)
  scriptArg[] := Module[{k = next[], r},
    Which[
      k === "{", r = seq[]; next[]; r,
      KeyExistsQ[cmd, k], cmd[k],
      MemberQ[styleCommands, k], styled[k, Replace[scriptArg[], RowBox[l_] :> StringJoin[l]]],
      True, k]];
  (* an atom without its scripts *)
  atom[] := Module[{k = next[], base},
    base = Which[
      k === "{", With[{r = seq[]}, next[]; r],
      MemberQ[styleCommands, k], styled[k, Replace[scriptArg[], RowBox[l_] :> StringJoin[l]]],
      KeyExistsQ[accent, k], OverscriptBox[scriptArg[], accent[k]],
      KeyExistsQ[cmd, k], cmd[k],
      StringStartsQ[k, "\\"], StringDrop[k, 1],
      True, k];
    withScripts[base]];
  (* primes, superscripts and subscripts after a base *)
  withScripts[base_] := Module[{sub = None, sup = None, primes = "", go = True},
    While[go && i <= n,
      Which[
        peek[] === "'", next[]; primes = primes <> "\[Prime]",
        peek[] === "''", next[]; primes = primes <> "\[Prime]\[Prime]",
        peek[] === "^", next[]; sup = scriptArg[],
        peek[] === "_", next[]; sub = scriptArg[],
        True, go = False]];
    If[primes =!= "", sup = If[sup === None, primes, RowBox[{primes, sup}]]];
    Which[
      sub === None && sup === None, base,
      sup === None, SubscriptBox[base, sub],
      sub === None, SuperscriptBox[base, sup],
      True, SubsuperscriptBox[base, sub, sup]]];
  seq[]];

(* ---------- 4. Splitting a TeX segment into single symbols ---------- *)
SL31SplitTeX[tex_String] := Module[{chars = Characters[tex], depth = 0, cur = "", out = {}, items, expand},
  Do[Which[
      MemberQ[{"{", "(", "["}, c], depth++; cur = cur <> c,
      MemberQ[{"}", ")", "]"}, c], depth--; cur = cur <> c,
      c === "," && depth == 0, AppendTo[out, StringTrim[cur]]; cur = "",
      True, cur = cur <> c],
    {c, chars}];
  AppendTo[out, StringTrim[cur]];
  items = Select[out, # =!= "" &];
  (* a, \dots, b with a and b differing in one integer: expand *)
  expand[{a_String, "\\dots", b_String}] := Module[{na = StringCases[a, DigitCharacter ..], nb = StringCases[b, DigitCharacter ..], pos},
    pos = Flatten[Position[MapThread[Equal, {na, nb}], False]];
    If[Length[na] == Length[nb] && Length[pos] == 1,
      Table[StringReplacePart[a, ToString[v], Last[StringPosition[a, na[[First[pos]]]]]],
        {v, ToExpression[na[[First[pos]]]], ToExpression[nb[[First[pos]]]]}],
      {a, "\\dots", b}]];
  If[Length[items] == 3 && items[[2]] === "\\dots", expand[items], items]];

(* ---------- 5. Kinds ---------- *)
SL31IndexLetters = {"p", "j", "\[Iota]", "k", "n",
   SuperscriptBox["p", "\[Prime]"], SuperscriptBox["p", "\[Prime]\[Prime]"],
   SuperscriptBox["j", "\[Prime]"], SuperscriptBox["j", "\[Prime]\[Prime]"],
   SuperscriptBox["\[Iota]", "\[Prime]"], SuperscriptBox["\[Iota]", "\[Prime]\[Prime]"]};

(* the script parts of a box (subscripts and superscripts, not the base) *)
scriptsOf[SubscriptBox[b_, s_]] := Join[{s}, scriptsOf[b]];
scriptsOf[SuperscriptBox[b_, s_]] := Join[{s}, scriptsOf[b]];
scriptsOf[SubsuperscriptBox[b_, s1_, s2_]] := Join[{s1, s2}, scriptsOf[b]];
scriptsOf[OverscriptBox[b_, _]] := scriptsOf[b];
scriptsOf[_] := {};
indexIn[s_] := ! FreeQ[s /. RowBox[l_] :> l, Alternatives @@ SL31IndexLetters];
hasRunningIndex[b_] := AnyTrue[scriptsOf[b], indexIn];

(* the head of a function symbol: RowBox[{head, "(", args, ")"}] *)
SL31Head[RowBox[{h_, "(", ___, ")"}]] := h;
SL31Head[b_] := b;

SL31Classify[tex_String, b_] := Which[
  MemberQ[{"i"}, b], "imaginary unit",
  MemberQ[{"p", "j", "\[Iota]", "k", "n"}, b], "index",
  MemberQ[SL31IndexLetters, b], "primed index",
  MemberQ[{"\[DoubleStruckCapitalR]", "\[DoubleStruckCapitalC]", "\[DoubleStruckCapitalH]", "\[DoubleStruckCapitalO]"}, b], "set",
  (* the operators are tested before the single letters: the tensor product, the direct sum and
     the composition are single characters too *)
  MemberQ[{"tr", "det", "diag", "Re", "Im", "\[CircleTimes]", "\[CirclePlus]", "\[SmallCircle]"}, b], "operator",
  StringQ[b] && StringLength[b] == 1, "letter",
  MatchQ[b, SuperscriptBox["", "T"] | RowBox[{"\[LeftAngleBracket]", "\[RightAngleBracket]"}] | RowBox[{"[", "\[CenterDot]", "]"}]], "operator",
  MatchQ[b, RowBox[{"|", __, "\[RightAngleBracket]"}]], "ket",
  MatchQ[b, RowBox[{"[", __, "]"}]], "operator",
  StringStartsQ[tex, {"SL(", "SO(", "SU(", "Spin(", "U(", "O(", "\\mathrm G_2", "\\mathfrak{"}], "group",
  (* a group in parentheses with a script, such as (T_LL)^2: an expression, not a power of T_LL *)
  MatchQ[b, (SubscriptBox | SuperscriptBox | SubsuperscriptBox)[RowBox[{"(", ___, ")"}], __]], "expression",
  MatchQ[b, RowBox[{_, "(", ___, ")"}]], If[hasRunningIndex[SL31Head[b]], "function family", "function"],
  MatchQ[b, RowBox[_]], "expression",
  hasRunningIndex[b], "family",
  True, "symbol"];

(* ---------- 6. Symbolization (the technique of the Notation package) ---------- *)
(* The option WorkingForm -> StandardForm is given explicitly: by default the Notation package
   takes the working form from the notebook that holds the input (InputNotebook[]); when the
   notebook is evaluated without a selected input notebook this gives $Failed, and the parsing
   rule would then be stored for the form $Failed and never used. *)
(* The name of the new symbol.  The Notation package would name a symbolized box after its
   letters and its scripts only (it removes StyleBox and TagBox wrappers when it builds the name),
   so that a bold, a sans-serif and an ordinary D with the same scripts would all receive the same
   name and become ONE symbol.  The name built here records the style as well, and replaces every
   character that may not stand in a symbol name by its character name. *)
charName[c_String] := Replace[Quiet[CharacterName[c]], Except[_String] :> "U" <> IntegerString[First[ToCharacterCode[c]], 16]];
safeChars[s_String] := StringJoin[If[LetterQ[#] || DigitQ[#], #, "\[UnderBracket]" <> charName[#] <> "\[UnderBracket]"] & /@ Characters[s]];
SL31NameOf[b_] := Module[{parts, name},
  parts = Flatten[{b} //. {
     TagBox[StyleBox[x_, ___], "SL31Bold", ___] :> {"Bold\[UnderBracket]", x},
     TagBox[StyleBox[x_, ___], "SL31SansSerif", ___] :> {"SansSerif\[UnderBracket]", x},
     TagBox[x_, "SL31Plain", ___] :> {"Plain\[UnderBracket]", x},
     TagBox[x_, "SL31Name", ___] :> {"Name\[UnderBracket]", x},
     TagBox[x_, ___] :> x, StyleBox[x_, ___] :> x,
     RowBox[l_List] :> l,
     SubscriptBox[x_, c_] :> {x, "\[UnderBracket]Sub\[UnderBracket]", c},
     SuperscriptBox[x_, c_] :> {x, "\[UnderBracket]Sup\[UnderBracket]", c},
     SubsuperscriptBox[x_, c_, d_] :> {x, "\[UnderBracket]Sub\[UnderBracket]", c, "\[UnderBracket]Sup\[UnderBracket]", d},
     OverscriptBox[x_, c_] :> {x, "\[UnderBracket]Over\[UnderBracket]", c}}];
  name = StringJoin[Map[If[MemberQ[{"\[UnderBracket]Sub\[UnderBracket]", "\[UnderBracket]Sup\[UnderBracket]",
       "\[UnderBracket]Over\[UnderBracket]", "Bold\[UnderBracket]", "SansSerif\[UnderBracket]", "Plain\[UnderBracket]",
       "Name\[UnderBracket]"}, #], #, safeChars[#]] &, parts]];
  If[LetterQ[StringTake[name, 1]], name, "Sym" <> name]];

(* The option WorkingForm -> StandardForm is given explicitly: by default the Notation package
   takes the working form from the notebook that holds the input (InputNotebook[]); when the
   notebook is evaluated without a selected input notebook this gives $Failed, and the parsing
   rule would then be stored for the form $Failed and never used. *)
SL31Symbolize[b_] := (Notation`Symbolize[Notation`ParsedBoxWrapper[b], Notation`WorkingForm -> StandardForm,
     Notation`SymbolizeRootName -> SL31NameOf[b]];
   SL31SymbolOf[b]);
SL31SymbolOf[b_] := MakeExpression[b, StandardForm];

(* components: replace running indices in the scripts by values *)
replaceIndex[b_, rules_List] := Module[{r = Map[(#[[1]] -> #[[2]]) &, rules]},
  b /. {SubscriptBox[x_, s_] :> SubscriptBox[x, s /. r], SuperscriptBox[x_, s_] :> SuperscriptBox[x, s /. r],
        SubsuperscriptBox[x_, s1_, s2_] :> SubsuperscriptBox[x, s1 /. r, s2 /. r]}];
SL31Instance[b_, rules_List] := SL31Symbolize[replaceIndex[b, rules]];

(* the boxes that are symbolized for a registered TeX item, by kind:
   - a single letter (D, the dimension count) is wrapped in TagBox[.., "SL31Plain"], so that the
     ordinary letter (for example the derivative D of the Wolfram Language) keeps its meaning;
   - a group name, a ket or another expression whose boxes begin with RowBox is wrapped in
     TagBox[.., "SL31Name"], because the Notation package cannot symbolize a RowBox;
   - a function symbol W(u) gives the boxes of its head W;
   - every other kind gives its boxes unchanged. *)
SL31Symbolized = {"symbol", "family", "function", "function family", "letter", "group", "ket", "expression", "primed index"};
(* the innermost base of a box with scripts or an accent *)
baseOf[(SubscriptBox | SuperscriptBox | SubsuperscriptBox | OverscriptBox)[x_, __]] := baseOf[x];
baseOf[x_] := x;
(* a box whose base is an operator sign, such as the partial-derivative sign or nabla, would be
   read by the front end as an operator applied to what follows it *)
operatorBaseQ[b_] := With[{x = baseOf[b]}, StringQ[x] && x =!= "" && ! LetterQ[StringTake[x, 1]] && ! DigitQ[StringTake[x, 1]]];
(* TagBox[.., "SL31Name", SyntaxForm -> "symbol"] makes the front end treat its content as ONE
   operand, like a symbol, wherever it stands *)
asName[b_] := TagBox[b, "SL31Name", SyntaxForm -> "symbol"];
SL31SymbolBoxes[tex_String] := Module[{b = SL31TeXToBoxes[tex], k, r},
  k = SL31Classify[tex, b];
  r = Switch[k,
    "letter", TagBox[b, "SL31Plain"],
    "expression", asName[b],
    "group" | "ket", If[Head[b] === RowBox, asName[b], b],
    "function" | "function family", SL31Head[b],
    _, b];
  If[Head[r] =!= TagBox && operatorBaseQ[r], asName[r], r]];

(* the boxes of a registered TeX symbol, with optional index values given as TeX strings *)
SL31`Build`symbolBoxes[tex_String, rules___Rule] := Module[{b = SL31SymbolBoxes[tex]},
  If[{rules} === {}, b, replaceIndex[b, Map[SL31TeXToBoxes[#[[1]]] -> SL31TeXToBoxes[#[[2]]] &, {rules}]]]];
LL[tex_String, rules___Rule] := ReleaseHold[SL31SymbolOf[SL31`Build`symbolBoxes[tex, rules]]];

End[];
EndPackage[];
