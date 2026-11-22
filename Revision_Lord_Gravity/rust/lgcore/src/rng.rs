//! SplitMix64: a small deterministic pseudo-random generator (a fixed seed gives the same numbers on
//! every computer, so that every check is reproducible).
//! (Copied unchanged in its code from the library t2core of Revision_SuperLoop/test2/rust; only this note is added.)
//!
//! (Copied unchanged in its code from the library sfcore of the folder Revision_SM_Higgs of PROJECT spacetimes,
//! file rust/sfcore/src/rng.rs; only this note is added.)
pub struct Rng(pub u64);

impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// uniform in [-1, 1)
    pub fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
    }
}
