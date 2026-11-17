//! A small plotting library that writes Scalable Vector Graphics (SVG): figures with panels, linear or logarithmic axes
//! with readable tick values, lines, markers, colour maps, shaded bands, legends and labels with superscripts and
//! subscripts.  Pure Rust; the SVG text is built by hand.
//!
//! Coordinates: SVG measures in pixels from the upper left corner of the figure, with y growing downwards.  A panel
//! (`Axes`) occupies the rectangle (x0, y0, w, h) of the figure and maps a data point (x, y) to the pixel
//! (x0 + w (x - xmin)/(xmax - xmin), y0 + h (1 - (y - ymin)/(ymax - ymin))); on a logarithmic axis log10(y) replaces y.
//! Everything a panel draws goes into its `body` (SVG elements) and is clipped to its rectangle when the figure is written.

/// the colours of the figures of the document of Test 2 (the same as in the Python plotter)
pub const BLUE: &str = "#2a78d6";
pub const ORANGE: &str = "#eb6834";
pub const AQUA: &str = "#1baf7a";
pub const RED: &str = "#e34948";
pub const GREY: &str = "#8a8984";
pub const INK: &str = "#1f1f1f";
pub const MUTED: &str = "#5f5e5a";
pub const GRID: &str = "#e6e5e0";
/// four steps of one hue, from earlier to later times
pub const SEQ: [&str; 4] = ["#9ec5f2", "#5c9ce6", "#2a78d6", "#174d91"];

/// escapes the three characters with a special meaning in SVG text
fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// a label with the markup x^{1}_{LL}: the text in braces after ^ is raised, after _ lowered (smaller type)
/// (SVG text spans, `tspan`, with a vertical shift `dy` in em; the shift is undone before the following normal text)
pub fn markup(s: &str) -> String {
    let mut out = String::new();
    let ch: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut shift = 0.0f64; // the current vertical offset in em
    while i < ch.len() {
        if (ch[i] == '^' || ch[i] == '_') && i + 1 < ch.len() && ch[i + 1] == '{' {
            let up = ch[i] == '^';
            let mut j = i + 2;
            while j < ch.len() && ch[j] != '}' {
                j += 1;
            }
            let inner: String = ch[i + 2..j].iter().collect();
            let target = if up { -0.45 } else { 0.3 };
            // a superscript directly followed by a subscript is placed above it (the subscript moves back)
            out += &format!("<tspan dy=\"{:.2}em\" font-size=\"70%\">{}</tspan>", (target - shift) / 0.7, esc(&inner));
            shift = target;
            i = j + 1;
            continue;
        }
        if shift != 0.0 {
            out += &format!("<tspan dy=\"{:.2}em\">", -shift);
            shift = 0.0;
            let mut j = i;
            while j < ch.len() && !((ch[j] == '^' || ch[j] == '_') && j + 1 < ch.len() && ch[j + 1] == '{') {
                j += 1;
            }
            let plain: String = ch[i..j].iter().collect();
            out += &esc(&plain);
            out += "</tspan>";
            i = j;
            continue;
        }
        out += &esc(&ch[i].to_string());
        i += 1;
    }
    out
}

/// the approximate width of a label in pixels (for placing legends), for the font size fs: 0.56 fs per normal character and
/// 0.4 fs per raised or lowered character (the mean widths of the sans-serif fonts the figures use)
pub fn text_width(s: &str, fs: f64) -> f64 {
    let mut w = 0.0;
    let mut small = false;
    let mut depth = 0;
    let ch: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < ch.len() {
        if (ch[i] == '^' || ch[i] == '_') && i + 1 < ch.len() && ch[i + 1] == '{' {
            small = true;
            depth += 1;
            i += 2;
            continue;
        }
        if ch[i] == '}' && depth > 0 {
            depth -= 1;
            small = depth > 0;
            i += 1;
            continue;
        }
        w += fs * if small { 0.4 } else { 0.56 };
        i += 1;
    }
    w
}

