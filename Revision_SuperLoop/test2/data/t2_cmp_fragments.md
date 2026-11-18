<!-- BEGIN t2_ex8_table -->
| quantities | largest difference | origin |
|------------------------------|----------|------------------------------|
| spectra, densities of eigenvectors and self-consistent pair fields (`kink_spectrum`, `ground_state`, `kink_ground_state`, and the spectrum of the ground state of `quench`) | $10^{-15}$ | rounding in the operator and the eigensolvers |
| time series and fields of `evolve` (norm, energy, probabilities, centre, densities) | $10^{-10}$ | rounding, carried through the adaptive steps of CVODE; the files print 12 significant digits |
| time series and fields of `quench` ($\lvert\vec d_{\rm LL}\rvert$, $N^{\rm F}_{\rm LL}$, $E^{\rm MF}_{\rm LL}$, orthonormality, pair field) | $10^{-9}$ | rounding, carried through the adaptive steps of CVODE; the files print 12 significant digits |
| histories of the Anderson iteration | $2\times10^{-7}$ | rounding, amplified by the least-squares step near convergence (the same numbers of iterations) |
<!-- END t2_ex8_table -->
<!-- BEGIN t2_times_table -->
| example | Python | Rust |
|------|------|------|
| `packet_free` | 0.53 | 0.15 |
| `cyclotron` | 9.27 | 7.26 |
| `andreev` | 1.28 | 0.85 |
| `kink_spectrum` | 1.01 | 3.09 |
| `ground_state` | 1.70 | 6.04 |
| `kink_ground_state` | 8.69 | 28.92 |
| `quench` (the time evolution) | 43.06 | 29.09 |
<!-- END t2_times_table -->
