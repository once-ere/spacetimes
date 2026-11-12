#!/usr/bin/env python3
"""
nb2txt.py -- convert a Wolfram/Mathematica notebook (.nb, plain-text box form) into
readable plain text, cell by cell, WITHOUT needing Mathematica.

Usage:  python3 nb2txt.py notebook.nb out.txt [max_output_chars]

Method
  1. tokenize the notebook expression (strings, numbers, symbols, brackets);
  2. parse it into a tree  (head, [args])  /  ('list', [...])  /  ('str', s) / ('sym', s);
  3. walk Cell[...] expressions and render the typeset "boxes" as linear text:
     RowBox -> concatenation, SuperscriptBox[a,b] -> a^b, FractionBox[a,b] -> (a)/(b),
     GridBox -> {{..},{..}}, etc.
Every cell is written with its style (Title, Section, Input, Output, Text, ...) and a
running cell number, so that a statement can be cited as  "<file> cell #n".
"""
import sys, re

NAMED = {  # the named characters that occur in these notebooks -> Unicode
 'Alpha':'α','Beta':'β','Gamma':'γ','Delta':'δ','Epsilon':'ε','Zeta':'ζ','Eta':'η','Theta':'θ',
 'Iota':'ι','Kappa':'κ','Lambda':'λ','Mu':'μ','Nu':'ν','Xi':'ξ','Omicron':'ο','Pi':'π','Rho':'ρ',
 'Sigma':'σ','Tau':'τ','Upsilon':'υ','Phi':'φ','Chi':'χ','Psi':'ψ','Omega':'ω',
 'CapitalAlpha':'Α','CapitalBeta':'Β','CapitalGamma':'Γ','CapitalDelta':'Δ','CapitalTheta':'Θ',
 'CapitalLambda':'Λ','CapitalXi':'Ξ','CapitalPi':'Π','CapitalSigma':'Σ','CapitalPhi':'Φ',
 'CapitalPsi':'Ψ','CapitalOmega':'Ω','CapitalUpsilon':'Υ','CapitalChi':'Χ','CapitalEta':'Η',
 'CapitalTau':'Τ','CapitalEpsilon':'Ε','CapitalZeta':'Ζ','CapitalKappa':'Κ','CapitalMu':'Μ',
 'CapitalNu':'Ν','CapitalRho':'Ρ','CapitalIota':'Ι','CapitalOmicron':'Ο',
 'CurlyEpsilon':'ϵ','CurlyPhi':'ϕ','CurlyTheta':'ϑ','CurlyPi':'ϖ','CurlyRho':'ϱ','CurlyKappa':'ϰ',
 'FinalSigma':'ς','Digamma':'ϝ',
 'Times':'×','Cross':'⨯','CenterDot':'·','Rule':'->','RuleDelayed':':>','Equal':'==','LongEqual':'==',
 'NotEqual':'!=','LessEqual':'<=','GreaterEqual':'>=','PlusMinus':'±','MinusPlus':'∓',
 'Element':'∈','NotElement':'∉','Infinity':'∞','PartialD':'∂','Del':'∇','Sum':'Σ','Product':'∏',
 'Integral':'∫','DifferentialD':'d','ImaginaryI':'I','ImaginaryJ':'J','ExponentialE':'E',
 'Transpose':'ᵀ','Conjugate':'^*','ConjugateTranspose':'†','HermitianConjugate':'†','Dagger':'†',
 'Minus':'-','LongDash':'—','Dash':'–','Ellipsis':'…','Bullet':'•','Degree':'°',
 'LeftDoubleBracket':'[[','RightDoubleBracket':']]','LeftAngleBracket':'<','RightAngleBracket':'>',
 'LeftBracketingBar':'|','RightBracketingBar':'|','LeftDoubleBracketingBar':'||','RightDoubleBracketingBar':'||',
 'VerticalSeparator':'|','VerticalBar':'|','Therefore':'∴','Because':'∵','Implies':'=>','DoubleLeftRightArrow':'<=>',
 'RightArrow':'→','LeftArrow':'←','LeftRightArrow':'↔','UpArrow':'↑','DownArrow':'↓','LongRightArrow':'⟶',
 'DoubleRightArrow':'⇒','DoubleLongRightArrow':'⟹','Proportional':'∝','Tilde':'~','TildeTilde':'≈',
 'TildeEqual':'≃','TildeFullEqual':'≅','Congruent':'≡','NotCongruent':'≢','Subset':'⊂','Superset':'⊃',
 'SubsetEqual':'⊆','Union':'∪','Intersection':'∩','CircleTimes':'⊗','CirclePlus':'⊕','CircleDot':'⊙',
 'Wedge':'⋀','Vee':'⋁','And':'&&','Or':'||','Not':'!','ForAll':'∀','Exists':'∃','EmptySet':'∅',
 'Square':'□','Star':'⋆','SixPointedStar':'✶','Diamond':'⋄','Backslash':'\\','SmallCircle':'∘',
 'InvisibleSpace':'','InvisibleComma':',','InvisibleApplication':'@','InvisibleTimes':' ','NoBreak':'',
 'ThinSpace':' ','VeryThinSpace':'','MediumSpace':' ','ThickSpace':' ','NonBreakingSpace':' ','SpaceIndicator':' ',
 'NegativeThinSpace':'','NegativeVeryThinSpace':'','NegativeMediumSpace':'','NegativeThickSpace':'',
 'IndentingNewLine':'\n','NewLine':'\n','Continuation':'','AlignmentMarker':'','LineSeparator':'\n','ParagraphSeparator':'\n',
 'RawEscape':'','Null':'','InvisiblePrefixScriptBase':'','InvisiblePostfixScriptBase':'',
 'OpenCurlyQuote':"'",'CloseCurlyQuote':"'",'OpenCurlyDoubleQuote':'"','CloseCurlyDoubleQuote':'"',
 'DoubleStruckCapitalR':'ℝ','DoubleStruckCapitalC':'ℂ','DoubleStruckCapitalH':'ℍ','DoubleStruckCapitalO':'𝕆',
 'DoubleStruckCapitalZ':'ℤ','DoubleStruckCapitalN':'ℕ','DoubleStruckCapitalQ':'ℚ','DoubleStruckOne':'𝟙',
 'DoubleStruckCapitalI':'𝕀','DoubleStruckCapitalM':'𝕄','DoubleStruckCapitalE':'𝔼','DoubleStruckCapitalP':'ℙ',
 'ScriptCapitalL':'ℒ','ScriptCapitalH':'ℋ','ScriptCapitalD':'𝒟','ScriptCapitalO':'𝒪','ScriptL':'ℓ',
 'ScriptCapitalA':'𝒜','ScriptCapitalB':'ℬ','ScriptCapitalC':'𝒞','ScriptCapitalE':'ℰ','ScriptCapitalF':'ℱ',
 'ScriptCapitalG':'𝒢','ScriptCapitalJ':'𝒥','ScriptCapitalK':'𝒦','ScriptCapitalM':'ℳ','ScriptCapitalN':'𝒩',
 'ScriptCapitalP':'𝒫','ScriptCapitalR':'ℛ','ScriptCapitalS':'𝒮','ScriptCapitalT':'𝒯','ScriptCapitalV':'𝒱',
 'HBar':'ℏ','Angstrom':'Å','Micro':'µ','Section':'§','Paragraph':'¶','Copyright':'©','Checkmark':'✓',
 'FilledSquare':'■','EmptySquare':'□','FilledCircle':'●','EmptyCircle':'○','FilledSmallSquare':'▪',
 'FilledRightTriangle':'▶','RightTriangle':'▷','DoubleDagger':'‡','Prime':'′','DoublePrime':'″',
 'Sqrt':'√','SquareSuperset':'⊐','SquareSubset':'⊏','PrecedesEqual':'⪯','Function':'↦','UpTee':'⊥','Perpendicular':'⊥',
 'LeftCeiling':'⌈','RightCeiling':'⌉','LeftFloor':'⌊','RightFloor':'⌋','DotlessI':'ı','Sterling':'£',
 'EAcute':'é','AGrave':'à','ODoubleDot':'ö','UDoubleDot':'ü','ADoubleDot':'ä','EGrave':'è','CCedilla':'ç',
 'LeftGuillemet':'«','RightGuillemet':'»','LessLess':'≪','GreaterGreater':'≫','SuchThat':'∍','Cap':'⌢','Cup':'⌣',
 'Conditioned':'|','Distributed':'~','UndirectedEdge':'--','DirectedEdge':'->','TwoWayRule':'<->',
 'DownQuestion':'¿','FormalA':'a.','FormalX':'x.','Hacek':'ˇ','Breve':'˘','Trademark':'™','RegisteredTrademark':'®',
 'TripleDot':'⃛','DoubleDot':'¨','Vee':'∨','Nabla':'∇','LightBulb':'(i)','WarningSign':'(!)',
 'FreeformPrompt':'=','WolframAlphaPrompt':'==','AutoPlaceholder':'□','Placeholder':'□','SelectionPlaceholder':'■',
 'CapitalDifferentialD':'D','DoubledGamma':'ℽ','DoubledPi':'ℼ','Aleph':'ℵ','Bet':'ℶ','Mho':'℧','WeierstrassP':'℘',
 'GothicCapitalR':'ℜ','GothicCapitalI':'ℑ','EntityStart':'','EntityEnd':'','SpanFromLeft':'','SpanFromAbove':'','SpanFromBoth':'',
}
def unescape(s):
    """Turn the notebook-file form of a string into text."""
    s = s.replace('\\\n', '')                      # line continuation
    s = s.replace('\\<', '').replace('\\>', '')    # raw-string brackets
    def named(m):
        return NAMED.get(m.group(1), '\\[' + m.group(1) + ']')
    s = re.sub(r'\\\[([A-Za-z0-9]+)\]', named, s)
    s = re.sub(r'\\:([0-9a-fA-F]{4})', lambda m: chr(int(m.group(1), 16)), s)
    s = re.sub(r'\\\.([0-9a-fA-F]{2})', lambda m: chr(int(m.group(1), 16)), s)
    s = re.sub(r'\\\|([0-9a-fA-F]{6})', lambda m: chr(int(m.group(1), 16)), s)
    s = s.replace('\\n', '\n').replace('\\t', '\t').replace('\\"', '"').replace("\\'", "'")
    s = s.replace('\\ ', ' ').replace('\\(', '').replace('\\)', '').replace('\\!', '').replace('\\*', '')
    s = s.replace('\\\\', '\\')
    return s