/// tick values: about n readable numbers between lo and hi (steps 1, 2, 2.5 or 5 times a power of 10): the raw step
/// (hi - lo)/n is rounded up to the next such number, and the ticks are its multiples inside [lo, hi]
pub fn nice_ticks(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    if !(hi > lo) {
        return vec![lo];
    }
    let raw = (hi - lo) / n as f64;
    let p = 10f64.powf(raw.log10().floor());
    let step = [1.0, 2.0, 2.5, 5.0, 10.0].iter().map(|m| m * p).find(|&s| s >= raw).unwrap_or(10.0 * p);
    let start = (lo / step).ceil() * step;
    let mut t = vec![];
    let mut v = start;
    while v <= hi + 1e-9 * step {
        t.push(if v.abs() < 1e-12 * step { 0.0 } else { v });
        v += step;
    }
    t
}

/// a tick label: no needless digits; numbers of magnitude below 1e-3 or from 1e4 on as a times 10^{e} (or 10^{e})
pub fn tick_label(v: f64) -> String {
    if v == 0.0 {
        return "0".into();
    }
    let a = v.abs();
    if a >= 1e4 || a < 1e-3 {
        let e = a.log10().floor() as i32;
        let m = v / 10f64.powi(e);
        if (m - m.round()).abs() < 1e-9 && m.round().abs() == 1.0 {
            return format!("{}10^{{{}}}", if m < 0.0 { "-" } else { "" }, e);
        }
        return format!("{}·10^{{{}}}", trim(m, 3), e);
    }
    trim(v, 6)
}

/// v with the given number of decimals, trailing zeros (and a trailing decimal point) removed, "-0" written as "0"
fn trim(v: f64, digits: usize) -> String {
    let s = format!("{:.*}", digits, v);
    let s = if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s };
    if s == "-0" {
        "0".into()
    } else {
        s
    }
}

/// a legend entry: a line (colour, width, dash pattern, which may be empty) or a marker
pub enum Key {
    Line(String, f64, String),
    Dot(String),
}

/// one panel: its place on the figure (pixels), its data ranges and its contents (SVG elements)
pub struct Axes {
    pub x0: f64,                      // left edge of the plotting area (pixels)
    pub y0: f64,                      // upper edge of the plotting area (pixels)
    pub w: f64,                       // width (pixels)
    pub h: f64,                       // height (pixels)
    pub xlim: (f64, f64),             // the data range of the horizontal axis
    pub ylim: (f64, f64),             // the data range of the vertical axis (positive if logy)
    pub logy: bool,                   // logarithmic vertical axis
    pub body: String,                 // the SVG elements of the data (lines, dots, colour maps)
    pub legend: Vec<(Key, String)>,   // the legend entries (key, label with markup)
    pub legend_at: (f64, f64),        // the lower left corner of the legend box, as fractions of (w, h) from the lower left
    pub title: String,                // the panel title, above the panel
    pub xlabel: String,               // the label of the horizontal axis
    pub ylabel: String,               // the label of the vertical axis (rotated)
    pub grid: bool,                   // draw grid lines at the ticks
}

