//! The analysis of time series (Sections 15.1 and 15.2 of superfluid.md): the frequency of an oscillation and the rate of an
//! exponential growth.
//!
//! FREQUENCY.  For samples s_j at the times t_j (equally spaced) the windowed Fourier sum
//!     X(omega) = sum_j w_j (s_j - s_mean) e^{-i omega t_j},     w_j = sin^2(pi j/(J-1))  (the Hann window, J samples)
//! has, for a sinusoid of the frequency omega_0, its maximum at omega_0; the window suppresses the leakage of the other
//! frequencies (its side lobes fall as the third power of the distance in frequency).  The maximum of |X| is found by a
//! scan of 2000 frequencies in [lo, hi] and a golden-section refinement.
//!
//! GROWTH RATE.  For a signal that grows as A e^{gamma t}, gamma is the slope of the least-squares straight line through
//! (t_j, ln |s_j|) on a window of times.
pub fn peak_frequency(t: &[f64], s: &[f64], lo: f64, hi: f64) -> f64 {
    let nj = s.len();
    let mean = s.iter().sum::<f64>() / nj as f64;
    let w: Vec<f64> = (0..nj).map(|j| (std::f64::consts::PI * j as f64 / (nj as f64 - 1.0)).sin().powi(2)).collect();
    let amp = |om: f64| -> f64 {
        let (mut re, mut im) = (0.0, 0.0);
        for j in 0..nj {
            let v = w[j] * (s[j] - mean);
            re += v * (om * t[j]).cos();
            im -= v * (om * t[j]).sin();
        }
        re * re + im * im
    };
    let nscan = 2000;
    let mut best = (lo, amp(lo));
    for i in 1..=nscan {
        let om = lo + (hi - lo) * i as f64 / nscan as f64;
        let v = amp(om);
        if v > best.1 {
            best = (om, v);
        }
    }
    let dx = (hi - lo) / nscan as f64;
    let (mut a, mut b) = (best.0 - dx, best.0 + dx);
    let gr = 0.5 * (5f64.sqrt() - 1.0);
    let (mut x1, mut x2) = (b - gr * (b - a), a + gr * (b - a));
    let (mut f1, mut f2) = (amp(x1), amp(x2));
    for _ in 0..100 {
        if f1 > f2 {
            b = x2;
            x2 = x1;
            f2 = f1;
            x1 = b - gr * (b - a);
            f1 = amp(x1);
        } else {
            a = x1;
            x1 = x2;
            f1 = f2;
            x2 = a + gr * (b - a);
            f2 = amp(x2);
        }
    }
    0.5 * (a + b)
}

/// the slope and the intercept of the least-squares line through (t_j, y_j)
pub fn line_fit(t: &[f64], y: &[f64]) -> (f64, f64) {
    let n = t.len() as f64;
    let (st, sy) = (t.iter().sum::<f64>(), y.iter().sum::<f64>());
    let (mt, my) = (st / n, sy / n);
    let sxx: f64 = t.iter().map(|x| (x - mt) * (x - mt)).sum();
    let sxy: f64 = t.iter().zip(y).map(|(x, v)| (x - mt) * (v - my)).sum();
    let slope = sxy / sxx;
    (slope, my - slope * mt)
}