TOK = re.compile(r'''
   (?P<ws>\s+)
 | (?P<str>"(?:[^"\\]|\\.|\\\n)*")
 | (?P<num>-?(?:\d+\.?\d*|\.\d+)(?:`{1,2}[-+]?\d*\.?\d*)?(?:\*\^[-+]?\d+)?)
 | (?P<sym>[A-Za-z$][A-Za-z0-9$`]*)
 | (?P<op>->|:>|\[|\]|\{|\}|,)
 | (?P<other>.)
''', re.X | re.S)

def tokenize(text):
    # strip (* ... *) comments that are OUTSIDE strings: do it on the fly
    toks = []; i = 0; n = len(text)
    while i < n:
        if text.startswith('(*', i):
            depth = 1; j = i + 2
            while j < n and depth:
                if text.startswith('(*', j): depth += 1; j += 2
                elif text.startswith('*)', j): depth -= 1; j += 2
                else: j += 1
            i = j; continue
        m = TOK.match(text, i)
        i = m.end()
        k = m.lastgroup
        if k == 'ws': continue
        toks.append((k, m.group(k)))
    return toks

class P:
    def __init__(self, toks): self.t = toks; self.i = 0
    def peek(self): return self.t[self.i] if self.i < len(self.t) else ('eof', '')
    def parse_seq(self):
        """sequence of terms up to , ] } ; returns one node"""
        items = []
        while True:
            k, v = self.peek()
            if k == 'eof' or (k == 'op' and v in (',', ']', '}')): break
            items.append(self.parse_term())
        if len(items) == 1: return items[0]
        return ('seq', items)
    def parse_args(self, close):
        args = []
        k, v = self.peek()
        if k == 'op' and v == close: self.i += 1; return args
        while True:
            args.append(self.parse_seq())
            k, v = self.peek(); self.i += 1
            if k == 'op' and v == close: break
            if k == 'eof': break
        return args
    def parse_term(self):
        k, v = self.peek(); self.i += 1
        if k == 'str': node = ('str', v[1:-1])
        elif k == 'num': node = ('num', v)
        elif k == 'sym': node = ('sym', v)
        elif k == 'op' and v == '{': node = ('list', self.parse_args('}'))
        elif k == 'op' and v in ('->', ':>'): return ('opr', v)
        else: node = ('opr', v)
        # call(s):  head[args][args]...
        while True:
            k2, v2 = self.peek()
            if k2 == 'op' and v2 == '[':
                self.i += 1
                node = ('call', node, self.parse_args(']'))
            else: break
        return node