impl Axes {
    /// an empty panel at (x0, y0) of the size (w, h), with the ranges (0, 1)
    pub fn new(x0: f64, y0: f64, w: f64, h: f64) -> Axes {
        Axes {
            x0,
            y0,
            w,
            h,
            xlim: (0.0, 1.0),
            ylim: (0.0, 1.0),
            logy: false,
            body: String::new(),
            legend: vec![],
            legend_at: (0.02, 0.02),
            title: String::new(),
            xlabel: String::new(),
            ylabel: String::new(),
            grid: true,
        }
    }
    /// the value used for the vertical position: y, or log10(y) on a logarithmic axis (non-positive values are pushed far
    /// below the panel, where the clipping hides them)
    fn yv(&self, y: f64) -> f64 {
        if self.logy {
            y.max(1e-300).log10()
        } else {
            y
        }
    }
    /// the vertical range in the same transformed values
    fn ylim_t(&self) -> (f64, f64) {
        if self.logy {
            (self.ylim.0.log10(), self.ylim.1.log10())
        } else {
            self.ylim
        }
    }
    /// the pixel position of a data point
    pub fn px(&self, x: f64, y: f64) -> (f64, f64) {
        let (y0, y1) = self.ylim_t();
        let u = (x - self.xlim.0) / (self.xlim.1 - self.xlim.0);
        let v = (self.yv(y) - y0) / (y1 - y0);
        (self.x0 + u * self.w, self.y0 + (1.0 - v) * self.h)
    }
    /// set the ranges from the data, with a margin of 4 per cent (log: from the positive values)
    pub fn autoscale(&mut self, xs: &[f64], ys: &[f64]) {
        let fin = |v: &[f64]| -> (f64, f64) {
            let f: Vec<f64> = v.iter().cloned().filter(|z| z.is_finite()).collect();
            (f.iter().cloned().fold(f64::INFINITY, f64::min), f.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
        };
        let (a, b) = fin(xs);
        self.xlim = (a, if b > a { b } else { a + 1.0 });
        if self.logy {
            let p: Vec<f64> = ys.iter().cloned().filter(|&z| z > 0.0 && z.is_finite()).collect();
            let lo = p.iter().cloned().fold(f64::INFINITY, f64::min).log10().floor();
            let hi = p.iter().cloned().fold(f64::NEG_INFINITY, f64::max).log10().ceil();
            self.ylim = (10f64.powf(lo), 10f64.powf(if hi > lo { hi } else { lo + 1.0 }));
        } else {
            let (c, d) = fin(ys);
            let m = if d > c { 0.04 * (d - c) } else { 0.5 };
            self.ylim = (c - m, d + m);
        }
    }
    /// a polyline through the points (xs, ys); a point that is not finite (or not positive on a log axis) lifts the pen, so
    /// the line is broken there; `dash` is an SVG dash pattern ("5,3": 5 pixels drawn, 3 left out) or empty for a solid line
    pub fn line(&mut self, xs: &[f64], ys: &[f64], color: &str, width: f64, dash: &str) {
        let mut d = String::new();
        let mut pen = false;
        for (x, y) in xs.iter().zip(ys) {
            if !x.is_finite() || !y.is_finite() || (self.logy && *y <= 0.0) {
                pen = false;
                continue;
            }
            let (px, py) = self.px(*x, *y);
            d += &format!("{}{:.2},{:.2} ", if pen { "L" } else { "M" }, px, py);
            pen = true;
        }
        let da = if dash.is_empty() { String::new() } else { format!(" stroke-dasharray=\"{}\"", dash) };
        self.body += &format!("<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" stroke-linejoin=\"round\"{}/>\n", d, color, width, da);
    }
    /// a filled circle of radius r pixels at each finite point
    pub fn dots(&mut self, xs: &[f64], ys: &[f64], color: &str, r: f64) {
        for (x, y) in xs.iter().zip(ys) {
            if !x.is_finite() || !y.is_finite() {
                continue;
            }
            let (px, py) = self.px(*x, *y);
            self.body += &format!("<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{}\" fill=\"{}\"/>\n", px, py, r, color);
        }
    }
    /// a horizontal reference line at the value y across the panel
    pub fn hline(&mut self, y: f64, color: &str, width: f64, dash: &str) {
        let (a, b) = self.xlim;
        self.line(&[a, b], &[y, y], color, width, dash);
    }
    /// a vertical reference line at the value x across the panel
    pub fn vline(&mut self, x: f64, color: &str, width: f64, dash: &str) {
        let (a, b) = self.ylim;
        self.line(&[x, x], &[a, b], color, width, dash);
    }
    /// a shaded horizontal band between the values y0 and y1 (drawn first, under the data)
    pub fn hspan(&mut self, y0: f64, y1: f64, color: &str) {
        let (p0, q0) = self.px(self.xlim.0, y1);
        let (p1, q1) = self.px(self.xlim.1, y0);
        self.body += &format!("<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"{}\"/>\n", p0, q0, p1 - p0, q1 - q0, color);
    }
    /// a colour map of the values z[row][col] over the rectangle [x0, x1] x [y0, y1] (white to dark blue)
    pub fn heatmap(&mut self, z: &[Vec<f64>], ext: (f64, f64, f64, f64), zmax: f64) {
        let ny = z.len();
        let nx = if ny > 0 { z[0].len() } else { 0 };
        // each value fills a cell centred on its grid point; the cells overlap by 0.4 pixel so that no white seams appear
        let dxc = (ext.1 - ext.0) / (nx.max(2) - 1) as f64;
        let dyc = (ext.3 - ext.2) / (ny.max(2) - 1) as f64;
        for (r, row) in z.iter().enumerate() {
            for (cc, &v) in row.iter().enumerate() {
                let xm = ext.0 + cc as f64 * dxc;
                let ym = ext.2 + r as f64 * dyc;
                let (pa, qa) = self.px(xm - 0.5 * dxc, ym + 0.5 * dyc);
                let (pb, qb) = self.px(xm + 0.5 * dxc, ym - 0.5 * dyc);
                self.body += &format!(
                    "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"{}\"/>\n",
                    pa,
                    qa,
                    (pb - pa).abs() + 0.4,
                    (qb - qa).abs() + 0.4,
                    blues(if zmax > 0.0 { v / zmax } else { 0.0 })
                );
            }
        }
    }
    /// adds a legend entry
    pub fn add_key(&mut self, k: Key, label: &str) {
        self.legend.push((k, label.to_string()));
    }

    /// the SVG of the panel: frame, grid, ticks, contents (clipped), labels, title and legend
    pub fn render(&self, id: usize) -> String {
        let mut s = String::new();
        let fs = 12.0;
        s += &format!("<clipPath id=\"c{}\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath>\n", id, self.x0, self.y0, self.w, self.h);
        // ticks and grid
        let xt = nice_ticks(self.xlim.0, self.xlim.1, (self.w / 90.0).max(3.0) as usize);
        for &t in &xt {
            let (px, _) = self.px(t, self.ylim.0);
            if self.grid {
                s += &format!("<line x1=\"{:.2}\" y1=\"{}\" x2=\"{:.2}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.7\"/>\n", px, self.y0, px, self.y0 + self.h, GRID);
            }
            s += &format!("<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"{}\" fill=\"{}\" text-anchor=\"middle\">{}</text>\n", px, self.y0 + self.h + 15.0, fs - 1.0, MUTED, markup(&tick_label(t)));
        }
        let yt: Vec<f64> = if self.logy {
            let (a, b) = self.ylim_t();
            let step = ((b - a) / 6.0).ceil().max(1.0) as i32;
            (a as i32..=b as i32).step_by(step as usize).map(|e| 10f64.powi(e)).collect()
        } else {
            nice_ticks(self.ylim.0, self.ylim.1, (self.h / 55.0).max(3.0) as usize)
        };
        for &t in &yt {
            let (_, py) = self.px(self.xlim.0, t);
            if self.grid {
                s += &format!("<line x1=\"{}\" y1=\"{:.2}\" x2=\"{}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"0.7\"/>\n", self.x0, py, self.x0 + self.w, py, GRID);
            }
            s += &format!("<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"{}\" fill=\"{}\" text-anchor=\"end\">{}</text>\n", self.x0 - 5.0, py + 4.0, fs - 1.0, MUTED, markup(&tick_label(t)));
        }
        s += &format!("<g clip-path=\"url(#c{})\">\n{}</g>\n", id, self.body);
        // the frame (left and bottom axes)
        s += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n", self.x0, self.y0 + self.h, self.x0 + self.w, self.y0 + self.h, MUTED);
        s += &format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"1\"/>\n", self.x0, self.y0, self.x0, self.y0 + self.h, MUTED);
        s += &format!("<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"{}\" fill=\"{}\" text-anchor=\"middle\">{}</text>\n", self.x0 + 0.5 * self.w, self.y0 + self.h + 34.0, fs, INK, markup(&self.xlabel));
        let (yx, yy) = (self.x0 - 52.0, self.y0 + 0.5 * self.h);
        s += &format!("<text transform=\"translate({:.2},{:.2}) rotate(-90)\" font-size=\"{}\" fill=\"{}\" text-anchor=\"middle\">{}</text>\n", yx, yy, fs, INK, markup(&self.ylabel));
        s += &format!("<text x=\"{}\" y=\"{}\" font-size=\"{}\" fill=\"{}\">{}</text>\n", self.x0, self.y0 - 8.0, fs + 0.5, INK, markup(&self.title));
        // the legend: a white box with the entries, at the given fraction of the panel (from the lower left)
        if !self.legend.is_empty() {
            let lh = 16.0;
            let wmax = self.legend.iter().map(|(_, l)| text_width(l, fs - 1.5)).fold(0.0, f64::max) + 34.0;
            let hbox = lh * self.legend.len() as f64 + 6.0;
            let bx = self.x0 + self.legend_at.0 * self.w;
            let by = self.y0 + (1.0 - self.legend_at.1) * self.h - hbox;
            s += &format!("<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"white\" fill-opacity=\"0.85\" stroke=\"none\"/>\n", bx, by, wmax, hbox);
            for (k, (key, label)) in self.legend.iter().enumerate() {
                let yk = by + 4.0 + lh * (k as f64 + 0.5);
                match key {
                    Key::Line(col, w, dash) => {
                        let da = if dash.is_empty() { String::new() } else { format!(" stroke-dasharray=\"{}\"", dash) };
                        s += &format!("<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"{}\"{}/>\n", bx + 4.0, yk, bx + 26.0, yk, col, w, da);
                    }
                    Key::Dot(col) => s += &format!("<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"3.5\" fill=\"{}\"/>\n", bx + 15.0, yk, col),
                }
                s += &format!("<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"{}\" fill=\"{}\">{}</text>\n", bx + 32.0, yk + 4.0, fs - 1.5, INK, markup(label));
            }
        }
        s
    }
}

