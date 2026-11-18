#!/usr/bin/env python3
"""Reference values of the random-number generator SplitMix64 for the check T2-RNG of the notebook
3+1super-loop.nb (PROJECT spacetimes).  The algorithm is that of t2core/src/rng.rs of Test 2:
    state <- state + 0x9E3779B97F4A7C15                      (modulo 2^64)
    z <- state
    z <- (z XOR (z >> 30)) * 0xBF58476D1CE4E5B9              (modulo 2^64)
    z <- (z XOR (z >> 27)) * 0x94D049BB133111EB              (modulo 2^64)
    output z XOR (z >> 31)
and a uniform number in [-1, 1) is 2 * (output >> 11) / 2^53 - 1 (exact in double precision).
Python integers are unbounded, so every product is reduced modulo 2^64 explicitly.
Usage:  python3 splitmix64_reference.py  [seed]      (default seed 2026111707, the seed of t2_fields)"""
import sys
MASK = (1 << 64) - 1

class SplitMix64:
    def __init__(self, seed):
        self.state = seed & MASK
    def next_u64(self):
        self.state = (self.state + 0x9E3779B97F4A7C15) & MASK
        z = self.state
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & MASK
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & MASK
        return z ^ (z >> 31)
    def unit(self):
        return 2.0 * (self.next_u64() >> 11) / float(1 << 53) - 1.0

seed = int(sys.argv[1]) if len(sys.argv) > 1 else 2026111707
g = SplitMix64(seed)
print("seed", seed)
print("first three integers:", [g.next_u64() for _ in range(3)])
g = SplitMix64(seed)
print("first four uniform numbers:", [repr(g.unit()) for _ in range(4)])