def head(node):
    if node[0] == 'call' and node[1][0] == 'sym': return node[1][1]
    return None

def options(args):
    """collect  name -> value  rules from an argument list"""
    d = {}
    for a in args:
        if a[0] == 'seq' and len(a[1]) >= 3 and a[1][1] == ('opr', '->') and a[1][0][0] == 'sym':
            d[a[1][0][1]] = a[1][2] if len(a[1]) == 3 else ('seq', a[1][2:])
    return d

def box(node):
    """linear text of a box / text expression"""
    k = node[0]
    if k == 'str': return unescape(node[1])
    if k == 'num': return node[1]
    if k == 'sym': return node[1]
    if k == 'opr': return node[1]
    if k == 'list': return '{' + ', '.join(box(a) for a in node[1]) + '}'
    if k == 'seq':
        return ' '.join(box(a) for a in node[1])
    if k == 'call':
        h = head(node); a = node[2]
        if h == 'RowBox':
            if a and a[0][0] == 'list': return ''.join(box(x) for x in a[0][1])
            return ''.join(box(x) for x in a)
        if h in ('BoxData', 'TextData'):
            if a and a[0][0] == 'list': return ''.join(box(x) for x in a[0][1])
            return box(a[0]) if a else ''
        if h in ('StyleBox', 'TagBox', 'InterpretationBox', 'FormBox', 'TooltipBox', 'ButtonBox',
                 'PaneBox', 'FrameBox', 'ItemBox', 'AdjustmentBox', 'ErrorBox', 'PanelBox', 'Cell',
                 'HoldForm', 'DynamicBox', 'PaneSelectorBox', 'OverlayBox'):
            return box(a[0]) if a else ''
        if h == 'SuperscriptBox': return box(a[0]) + '^' + wrap(box(a[1]))
        if h == 'SubscriptBox': return box(a[0]) + '_' + wrap(box(a[1]))
        if h == 'SubsuperscriptBox': return box(a[0]) + '_' + wrap(box(a[1])) + '^' + wrap(box(a[2]))
        if h == 'OverscriptBox': return 'Over[' + box(a[0]) + ',' + box(a[1]) + ']'
        if h == 'UnderscriptBox': return 'Under[' + box(a[0]) + ',' + box(a[1]) + ']'
        if h == 'UnderoverscriptBox': return 'UnderOver[' + ','.join(box(x) for x in a[:3]) + ']'
        if h == 'FractionBox': return '(' + box(a[0]) + ')/(' + box(a[1]) + ')'
        if h == 'SqrtBox': return 'Sqrt[' + box(a[0]) + ']'
        if h == 'RadicalBox': return 'Surd[' + box(a[0]) + ',' + box(a[1]) + ']'
        if h == 'GridBox':
            rows = a[0][1] if a and a[0][0] == 'list' else []
            out = []
            for r in rows:
                if r[0] == 'list': out.append('{' + ', '.join(box(c) for c in r[1]) + '}')
                else: out.append(box(r))
            return '{' + ',\n '.join(out) + '}'
        if h == 'TemplateBox':
            name = box(a[1]) if len(a) > 1 else ''
            args = a[0][1] if a and a[0][0] == 'list' else []
            return 'T<' + name + '>[' + ', '.join(box(x) for x in args) + ']'
        if h in ('GraphicsBox', 'Graphics3DBox', 'GraphicsData', 'RasterBox', 'DynamicModuleBox', 'CompressedData'):
            return '<<' + h + '>>'
        if h == 'OutputFormData': return box(a[1]) if len(a) > 1 else box(a[0])
        # generic head[args]
        return (h or box(node[1])) + '[' + ', '.join(box(x) for x in a) + ']'
    return '?'