/// the colour map: white (0) to dark blue (1), one hue, interpolated linearly in RGB between (247, 251, 255) and
/// (8, 48, 107); the value v is clamped to [0, 1]
pub fn blues(v: f64) -> String {
    let t = v.clamp(0.0, 1.0);
    let a = (247.0, 251.0, 255.0);
    let b = (8.0, 48.0, 107.0);
    let c = |p: f64, q: f64| (p + (q - p) * t).round() as u8;
    format!("#{:02x}{:02x}{:02x}", c(a.0, b.0), c(a.1, b.1), c(a.2, b.2))
}

/// a figure: its size, its title and its panels
pub struct Figure {
    pub w: f64,           // width (pixels)
    pub h: f64,           // height (pixels)
    pub title: String,    // the title, centred at the top
    pub axes: Vec<Axes>,  // the panels
    pub extra: String,    // SVG elements outside the panels (colour bars)
}

impl Figure {
    /// an empty figure of the size (w, h) with a title
    pub fn new(w: f64, h: f64, title: &str) -> Figure {
        Figure { w, h, title: title.to_string(), axes: vec![], extra: String::new() }
    }
    /// a colour bar at (x, y) of the size (w, h) for the values 0 .. zmax
    pub fn colorbar(&mut self, x: f64, y: f64, w: f64, h: f64, zmax: f64, label: &str) {
        let n = 60;
        for k in 0..n {
            let v = k as f64 / (n - 1) as f64;
            self.extra += &format!("<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"{}\"/>\n", x, y + h * (1.0 - (k + 1) as f64 / n as f64), w, h / n as f64 + 0.5, blues(v));
        }
        for t in nice_ticks(0.0, zmax, 5) {
            let py = y + h * (1.0 - t / zmax);
            self.extra += &format!("<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"11\" fill=\"{}\">{}</text>\n", x + w + 4.0, py + 4.0, MUTED, markup(&tick_label(t)));
        }
        self.extra += &format!(
            "<text transform=\"translate({:.2},{:.2}) rotate(-90)\" font-size=\"12\" fill=\"{}\" text-anchor=\"middle\">{}</text>\n",
            x + w + 48.0,
            y + 0.5 * h,
            INK,
            markup(label)
        );
    }
    /// the complete SVG document: a white background, the title, the panels (each with its own clipping rectangle) and the
    /// colour bars
    pub fn svg(&self) -> String {
        let mut s = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" font-family=\"DejaVu Sans, Helvetica, Arial, sans-serif\">\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n",
            self.w, self.h, self.w, self.h
        );
        s += &format!("<text x=\"{}\" y=\"26\" font-size=\"16\" fill=\"{}\" text-anchor=\"middle\">{}</text>\n", self.w / 2.0, INK, markup(&self.title));
        for (i, a) in self.axes.iter().enumerate() {
            s += &a.render(i);
        }
        s += &self.extra;
        s += "</svg>\n";
        s
    }
}