def wrap(s):
    return s if re.fullmatch(r'[A-Za-z0-9α-ωΑ-Ω.+\-]+', s) and not s.startswith('-') else '(' + s + ')'

def walk(node, out, counter, maxout):
    """emit every Cell (flattening CellGroupData)"""
    k = node[0]
    if k == 'list':
        for a in node[1]: walk(a, out, counter, maxout)
        return
    if k != 'call': return
    h = head(node)
    if h == 'Notebook':
        walk(node[2][0], out, counter, maxout); return
    if h == 'CellGroupData':
        walk(node[2][0], out, counter, maxout); return
    if h == 'Cell':
        a = node[2]
        if a and head(a[0]) == 'CellGroupData':
            walk(a[0], out, counter, maxout); return
        style = ''
        for x in a[1:]:
            if x[0] == 'str': style = x[1]; break
        counter[0] += 1
        txt = box(a[0]) if a else ''
        opts = options(a)
        lab = ''
        if 'CellLabel' in opts: lab = ' ' + box(opts['CellLabel'])
        tags = ''
        if 'CellTags' in opts: tags = ' tags=' + box(opts['CellTags'])
        full = len(txt)
        if style in ('Output', 'Print', 'Message', 'Echo') and maxout and len(txt) > maxout:
            txt = txt[:maxout] + '\n   ...[%d more chars]...' % (full - maxout)
        if style in ('Title', 'Subtitle', 'Chapter', 'Section', 'Subsection', 'Subsubsection', 'Subsubsubsection'):
            out.append('\n' + '#' * 70 + '\n#%d [%s] %s\n' % (counter[0], style, txt.strip()))
        else:
            out.append('\n#%d [%s]%s%s\n%s\n' % (counter[0], style, lab, tags, txt))

def main():
    src, dst = sys.argv[1], sys.argv[2]
    maxout = int(sys.argv[3]) if len(sys.argv) > 3 else 0
    text = open(src, encoding='utf-8', errors='replace').read()
    toks = tokenize(text)
    p = P(toks)
    tree = p.parse_seq()
    # the file may hold several top-level expressions; find the Notebook[...] call
    nodes = tree[1] if tree[0] == 'seq' else [tree]
    out = []; counter = [0]
    for nd in nodes:
        if head(nd) == 'Notebook': walk(nd, out, counter, maxout)
    open(dst, 'w', encoding='utf-8').write(''.join(out))
    print('%s: %d tokens, %d cells -> %s (%d chars)' % (src, len(toks), counter[0], dst, sum(len(x) for x in out)))

if __name__ == '__main__':
    sys.setrecursionlimit(100000)
    main()
