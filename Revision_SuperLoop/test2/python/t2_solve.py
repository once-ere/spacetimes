#!/usr/bin/env python3
"""t2_solve.py -- PROJECT spacetimes, Revision_SuperLoop, folder test2, Test 2, item [3].

A solver for the gauge-covariant field equations of the self-interacting spinor field Psi_LL on R^(1,3)_LL in
their mean-field (Dirac-Bogoliubov-de Gennes) form, discretized on a lattice exactly as in item [1]: the lattice
operator H_lat with the link matrices (the covariant central difference) and the Wilson term.  The program

  1. guides the user in defining the initial and the boundary conditions (it prints a guide, lists ready
     examples, and writes any example as a commented input file),
  2. asks for the name of an input file (or takes it from the command line),
  3. reads and checks the initial and boundary conditions and all parameters from that file,
  4. solves the field equations, checks the result (conservation laws, symmetries, known exact results), and
     writes the solution into data files that the plotters of items [4] and [6] read.

Tasks (the key `task` of the input file):
  evolve        the time evolution of one mode in prescribed fields (gauge potentials that may depend on the
                site and on the time, magnetic fields, a pair field): i dw/dx^0 = H_lat w, integrated with
                CVODE (SUNDIALS 7.8.0, the variable-order, variable-step Adams-Moulton integrator);
  spectrum      all eigenvalues of H_lat in prescribed fields, and the densities of the eigenvectors in an
                energy window (bound states);
  ground_state  the self-consistent mean-field ground state: the pair field d^iota(site) that equals
                -(G/2) Pi^iota(site)/dx^D for the positive-energy eigenvectors of H_lat, found by iteration;
  quench        the self-consistent time evolution of all positive-energy modes of a ground state after the
                coupling G is changed at x^0 = 0 (and, optionally, a pulse of a gauge field), integrated with
                CVODE as well (all modes at once, the pair field computed from the modes at every evaluation).

The time integration.  Both tasks that evolve in time call CVODE of SUNDIALS 7.8.0 in pure Rust (vendored in
PROJECT spacetimes) through the shared library libt2cvode of the crate test2/rust/t2cvode_c: the same integrator,
with the same settings and the same error weights of complex components, as the Rust solver t2_solve.  Build the
library once with 'cargo build --release' in the folder test2/rust; this program looks for it in the environment
variable T2CVODE_LIB (the path of the file), then in test2/rust/target/release and in $CARGO_TARGET_DIR/release.

Usage:
  python3 t2_solve.py                         guidance, then the program asks for the input file
  python3 t2_solve.py FILE.t2in               solve the problem of FILE.t2in
  python3 t2_solve.py FILE.t2in --out BASE    the same, with the output files BASE_* (BASE relative to the
                                              current folder) instead of the key `output`
  python3 t2_solve.py --guide                 print the complete guide to the input file
  python3 t2_solve.py --examples              list the examples
  python3 t2_solve.py --example NAME [FILE]   write the example NAME as an input file
  python3 t2_solve.py --selftest [REPORT]     check the lattice operator of this program against an independent
                                              site-by-site implementation and against exact plane-wave energies
                                              (and write the result into the file REPORT)
  python3 t2_solve.py --keys-md FILE          write the table of all keys of the input file (Markdown) into FILE

Requirements: Python 3.8 or later, numpy and scipy, and the shared library libt2cvode (libt2cvode.so on Linux,
libt2cvode.dylib on macOS) built from test2/rust (see above).  The program and its companions (t2_plot.py, and the
Rust programs t2_solve and t2_plot of items [5] and [6]) use the same input format and the same output files.
"""

import ctypes
import math
import os
import re
import sys
import time

import numpy as np
import scipy.sparse as sps

PROGRAM = "t2_solve.py"
FORMAT = "t2in 2"

# ============================================================================ the matrices
# The conventions of t2core/src/model.rs (and of the document of Test 2, Section 2): the bi-quaternion
# matrix M_LL(b) = xi_0 sigma_0 - i (xi_1 sigma_1 + xi_2 sigma_2 + xi_3 sigma_3); the canonical frame
# f_0 = e0, f_j = i e_j; gamma^p = [[0, M(f_p)], [M(conj f_p), 0]] (chiral basis); alpha^j = gamma^0 gamma^j,
# beta = gamma^0; the pair matrices Kch_iota = (C gamma_ch) (x) (eps2 tau_iota) with C = i gamma^2 gamma^0 and
# gamma_ch = i gamma^0 gamma^1 gamma^2 gamma^3.  The 8 components of a mode at a site are ordered
# (Dirac index a = 0..3) x (isospin up, down): component 2a + (0 for up, 1 for down); the 16 components of a
# Nambu mode are the 8 particle components followed by the 8 hole components.

SIG = [np.array([[1, 0], [0, 1]], complex), np.array([[0, 1], [1, 0]], complex),
       np.array([[0, -1j], [1j, 0]], complex), np.array([[1, 0], [0, -1]], complex)]
EPS2 = np.array([[0, 1], [-1, 0]], complex)


def mq(xi):
    """M_LL(b) of the bi-quaternion with the coefficients xi (4 complex numbers)."""
    m = xi[0] * SIG[0]
    for j in range(1, 4):
        m = m - 1j * xi[j] * SIG[j]
    return m


def gamma(p):
    """gamma_LL^p of the canonical frame (4 x 4)."""
    f = np.zeros(4, complex)
    f[p] = 1.0 if p == 0 else 1j
    fc = f.copy()
    fc[1:] = -fc[1:]
    z = np.zeros((2, 2), complex)
    return np.block([[z, mq(f)], [mq(fc), z]])


GAM = [gamma(p) for p in range(4)]
GAMMA_CH = 1j * GAM[0] @ GAM[1] @ GAM[2] @ GAM[3]
CCONJ = 1j * GAM[2] @ GAM[0]
ALPHA = [GAM[0] @ GAM[j] for j in (1, 2, 3)]
BETA = GAM[0].copy()
KPAIR = [np.kron(CCONJ @ GAMMA_CH, EPS2 @ SIG[i]) for i in (1, 2, 3)]
KDAG = [k.conj().T for k in KPAIR]
I2 = np.eye(2, dtype=complex)
I4 = np.eye(4, dtype=complex)

# ============================================================================ the input file
# Each entry: key -> (kind, default, explanation).  The default None means: required (for the tasks that use it).
SPEC = {
    "format": ("str", None, "the format of the file; must be 't2in 2'"),
    "title": ("str", "", "a free title of the problem"),
    "task": ("choice:evolve,spectrum,ground_state,quench", None, "what to solve (see the guide)"),
    "output": ("str", "", "the base name of the output files, relative to the folder of the input file "
                          "(default: the name of the input file without its extension)"),
    "sites": ("ints3", None, "the numbers of lattice sites n_x n_y n_z along x^1, x^2, x^3; 1 means that the "
                             "fields do not depend on that coordinate"),
    "spacing": ("float+", None, "the lattice spacing dx (the same in all directions)"),
    "boundary": ("words3:periodic,wall", "periodic periodic periodic",
                 "the boundary condition of each direction: periodic (a ring) or wall (the field vanishes "
                 "beyond the last site: Dirichlet)"),
    "wilson": ("float0+", "1.0", "the coefficient r_W of the Wilson term (removes the doublers; 1 is standard)"),
    "mass": ("float", "0.4", "the mass m of the spinor charges"),
    "mu": ("float", "0.0", "the chemical potential mu"),
    "g_v": ("float", "0.7", "the isospin coupling g_v"),
    "g_F": ("float", "0.45", "the coupling g_F of the phase group"),
    "potential_0": ("floats4", "0 0 0 0", "the constant potentials A^1_0 A^2_0 A^3_0 A^F_0 (time direction)"),
    "potential_1": ("floats4", "0 0 0 0", "the constant potentials A^1_1 A^2_1 A^3_1 A^F_1 (direction x^1)"),
    "potential_2": ("floats4", "0 0 0 0", "the constant potentials A^1_2 A^2_2 A^3_2 A^F_2 (direction x^2)"),
    "potential_3": ("floats4", "0 0 0 0", "the constant potentials A^1_3 A^2_3 A^3_3 A^F_3 (direction x^3)"),
    "magnetic": ("floats2", "0 0", "constant magnetic fields along x^3: F^F_12 (phase group) and F^3_12 "
                                   "(third isospin direction), in the Landau gauge A_2 = F (x^1 - centre); "
                                   "needs n_x > 1, n_y > 1 and a wall in direction 1"),
    "gauge_file": ("str", "", "a file with potentials that depend on the site (added to the constant ones): one line "
                              "per site, in the order of the sites, with 16 numbers A^1_p A^2_p A^3_p A^F_p for "
                              "p = 0, 1, 2, 3"),
    "pulse_direction": ("int0123", "1", "the index p (0, 1, 2 or 3) of the potential A_p that carries the pulse"),
    "pulse_potential": ("floats4", "0 0 0 0", "the amplitudes A^1 A^2 A^3 A^F of a uniform pulse of the potential A_p "
                                             "(all zero: no pulse)"),
    "pulse_time": ("floats3", "5 1 0", "the centre t_c, the duration t_w and the angular frequency w_p of the pulse: "
                                      "A_p(x^0) = amplitudes exp(-(x^0 - t_c)^2/(2 t_w^2)) cos(w_p (x^0 - t_c))"),
    "pair": ("choice:none,uniform,kink,kink_antikink,step,random,file", "none",
             "the profile of the pair field d^iota: prescribed (evolve, spectrum) or the initial guess "
             "(ground_state, quench)"),
    "pair_direction": ("int123", "3", "the isospin direction iota of the pair field (1, 2 or 3)"),
    "pair_amplitude": ("float", "0.0", "the gap Delta (uniform, step) or Delta_inf (kinks)"),
    "pair_phase": ("float", "0.0", "the phase of the pair field, in radians"),
    "pair_width": ("float+", "1.0", "the width ell_K of a kink or of a step"),
    "pair_position": ("floatlist", "", "the positions along x^1 of the walls (kink: one value, default the "
                                       "centre; kink_antikink: two values, default 1/4 and 3/4 of the "
                                       "length; step: one value, default the centre)"),
    "pair_seed": ("int", "1", "the seed of the random pair field (pair = random)"),
    "pair_file": ("str", "", "a file with the pair field (pair = file): one line per site, in the order of "
                             "the sites, with d1_re d1_im d2_re d2_im d3_re d3_im"),
    "output_step": ("float+", None, "the interval of the time series: the state is written at the times x^0 = 0, "
                                    "output_step, 2 output_step, ..., outputs x output_step (evolve, quench); CVODE "
                                    "chooses its own internal steps between these times"),
    "outputs": ("int+", None, "the number of output intervals: the evolution ends at x^0 = outputs x output_step "
                              "(evolve, quench)"),
    "fields_every": ("int+", "0", "write the fields on the sites every this many outputs (0: about 50 times); the time "
                                  "series is written at every output"),
    "rtol": ("float+", "1e-12", "the relative tolerance of CVODE: the error weight of a complex component w_cmp of the "
                                "state is the inverse of (rtol times the modulus of w_cmp, plus atol) (evolve, quench)"),
    "atol": ("float+", "1e-14", "the absolute tolerance of CVODE: components smaller than this count as zero in the "
                                "error test (evolve, quench)"),
    "initial": ("choice:packet,eigenmode,file", "packet", "the initial mode of the task evolve"),
    "packet_center": ("floats3", "", "the centre of the Gaussian packet (x^1 x^2 x^3; default the centre of "
                                     "the lattice)"),
    "packet_width": ("float+", "2.0", "the width of the packet (standard deviation of the density)"),
    "packet_momentum": ("floats3", "0 0 0", "the momentum q of the packet"),
    "packet_branch": ("choice:positive,negative", "positive", "the sign of the energy +-eps_lat(q) of the free "
                                                              "lattice operator (without mu) of the packet"),
    "packet_spin": ("choice:up,down", "up", "the spin along x^3 of the reference spinor"),
    "packet_isospin": ("choice:up,down", "up", "the isospin component"),
    "packet_part": ("choice:particle,hole", "particle", "particle part or hole part of the mode"),
    "eigenmode_index": ("int", "0", "the number (ascending, from 0) of the eigenvector used as initial mode "
                                    "(initial = eigenmode)"),
    "initial_file": ("str", "", "a file with the initial mode (initial = file): 16 x (number of sites) lines "
                                "'re im' in the order of the components"),
    "spectrum_window": ("floats2", "-1 1", "the energy window of the eigenvectors whose densities are written "
                                          "(spectrum)"),
    "spectrum_max_states": ("int+", "32", "at most this many eigenvectors of the window are written (spectrum), summed "
                                         "over each group of degenerate eigenvalues"),
    "coupling": ("float+", None, "the coupling G of the self-consistency condition (ground_state, quench)"),
    "pair_constraint": ("choice:none,line", "none", "line: the self-consistent iteration keeps the pair field a real "
                                                    "multiple of exp(i pair_phase) along pair_direction (for walls of "
                                                    "the pair field, which are unstable without it)"),
    "mixing": ("float01", "0.5", "the damping factor beta of the Anderson iteration (ground_state, quench)"),
    "tolerance": ("float+", "1e-10", "the iteration stops when the largest change of d^iota is below this"),
    "max_iterations": ("int+", "400", "the largest number of iterations"),
    "quench_coupling": ("float+", None, "the coupling after x^0 = 0 (quench)"),
}

TASK_KEYS = {
    "evolve": {"outputs", "output_step"},
    "spectrum": set(),
    "ground_state": {"coupling"},
    "quench": {"coupling", "quench_coupling", "outputs", "output_step"},
}

GUIDE = r"""
THE TEST 2 SOLVER -- GUIDE TO THE INPUT FILE (format t2in 2)

What is solved.  The mean-field field equations of the self-interacting spinor field on the lattice:
    i dw/dx^0 = H_lat w,     H_lat = [[h_lat - V0 - mu, P], [P^dag, -conj(h_lat - V0 - mu)]],
for Nambu modes w (8 particle and 8 hole components per site).  h_lat is the covariant lattice Dirac
operator: the links U_j(s) = exp(-i dx (g_v A^iota_j(s) tau_iota/2 + g_F A^F_j(s))) on the bonds from the
site s to its neighbour s + j (the gauge-covariant central difference), the Wilson term with the coefficient
r_W, and the mass term m beta; V0 = g_v A^iota_0 tau_iota/2 + g_F A^F_0 is the potential of the time
direction, and P = sum_iota d^iota Kch_iota^dag the pair field.  With self-consistency (tasks ground_state
and quench) the pair field is
    d^iota(site) = -(G/2) Pi^iota(site)/dx^D,   Pi^iota = sum over the modes of w_p^T Kch_iota conj(w_h),
the modes being unit vectors and D the number of directions with more than one site.

The file.  One entry per line: 'key = value'.  '#' starts a comment.  Unknown keys are errors (so that a
misspelt key is not silently ignored).  Keys may appear in any order.

Boundary conditions.  'sites = n_x n_y n_z' fixes the lattice (sites at x^j = s^j dx, s^j = 0 .. n_j - 1);
a direction with one site is not used (the fields do not depend on that coordinate).  'boundary' gives,
for each direction, 'periodic' (the last site is the neighbour of the first: a ring or a torus; a constant
potential then acts through its holonomy) or 'wall' (the field vanishes beyond the last site: a hard
wall, the Dirichlet condition).  A magnetic field needs a wall in direction 1 (the Landau gauge).

Initial conditions.  For 'task = evolve' the initial mode is
    initial = packet     a Gaussian wave packet: centre, width (standard deviation of the density),
                         momentum q, energy branch (the sign of +-eps_lat(q)), spin and isospin along x^3, and
                         whether it is a particle or a hole; its spinor is the eigenvector of the free
                         lattice operator at q, so that the packet moves with the group velocity;
    initial = eigenmode  an eigenvector of H_lat (by its number in the ascending list; small lattices);
    initial = file       16 x (number of sites) lines 're im'.
For 'ground_state' and 'quench' the initial condition is the guess of the pair field ('pair = ...'), from
which the self-consistent iteration starts; the quench starts from the converged ground state.

Gauge fields.  Constant potentials ('potential_p = A^1_p A^2_p A^3_p A^F_p', p = 0 .. 3), potentials that
depend on the site ('gauge_file'), constant magnetic fields along x^3 ('magnetic = F^F_12 F^3_12', in the
Landau gauge), and a uniform pulse that depends on the time ('pulse_direction', 'pulse_potential',
'pulse_time'): A_p(x^0) = amplitudes exp(-(x^0 - t_c)^2/(2 t_w^2)) cos(w_p (x^0 - t_c)) is added to the
potential A_p (a pulse of A_1, A_2 or A_3 is a pulse of an electric field).  All contributions are added; the
links of a site use the potentials at that site.  The tasks spectrum and ground_state ignore the pulse.

The pair field.  'pair = none | uniform | kink | kink_antikink | step | random | file', with
'pair_direction', 'pair_amplitude', 'pair_phase', 'pair_width', 'pair_position'.  A kink is
Delta_inf tanh((x^1 - x_w)/ell_K); kink_antikink is Delta_inf tanh((x^1 - x_1)/ell_K) tanh((x_2 - x^1)/ell_K);
step is Delta (1 + tanh((x^1 - x_s)/ell_K))/2 (a superfluid on the right, a normal region on the left).

Time integration.  Both tasks that evolve in time (evolve, quench) integrate the equations with CVODE, the
variable-order, variable-step integrator of SUNDIALS 7.8.0 (in pure Rust, vendored in PROJECT spacetimes; this
program calls it through the shared library libt2cvode of the crate test2/rust/t2cvode_c): Adams-Moulton
formulas of the orders 1 to 12, the corrector solved by fixed-point iteration, and a test of the estimated local
error of every step,
    sqrt( (1/(2 n_complex)) sum over w_cmp of |error of w_cmp|^2 / (rtol |w_cmp| + atol)^2 ) <= 1
(the sum over the n_complex complex components w_cmp of the state, the real and imaginary parts of every
component weighted alike).  CVODE chooses its
step sizes and orders itself; 'output_step' and 'outputs' only give the output times x^0 = 0, output_step,
2 output_step, ..., at which the state is written (by the interpolating polynomial of CVODE), and 'rtol'
(default 1e-12) and 'atol' (default 1e-14) the accuracy.  The quench integrates all modes at once, with the pair field computed from the modes at
every evaluation of the right side.  The exact equations conserve the norm of a mode (evolve), the fermion
number and the orthonormality of the modes (quench); CVODE keeps them up to the accumulated local error that its
error test permits, (number of steps) sqrt(2) (rtol ||w_lat|| + atol sqrt(n_complex)) for a state w_lat of the
norm ||w_lat||, and the program checks the change of the norm against 10 times this amount (the change of the
fermion number against 2 ||W|| times that, of the orthonormality against 2 times that; the true local error may
exceed the estimate of CVODE by a modest factor).  Hbound = (number of active directions)(1 + 2 r_W)/dx + |m| +
|mu| + max|V0| + max sum|d^iota|, an upper bound of the eigenvalue magnitudes of H_lat, is reported.
The format t2in 1 (with the keys time_step, steps and output_every of a fixed-step method) is not accepted.

Output.  'output = NAME' (default: the name of the input file), or '--out NAME' on the command line.  The
program writes NAME_summary.txt (the problem, the checks and the results), NAME_series.csv (quantities
against time or iteration), NAME_fields.csv (fields on the sites), and for some tasks NAME_spectrum.csv and
NAME_iterations.csv.  Plot them with 'python3 t2_plot.py NAME' (item [4]) or the Rust 't2_plot NAME' (item [6]).

The keys:
"""

EXAMPLES = {
    "packet_free": ("a free Gaussian packet of positive energy on a ring (group velocity)", """\
format = t2in 2
title = a free Gaussian wave packet on a ring
task = evolve
sites = 128 1 1
spacing = 0.5
boundary = periodic periodic periodic
mass = 0.4
mu = 0.0
initial = packet
packet_center = 16 0 0
packet_width = 3.0
packet_momentum = 0.8 0 0
packet_branch = positive
packet_spin = up
packet_isospin = up
packet_part = particle
output_step = 0.05
outputs = 600
fields_every = 20
"""),
    "cyclotron": ("a packet in a magnetic field on a 40 x 40 lattice with walls (cyclotron orbit)", """\
format = t2in 2
title = a wave packet in a constant magnetic field (cyclotron motion)
task = evolve
sites = 40 40 1
spacing = 0.5
boundary = wall wall periodic
mass = 0.4
mu = 0.0
magnetic = 0.4 0.0
initial = packet
packet_center = 9.75 13.3 0
packet_width = 1.5
packet_momentum = 0.6 0 0
packet_branch = positive
packet_spin = up
packet_isospin = up
output_step = 0.05
outputs = 720
fields_every = 80
"""),
    "andreev": ("a particle hitting a superfluid region: Andreev reflection into a hole", """\
format = t2in 2
title = Andreev reflection of a particle at a normal-superfluid interface
task = evolve
sites = 240 1 1
spacing = 0.5
boundary = wall periodic periodic
mass = 0.4
mu = 1.0
pair = step
pair_direction = 3
pair_amplitude = 0.6
pair_width = 1.0
pair_position = 60
initial = packet
packet_center = 30 0 0
packet_width = 4.0
packet_momentum = 0.917 0 0
packet_branch = positive
packet_spin = up
packet_isospin = up
packet_part = particle
output_step = 0.05
outputs = 1200
fields_every = 40
"""),
    "kink_spectrum": ("the spectrum of a ring with a kink and an antikink of the pair field (bound states)", """\
format = t2in 2
title = bound states of a kink and an antikink of the pair field
task = spectrum
sites = 60 1 1
spacing = 0.4
boundary = periodic periodic periodic
mass = 0.4
mu = 0.0
pair = kink_antikink
pair_direction = 3
pair_amplitude = 1.0
pair_width = 1.0
pair_position = 6 18
spectrum_window = -0.8 0.8
spectrum_max_states = 16
"""),
    "ground_state": ("the self-consistent uniform superfluid of a chain (compare with the lattice gap equation)", """\
format = t2in 2
title = the self-consistent uniform polar superfluid of a chain
task = ground_state
sites = 32 1 1
spacing = 0.5
boundary = periodic periodic periodic
mass = 0.4
mu = 0.6
coupling = 0.46
pair = uniform
pair_direction = 3
pair_amplitude = 0.2
mixing = 0.6
tolerance = 1e-11
max_iterations = 400
"""),
    "kink_ground_state": ("a self-consistent kink of the pair field between two walls (massless charges)", """\
format = t2in 2
title = a self-consistent kink of the pair field
task = ground_state
sites = 40 1 1
spacing = 0.5
boundary = wall periodic periodic
mass = 0.0
mu = 0.0
coupling = 0.48
pair = kink
pair_direction = 3
pair_amplitude = 0.6
pair_width = 1.7
pair_constraint = line
mixing = 0.5
tolerance = 1e-10
max_iterations = 400
"""),
    "quench": ("a quench of the coupling: the amplitude (Higgs) oscillation of the pair field", """\
format = t2in 2
title = a quench of the coupling of the uniform superfluid
task = quench
sites = 24 1 1
spacing = 0.5
boundary = periodic periodic periodic
mass = 0.0
mu = 0.0
coupling = 0.45
quench_coupling = 0.455
pair = uniform
pair_direction = 3
pair_amplitude = 0.5
mixing = 0.6
tolerance = 1e-12
max_iterations = 400
output_step = 0.04
outputs = 600
fields_every = 25
"""),
}


class InputError(Exception):
    pass


def guide_text():
    lines = [GUIDE]
    for k, (kind, default, expl) in SPEC.items():
        d = "required" if default is None else ("default: " + (default if default != "" else "(see text)"))
        lines.append("  %-20s %s  [%s; %s]" % (k, expl, kind, d))
    lines.append("")
    lines.append("Tasks and the keys they need: " + "; ".join(
        "%s: %s" % (t, ", ".join(sorted(v)) if v else "-") for t, v in TASK_KEYS.items()))
    lines.append("Examples: python3 t2_solve.py --examples")
    return "\n".join(lines)


def parse_value(key, kind, text):
    """Convert the text of a value; raise InputError with a helpful message."""
    t = text.strip()
    try:
        if kind == "str":
            return t
        if kind.startswith("choice:"):
            opts = kind.split(":", 1)[1].split(",")
            if t not in opts:
                raise InputError("'%s' must be one of %s, not '%s'" % (key, " | ".join(opts), t))
            return t
        if kind.startswith("words3:"):
            opts = kind.split(":", 1)[1].split(",")
            w = t.split()
            if len(w) != 3 or any(x not in opts for x in w):
                raise InputError("'%s' needs three words, each one of %s" % (key, " | ".join(opts)))
            return w
        if kind == "float":
            return float(t)
        if kind == "float+":
            v = float(t)
            if not v > 0:
                raise InputError("'%s' must be positive" % key)
            return v
        if kind == "float0+":
            v = float(t)
            if v < 0:
                raise InputError("'%s' must not be negative" % key)
            return v
        if kind == "float01":
            v = float(t)
            if not (0 < v <= 1):
                raise InputError("'%s' must lie in (0, 1]" % key)
            return v
        if kind == "int":
            return int(t)
        if kind == "int+":
            v = int(t)
            if v < 0 or (v == 0 and key != "fields_every"):
                raise InputError("'%s' must be a positive integer" % key)
            return v
        if kind == "int0123":
            v = int(t)
            if v not in (0, 1, 2, 3):
                raise InputError("'%s' must be 0, 1, 2 or 3" % key)
            return v
        if kind == "int123":
            v = int(t)
            if v not in (1, 2, 3):
                raise InputError("'%s' must be 1, 2 or 3" % key)
            return v
        if kind == "ints3":
            v = [int(x) for x in t.split()]
            if len(v) != 3 or min(v) < 1:
                raise InputError("'%s' needs three integers >= 1" % key)
            return v
        if kind in ("floats2", "floats3", "floats4"):
            n = int(kind[-1])
            if t == "":
                return None
            v = [float(x) for x in t.split()]
            if len(v) != n:
                raise InputError("'%s' needs %d numbers" % (key, n))
            return v
        if kind == "floatlist":
            return [float(x) for x in t.split()] if t else []
    except ValueError:
        raise InputError("the value of '%s' ('%s') is not of the kind %s" % (key, t, kind))
    raise InputError("internal: unknown kind %s" % kind)


def defaults():
    """The dictionary of all keys with their default values (None where the key is required)."""
    return {k: (parse_value(k, kind, default) if default is not None else None) for k, (kind, default, _) in SPEC.items()}


def read_input(path):
    """Read an input file into a dictionary of checked values (with the defaults filled in)."""
    if not os.path.isfile(path):
        raise InputError("there is no file '%s'" % path)
    with open(path, encoding="utf-8") as f:
        text = f.read()
    for line in text.splitlines():
        s = line.split("#", 1)[0]
        if "=" in s and s.split("=", 1)[0].strip() == "format" and s.split("=", 1)[1].strip() == "t2in 1":
            raise InputError("the format 't2in 1' (with the keys time_step, steps and output_every of a fixed-step method) "
                             "is no longer accepted: the time integration is done by CVODE; write 'format = t2in 2' and "
                             "replace time_step by output_step, steps by outputs and output_every by fields_every "
                             "(optional: rtol, atol)")
    raw = {}
    with open(path, encoding="utf-8") as f:
        for no, line in enumerate(f, 1):
            s = line.split("#", 1)[0].strip()
            if not s:
                continue
            if "=" not in s:
                raise InputError("line %d: expected 'key = value', found '%s'" % (no, s))
            k, v = s.split("=", 1)
            k = k.strip()
            if k not in SPEC:
                close = [x for x in SPEC if x.startswith(k[:4])]
                raise InputError("line %d: unknown key '%s'%s" % (no, k, (" (did you mean %s?)" % ", ".join(close))
                                                                   if close else ""))
            if k in raw:
                raise InputError("line %d: the key '%s' appears twice" % (no, k))
            raw[k] = v
    cfg = defaults()
    for k, v in raw.items():
        cfg[k] = parse_value(k, SPEC[k][0], v)
    if cfg["format"] != FORMAT:
        raise InputError("the first entry must be 'format = %s'" % FORMAT)
    for k in ("task", "sites", "spacing"):
        if cfg[k] is None:
            raise InputError("the key '%s' is required" % k)
    for k in TASK_KEYS[cfg["task"]]:
        if cfg[k] is None:
            raise InputError("the task '%s' needs the key '%s'" % (cfg["task"], k))
    if cfg["output"] == "":
        cfg["output"] = os.path.splitext(os.path.basename(path))[0]
    cfg["_dir"] = os.path.dirname(os.path.abspath(path))
    cfg["_path"] = path
    return cfg


# ============================================================================ the lattice and the fields
class Lattice:
    def __init__(self, cfg):
        self.n = list(cfg["sites"])
        self.ell = cfg["spacing"]
        self.periodic = [b == "periodic" for b in cfg["boundary"]]
        self.wilson = cfg["wilson"]
        self.mass = cfg["mass"]
        self.mu = cfg["mu"]
        self.ns = self.n[0] * self.n[1] * self.n[2]
        self.active = [n > 1 for n in self.n]
        self.D = sum(self.active)
        s = np.arange(self.ns)
        self.c = np.stack([s % self.n[0], (s // self.n[0]) % self.n[1], s // (self.n[0] * self.n[1])], axis=1)
        self.x = self.c * self.ell

    def neighbours(self, j, fwd):
        """For each site the neighbour in direction j (forward or backward), -1 where there is none."""
        n = self.n[j]
        c = self.c.copy()
        if n == 1:
            return -np.ones(self.ns, int)
        cj = c[:, j] + (1 if fwd else -1)
        ok = (cj >= 0) & (cj < n)
        if self.periodic[j]:
            cj = cj % n
            ok = np.ones(self.ns, bool)
        c[:, j] = np.where(ok, cj, 0)
        t = c[:, 0] + self.n[0] * (c[:, 1] + self.n[1] * c[:, 2])
        return np.where(ok, t, -1)


def link(gv, gf, a, b, ell):
    """exp(-i ell (gv a.tau/2 + gf b)) in closed form (as t2core::lattice::link): one 2 x 2 matrix."""
    return link_vec(gv, gf, np.array([a], float), np.array([b], float), ell)[0]


def link_vec(gv, gf, a, b, ell):
    """exp(-i ell (gv a.tau/2 + gf b)) for the rows of a (n x 3) and b (n): an array (n, 2, 2).  Closed form:
    exp(-i ell gf b) (cos x - i sin x (n.tau)), x = ell gv |a|/2, n = a/|a|."""
    a = np.asarray(a, float)
    b = np.asarray(b, float)
    na = np.sqrt(np.sum(a ** 2, axis=1))
    x = 0.5 * ell * gv * na
    ph = np.exp(-1j * ell * gf * b)
    cx, sx = np.cos(x), np.sin(x)
    safe = np.where(na > 0, na, 1.0)
    nv = np.where(na[:, None] > 0, a / safe[:, None], 0.0)
    m = np.empty((len(b), 2, 2), complex)
    m[:, 0, 0] = cx - 1j * sx * nv[:, 2]
    m[:, 0, 1] = -sx * nv[:, 1] - 1j * sx * nv[:, 0]
    m[:, 1, 0] = sx * nv[:, 1] - 1j * sx * nv[:, 0]
    m[:, 1, 1] = cx + 1j * sx * nv[:, 2]
    return ph[:, None, None] * m


def potential(gv, gf, a0, a0f):
    """The 2 x 2 potential matrix gv A0.tau/2 + gf A0F (as t2core::lattice::potential)."""
    return potential_vec(gv, gf, np.array([a0], float), np.array([a0f], float))[0]


def potential_vec(gv, gf, a0, a0f):
    """gv A0.tau/2 + gf A0F for the rows of a0 (n x 3) and a0f (n): an array (n, 2, 2)."""
    h = 0.5 * gv
    v = np.empty((len(a0f), 2, 2), complex)
    v[:, 0, 0] = gf * a0f + h * a0[:, 2]
    v[:, 0, 1] = h * a0[:, 0] - 1j * h * a0[:, 1]
    v[:, 1, 0] = h * a0[:, 0] + 1j * h * a0[:, 1]
    v[:, 1, 1] = gf * a0f - h * a0[:, 2]
    return v


class Gauge:
    """The gauge potentials A^c_p(site, x^0) (c = 1, 2, 3, F) and from them the links U[site, j] and V0[site]."""

    def __init__(self, cfg, lat):
        self.gv, self.gf, self.ell = cfg["g_v"], cfg["g_F"], lat.ell
        ns = lat.ns
        A = np.zeros((ns, 4, 4))
        for p in range(4):
            A[:, p, :] = np.array(cfg["potential_%d" % p])[None, :]
        if cfg["gauge_file"]:
            path = cfg["gauge_file"]
            if not os.path.isabs(path):
                path = os.path.join(cfg["_dir"], path)
            arr = np.loadtxt(path, ndmin=2)
            if arr.shape != (ns, 16):
                raise InputError("the gauge file must have %d lines of 16 numbers" % ns)
            A += arr.reshape(ns, 4, 4)
        fF, f3 = cfg["magnetic"]
        if fF != 0 or f3 != 0:
            if lat.n[0] < 2 or lat.n[1] < 2:
                raise InputError("a magnetic field needs n_x > 1 and n_y > 1")
            if lat.periodic[0]:
                raise InputError("a magnetic field needs 'wall' in direction 1 (the Landau gauge A_2 = F x^1 is not "
                                 "periodic in x^1)")
            xc = 0.5 * (lat.n[0] - 1) * lat.ell
            A[:, 2, 3] += fF * (lat.x[:, 0] - xc)
            A[:, 2, 2] += f3 * (lat.x[:, 0] - xc)
        self.A = A
        self.pdir = cfg["pulse_direction"]
        self.pamp = np.array(cfg["pulse_potential"], float)
        self.ptc, self.ptw, self.pw = cfg["pulse_time"]
        self.static = not np.any(self.pamp != 0)
        self.lat = lat
        self.desc = "potentials %s, magnetic %s, gauge file %s, pulse %s" % (
            [cfg["potential_%d" % p] for p in range(4)], cfg["magnetic"], cfg["gauge_file"] or "none",
            "none" if self.static else "A_%d amplitudes %s, t_c %g, t_w %g, w_p %g" % (
                self.pdir, list(self.pamp), self.ptc, self.ptw, self.pw))

    def envelope(self, t):
        return math.exp(-(t - self.ptc) ** 2 / (2 * self.ptw ** 2)) * math.cos(self.pw * (t - self.ptc))

    def potentials(self, t=None):
        """A[site, p, c] at the time t (t = None: without the pulse)."""
        if t is None or self.static:
            return self.A
        A = self.A.copy()
        A[:, self.pdir, :] += self.pamp[None, :] * self.envelope(t)
        return A

    def at(self, t=None):
        """The links U[site, j] (2 x 2) and the potential matrices V0[site] (2 x 2) at the time t."""
        A = self.potentials(t)
        ns = self.lat.ns
        U = np.zeros((ns, 3, 2, 2), complex)
        for j in range(3):
            U[:, j] = link_vec(self.gv, self.gf, A[:, j + 1, :3], A[:, j + 1, 3], self.ell)
        V0 = potential_vec(self.gv, self.gf, A[:, 0, :3], A[:, 0, 3])
        return U, V0

    def vmax(self):
        """max over the sites and the times of the Frobenius norm of V0 (triangle inequality for the pulse)."""
        V = potential_vec(self.gv, self.gf, self.A[:, 0, :3], self.A[:, 0, 3])
        v = float(np.max(np.sqrt(np.sum(np.abs(V) ** 2, axis=(1, 2)))))
        if not self.static and self.pdir == 0:
            Vp = potential(self.gv, self.gf, self.pamp[:3], self.pamp[3])
            v += float(np.sqrt(np.sum(np.abs(Vp) ** 2)))
        return v

    def is_free(self):
        return self.static and not np.any(self.A != 0)


class SplitMix64:
    """The pseudo-random generator SplitMix64 of t2core/src/rng.rs (64-bit arithmetic modulo 2^64)."""
    MASK = (1 << 64) - 1

    def __init__(self, seed):
        self.state = int(seed) & self.MASK

    def next_u64(self):
        self.state = (self.state + 0x9E3779B97F4A7C15) & self.MASK
        z = self.state
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & self.MASK
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & self.MASK
        return z ^ (z >> 31)

    def unit(self):
        """uniform in [-1, 1)"""
        return (self.next_u64() >> 11) / float(1 << 53) * 2.0 - 1.0


def pair_profile(cfg, lat):
    """The pair field d[site, iota] (complex) of the key 'pair'."""
    ns = lat.ns
    d = np.zeros((ns, 3), complex)
    kind = cfg["pair"]
    if kind == "none":
        return d
    if kind == "file":
        p = cfg["pair_file"]
        if not os.path.isabs(p):
            p = os.path.join(cfg["_dir"], p)
        arr = np.loadtxt(p, ndmin=2)
        if arr.shape != (ns, 6):
            raise InputError("the pair file must have %d lines of 6 numbers" % ns)
        return arr[:, 0::2] + 1j * arr[:, 1::2]
    if kind == "random":
        # SplitMix64 (as t2core/src/rng.rs), so that the Rust solver makes the same field: for each site and each
        # isospin direction first the real, then the imaginary part, each uniform in [-1, 1), times the amplitude
        rng = SplitMix64(cfg["pair_seed"])
        amp = cfg["pair_amplitude"]
        for s in range(ns):
            for i in range(3):
                re = rng.unit()
                d[s, i] = amp * complex(re, rng.unit())
        return d
    i = cfg["pair_direction"] - 1
    amp = cfg["pair_amplitude"] * complex(math.cos(cfg["pair_phase"]), math.sin(cfg["pair_phase"]))
    x1 = lat.x[:, 0]
    L1 = lat.n[0] * lat.ell
    w = cfg["pair_width"]
    pos = cfg["pair_position"]
    if kind == "uniform":
        prof = np.ones(ns)
    elif kind == "kink":
        xw = pos[0] if pos else 0.5 * (lat.n[0] - 1) * lat.ell
        prof = np.tanh((x1 - xw) / w)
    elif kind == "kink_antikink":
        if len(pos) == 2:
            xa, xb = pos
        else:
            xa, xb = 0.25 * L1, 0.75 * L1
        prof = np.tanh((x1 - xa) / w) * np.tanh((xb - x1) / w)
    else:  # step
        xs = pos[0] if pos else 0.5 * (lat.n[0] - 1) * lat.ell
        prof = 0.5 * (1.0 + np.tanh((x1 - xs) / w))
    d[:, i] = amp * prof
    return d


# ============================================================================ the lattice operator
def onsite_and_hops(lat, U, V0):
    """The particle block A = h_lat - V0 - mu as lists of 8 x 8 blocks: (row sites, column sites, blocks)."""
    ns, ell, r = lat.ns, lat.ell, lat.wilson
    blocks_r, blocks_c, blocks_v = [], [], []
    # on the site: (m + r_W D/dx) beta (x) 1_2 - 1_4 (x) (V0 + mu)
    diag = np.kron(BETA * (lat.mass + r * lat.D / ell), I2)
    ons = np.broadcast_to(diag, (ns, 8, 8)) - np.einsum("ab,scd->sacbd", I4, V0 + lat.mu * I2).reshape(ns, 8, 8)
    blocks_r.append(np.arange(ns))
    blocks_c.append(np.arange(ns))
    blocks_v.append(ons)
    for j in range(3):
        if not lat.active[j]:
            continue
        df = (-1j * ALPHA[j] - r * BETA) / (2 * ell)  # multiplies U_j(s) psi(s + j)
        db = (1j * ALPHA[j] - r * BETA) / (2 * ell)   # multiplies U_j(s - j)^dag psi(s - j)
        fw = lat.neighbours(j, True)
        ok = fw >= 0
        s_ok = np.nonzero(ok)[0]
        blk = np.einsum("ab,scd->sacbd", df, U[s_ok, j]).reshape(len(s_ok), 8, 8)
        blocks_r.append(s_ok)
        blocks_c.append(fw[ok])
        blocks_v.append(blk)
        bw = lat.neighbours(j, False)
        okb = bw >= 0
        s_okb = np.nonzero(okb)[0]
        t = bw[okb]
        udag = np.conj(np.transpose(U[t, j], (0, 2, 1)))
        blk = np.einsum("ab,scd->sacbd", db, udag).reshape(len(s_okb), 8, 8)
        blocks_r.append(s_okb)
        blocks_c.append(t)
        blocks_v.append(blk)
    return blocks_r, blocks_c, blocks_v


def build_h0(lat, U, V0):
    """The sparse matrix (16 rows per site) of H_lat without the pair field: A on the particle components and
    -conj(A) on the hole components."""
    br, bc, bv = onsite_and_hops(lat, U, V0)
    rows, cols, vals = [], [], []
    a8 = np.arange(8)
    for r_, c_, v_ in zip(br, bc, bv):
        R = (16 * r_[:, None, None] + a8[None, :, None]) * np.ones((1, 1, 8), int)
        C = (16 * c_[:, None, None] + a8[None, None, :]) * np.ones((1, 8, 1), int)
        nz = v_ != 0
        rows += [R[nz], R[nz] + 8]
        cols += [C[nz], C[nz] + 8]
        vals += [v_[nz], -np.conj(v_[nz])]
    n = 16 * lat.ns
    return sps.coo_matrix((np.concatenate(vals), (np.concatenate(rows), np.concatenate(cols))), shape=(n, n)).tocsr()


def pair_blocks(d):
    """P[site] = sum_iota d^iota Kch_iota^dag (8 x 8 per site)."""
    return np.einsum("si,iab->sab", d, np.array(KDAG))


def build_hp(lat, d):
    """The sparse matrix of the pair terms: P on (particle row, hole column), P^dag on (hole row, particle column)."""
    P = pair_blocks(d)
    ns = lat.ns
    s = np.arange(ns)
    a8 = np.arange(8)
    R = (16 * s[:, None, None] + a8[None, :, None]) * np.ones((1, 1, 8), int)
    C = (16 * s[:, None, None] + a8[None, None, :]) * np.ones((1, 8, 1), int)
    Pd = np.conj(np.transpose(P, (0, 2, 1)))
    nz = P != 0
    nzd = Pd != 0
    rows = np.concatenate([R[nz], R[nzd] + 8])
    cols = np.concatenate([C[nz] + 8, C[nzd]])
    vals = np.concatenate([P[nz], Pd[nzd]])
    n = 16 * ns
    return sps.coo_matrix((vals, (rows, cols)), shape=(n, n)).tocsr()


def apply_pair(P, W, out):
    """out += the pair terms applied to the modes W (16 ns x M), with P[site] (8 x 8)."""
    ns = P.shape[0]
    M = W.shape[1]
    W3 = W.reshape(ns, 16, M)
    O3 = out.reshape(ns, 16, M)
    O3[:, :8, :] += np.einsum("sab,sbm->sam", P, W3[:, 8:, :])
    O3[:, 8:, :] += np.einsum("sba,sbm->sam", np.conj(P), W3[:, :8, :])


def norm_bound(lat, vmax, d):
    """Hbound: an upper bound of the eigenvalue magnitudes of H_lat (as t2core::Lattice::norm_bound)."""
    dmax = float(np.max(np.sum(np.abs(d), axis=1))) if lat.ns else 0.0
    return lat.D * (1 + 2 * lat.wilson) / lat.ell + abs(lat.mass) + abs(lat.mu) + vmax + dmax


def anomalous(W, ns):
    """Pi^iota[site] = sum over the modes (columns of W) of w_p^T Kch_iota conj(w_h)."""
    M = W.shape[1]
    W3 = W.reshape(ns, 16, M)
    u, v = W3[:, :8, :], W3[:, 8:, :]
    return np.stack([np.einsum("sam,ab,sbm->s", u, KPAIR[i], np.conj(v)) for i in range(3)], axis=1)


def h_plane(q, lat):
    """The 4 x 4 matrix of the free lattice operator on exp(i q.x) (as t2core::lattice::h_lat_plane)."""
    h = np.zeros((4, 4), complex)
    w = 0.0
    for j in range(3):
        if lat.active[j]:
            h = h + ALPHA[j] * math.sin(q[j] * lat.ell) / lat.ell
            w += 1.0 - math.cos(q[j] * lat.ell)
    return h + BETA * (lat.mass + lat.wilson / lat.ell * w)


def eps_lat(q, lat):
    """eps_lat(q) of (2.37)."""
    s2, w = 0.0, 0.0
    for j in range(3):
        if lat.active[j]:
            s2 += (math.sin(q[j] * lat.ell) / lat.ell) ** 2
            w += 1.0 - math.cos(q[j] * lat.ell)
    return math.sqrt(s2 + (lat.mass + lat.wilson / lat.ell * w) ** 2)


def grad_eps_lat(q, lat):
    """The group velocity d eps_lat/d q^j (analytic derivative of (2.37))."""
    e = eps_lat(q, lat)
    meff = lat.mass + lat.wilson / lat.ell * sum(1 - math.cos(q[j] * lat.ell) for j in range(3) if lat.active[j])
    g = []
    for j in range(3):
        if not lat.active[j]:
            g.append(0.0)
            continue
        s, c = math.sin(q[j] * lat.ell), math.cos(q[j] * lat.ell)
        g.append((s * c / lat.ell + meff * lat.wilson * s) / e)
    return g


# ============================================================================ the initial mode
def kgrid(lat):
    """The allowed momenta of a lattice whose active directions are periodic, in the order of numpy's FFT:
    an array k[iz, iy, ix, j] (zero along inactive directions)."""
    ks = []
    for j in range(3):
        n = lat.n[j]
        idx = np.arange(n)
        ks.append(2 * math.pi * np.where(idx <= n // 2, idx, idx - n) / (n * lat.ell) if lat.active[j] else np.zeros(n))
    K = np.zeros((lat.n[2], lat.n[1], lat.n[0], 3))
    K[..., 0] = ks[0][None, None, :]
    K[..., 1] = ks[1][None, :, None]
    K[..., 2] = ks[2][:, None, None]
    return K


def branch_projectors(lat, part, sgn):
    """For every allowed momentum k the projector (1 + sgn M(k)/eps_lat(k))/2 on the branch sgn eps_lat(k) of the free
    lattice operator: M(k) = h_lat(k) for the particle part, M(k) = -conj(h_lat(-k)) for the hole part."""
    K = kgrid(lat)
    Pr = np.zeros(K.shape[:3] + (4, 4), complex)
    for iz in range(lat.n[2]):
        for iy in range(lat.n[1]):
            for ix in range(lat.n[0]):
                k = K[iz, iy, ix]
                M = h_plane(k, lat) if part == "particle" else -np.conj(h_plane(-k, lat))
                Pr[iz, iy, ix] = 0.5 * (I4 + sgn * M / eps_lat(k, lat))
    return Pr


def packet_mode(cfg, lat):
    """The Gaussian packet of the input file as a unit vector (16 ns).  The envelope exp(-|x - c|^2/(4 sigma^2))
    exp(i q.(x - c)) multiplies the spinor of the chosen branch of the free lattice operator at the central momentum q
    (the projector of the branch applied to a reference spinor of definite spin along x^3), times the chosen isospin.
    If every active direction is periodic, each Fourier component of the packet is then projected on the chosen
    branch at its own momentum, so that the packet contains that branch only (no Zitterbewegung)."""
    ns = lat.ns
    q = cfg["packet_momentum"]
    cen = cfg["packet_center"] or [0.5 * (lat.n[j] - 1) * lat.ell for j in range(3)]
    sig = cfg["packet_width"]
    part = cfg["packet_part"]
    M = h_plane(q, lat) if part == "particle" else -np.conj(h_plane([-q[0], -q[1], -q[2]], lat))
    e = eps_lat(q, lat)
    sgn = 1.0 if cfg["packet_branch"] == "positive" else -1.0
    proj = 0.5 * (I4 + sgn * M / e)
    refs = {"up": np.array([1, 0, 1, 0], complex) / math.sqrt(2), "down": np.array([0, 1, 0, 1], complex) / math.sqrt(2)}
    chi = proj @ refs[cfg["packet_spin"]]
    if np.linalg.norm(chi) < 1e-8:
        chi = proj @ refs["down" if cfg["packet_spin"] == "up" else "up"]
    chi = chi / np.linalg.norm(chi)
    iso = np.array([1, 0], complex) if cfg["packet_isospin"] == "up" else np.array([0, 1], complex)
    spinor8 = np.kron(chi, iso)
    amp = np.ones(ns)
    phase = np.zeros(ns)
    for j in range(3):
        if not lat.active[j]:
            continue
        dxj = lat.x[:, j] - cen[j]
        if lat.periodic[j]:
            L = lat.n[j] * lat.ell
            dxj = (dxj + 0.5 * L) % L - 0.5 * L
        amp *= np.exp(-dxj ** 2 / (4 * sig ** 2))
        phase += q[j] * dxj
    f = (amp * np.exp(1j * phase))[:, None] * spinor8[None, :]
    if all(lat.periodic[j] for j in range(3) if lat.active[j]):
        F = np.fft.fftn(f.reshape(lat.n[2], lat.n[1], lat.n[0], 4, 2), axes=(0, 1, 2))
        F = np.einsum("zyxab,zyxbc->zyxac", branch_projectors(lat, part, sgn), F)
        f = np.fft.ifftn(F, axes=(0, 1, 2)).reshape(ns, 8)
    w = np.zeros((ns, 16), complex)
    off = 0 if part == "particle" else 8
    w[:, off:off + 8] = f
    w = w.reshape(-1)
    return w / np.linalg.norm(w)


def expected_velocity(w, cfg, lat):
    """The exact mean velocity of a mode of the free lattice operator on a lattice whose active directions are all
    periodic: the Fourier components of the occupied part (particle or hole), split by the projectors on the two
    branches +-eps_lat(k), weighted with +-grad eps_lat(k).  The mean position of a free mode moves with this velocity,
    apart from the Zitterbewegung of the interference of the two branches (absent if only one branch is present)."""
    ns = lat.ns
    part = cfg["packet_part"]
    off = 0 if part == "particle" else 8
    F = np.fft.fftn(w.reshape(ns, 16)[:, off:off + 8].reshape(lat.n[2], lat.n[1], lat.n[0], 4, 2), axes=(0, 1, 2))
    Pp = branch_projectors(lat, part, 1.0)
    K = kgrid(lat)
    num = np.zeros(3)
    den = 0.0
    for iz in range(lat.n[2]):
        for iy in range(lat.n[1]):
            for ix in range(lat.n[0]):
                phi = F[iz, iy, ix]
                wp = float(np.sum(np.abs(Pp[iz, iy, ix] @ phi) ** 2))
                wm = float(np.sum(np.abs(phi) ** 2)) - wp
                num += (wp - wm) * np.array(grad_eps_lat(K[iz, iy, ix], lat))
                den += wp + wm
    return list(num / den)


# ============================================================================ helpers for the output
def write_csv(path, content, meta, header, rows):
    with open(path, "w", encoding="utf-8") as f:
        f.write("# content: %s\n" % content)
        for k, v in meta:
            f.write("# %s: %s\n" % (k, v))
        f.write(",".join(header) + "\n")
        for r in rows:
            f.write(",".join(fmt(x) for x in r) + "\n")


def fmt(x):
    if isinstance(x, str):
        return x
    if isinstance(x, (int, np.integer)):
        return str(int(x))
    return "%.12g" % x


def center_of_mass(lat, rho):
    """The centre of a density: the ordinary mean along directions with walls; along periodic directions first the
    circular mean c (the angle of sum rho exp(2 pi i x/L), mapped to [0, L)), then the ordinary mean of the positions
    taken in the window [c - L/2, c + L/2) around it (exact for a density that is negligible half a circumference
    away from its centre), mapped to [0, L)."""
    tot = rho.sum()
    out = []
    for j in range(3):
        if not lat.active[j] or tot == 0:
            out.append(0.0)
            continue
        if lat.periodic[j]:
            L = lat.n[j] * lat.ell
            th = 2 * math.pi * lat.x[:, j] / L
            ang = math.atan2(float(np.sum(rho * np.sin(th))), float(np.sum(rho * np.cos(th))))
            c = (ang % (2 * math.pi)) * L / (2 * math.pi)
            y = (lat.x[:, j] - c + 0.5 * L) % L - 0.5 * L
            out.append((c + float(np.sum(rho * y)) / tot) % L)
        else:
            out.append(float(np.sum(rho * lat.x[:, j]) / tot))
    return out


class Report:
    """The summary file: the problem, the checks and the results."""

    def __init__(self, cfg, lat):
        self.lines = []
        self.checks = []
        self.cfg = cfg
        self.add("Revision_SuperLoop, folder test2, program `%s` (Test 2, item 3): %s" % (PROGRAM, cfg["title"] or "(no title)"))
        self.add("input file: %s" % os.path.basename(cfg["_path"]))
        self.add("task: %s" % cfg["task"])
        self.add("lattice: sites %d %d %d, dx = %g, boundary %s, r_W = %g, active directions D = %d, number of sites %d, "
                 "dimension of H_lat %d" % (lat.n[0], lat.n[1], lat.n[2], lat.ell, " ".join(cfg["boundary"]), lat.wilson,
                                            lat.D, lat.ns, 16 * lat.ns))
        self.add("parameters: m = %g, mu = %g, g_v = %g, g_F = %g" % (lat.mass, lat.mu, cfg["g_v"], cfg["g_F"]))

    def add(self, s):
        self.lines.append(s)

    def check(self, ok, text):
        self.checks.append((bool(ok), text))
        print("[%s] %s" % ("PASS" if ok else "FAIL", text))

    def write(self, path):
        with open(path, "w", encoding="utf-8") as f:
            f.write(self.lines[0] + "\n")
            npass = sum(1 for c in self.checks if c[0])
            f.write("%d of %d checks passed\n" % (npass, len(self.checks)))
            for line in self.lines[1:]:
                f.write(line + "\n")
            f.write("checks:\n")
            for ok, text in self.checks:
                f.write("[%s] %s\n" % ("PASS" if ok else "FAIL", text))
        return all(c[0] for c in self.checks)


def hermiticity(H):
    Dm = (H - H.conj().T).tocoo()
    return float(np.max(np.abs(Dm.data))) if Dm.nnz else 0.0


def meta_lines(cfg, lat):
    return [("program", PROGRAM), ("task", cfg["task"]), ("sites", "%d %d %d" % tuple(lat.n)), ("dx", "%g" % lat.ell),
            ("boundary", " ".join(cfg["boundary"])), ("r_W", "%g" % lat.wilson), ("m", "%g" % lat.mass),
            ("mu", "%g" % lat.mu), ("g_v", "%g" % cfg["g_v"]), ("g_F", "%g" % cfg["g_F"])]


# ============================================================================ the time integration (CVODE)
# The tasks evolve and quench integrate dw/dx^0 = F(x^0, w) with CVODE, the variable-order, variable-step linear
# multistep integrator of SUNDIALS 7.8.0 in pure Rust (vendored in PROJECT spacetimes, vendor/sundials_rs-7.8.0),
# through the shared library libt2cvode of the crate test2/rust/t2cvode_c.  That library calls the module
# t2core::cvode of the Rust programs, so that both solvers integrate with the same method, the same settings and the
# same error test: Adams-Moulton formulas of the orders 1 to 12, the corrector of every step solved by fixed-point
# iteration, and the estimated local error e of every step bounded by
#     sqrt( (1/(2n)) sum_k |e_k|^2 / (rtol |w_k| + atol)^2 ) <= 1
# for the n complex unknowns w_k (the real and the imaginary part of a component carry the same error weight
# 1/(rtol |w_k| + atol), so that the test does not depend on the phase of w_k).  The functions of the library are
# called with the standard module ctypes; the state is handed over as 2n doubles (Re w_0, Im w_0, Re w_1, ...), the
# memory layout of a numpy array of the type complex128, so that numpy views it without copying.

_RHS = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.c_double, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double),
                        ctypes.c_size_t, ctypes.c_void_p)
_OUT = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.c_size_t, ctypes.c_double, ctypes.POINTER(ctypes.c_double), ctypes.c_size_t,
                        ctypes.c_void_p)


class _CvStats(ctypes.Structure):
    """the counters of one integration (the C structure t2cv_stats of the library)"""
    _fields_ = [("nsteps", ctypes.c_int64), ("nfevals", ctypes.c_int64), ("netfails", ctypes.c_int64),
                ("nniters", ctypes.c_int64), ("nncfails", ctypes.c_int64), ("qlast", ctypes.c_int64),
                ("hlast", ctypes.c_double)]


_CVLIB = []  # the loaded library and its path, once found


def cvode_library():
    """The shared library libt2cvode (loaded once): the path in the environment variable T2CVODE_LIB, else
    test2/rust/target/release, else $CARGO_TARGET_DIR/release.  Returns (library, path)."""
    if _CVLIB:
        return _CVLIB[0]
    name = {"darwin": "libt2cvode.dylib", "win32": "t2cvode.dll"}.get(sys.platform, "libt2cvode.so")
    here = os.path.dirname(os.path.abspath(__file__))
    cands = []
    if os.environ.get("T2CVODE_LIB"):
        cands.append(os.environ["T2CVODE_LIB"])
    cands.append(os.path.join(here, "..", "rust", "target", "release", name))
    if os.environ.get("CARGO_TARGET_DIR"):
        cands.append(os.path.join(os.environ["CARGO_TARGET_DIR"], "release", name))
    for path in cands:
        if os.path.isfile(path):
            lib = ctypes.CDLL(os.path.abspath(path))
            dp = ctypes.POINTER(ctypes.c_double)
            lib.t2cv_integrate.restype = ctypes.c_int
            lib.t2cv_integrate.argtypes = [ctypes.c_size_t, dp, ctypes.c_double, dp, ctypes.c_size_t, ctypes.c_int,
                                           ctypes.c_double, ctypes.c_double, ctypes.c_int, ctypes.c_int64, _RHS, _OUT,
                                           ctypes.c_void_p, ctypes.POINTER(_CvStats), ctypes.c_char_p, ctypes.c_size_t]
            lib.t2cv_describe.restype = ctypes.c_int
            lib.t2cv_describe.argtypes = [ctypes.c_int, ctypes.c_double, ctypes.c_double, ctypes.c_int, ctypes.c_char_p,
                                          ctypes.c_size_t]
            lib.t2cv_error_budget.restype = ctypes.c_double
            lib.t2cv_error_budget.argtypes = [ctypes.c_double, ctypes.c_double, ctypes.c_int64, ctypes.c_size_t,
                                              ctypes.c_double]
            _CVLIB.append((lib, os.path.abspath(path)))
            return _CVLIB[0]
    raise InputError("the shared library %s of the time integration with CVODE was not found (looked for: %s); build it "
                     "with 'cargo build --release' in the folder test2/rust, or set the environment variable T2CVODE_LIB "
                     "to the path of the file" % (name, ", ".join(cands)))


def py_exponents(text):
    """The exponents of the Rust format '{:e}' ('1e-12', '1.5e-7', '0.0e0') as Python writes them ('1e-12', '1.5e-07',
    '0.0e+00'), exactly as the function py_exponents of the Rust solver, so that the reports of the two solvers read
    alike."""
    return re.sub(r"(?<=[0-9])e(-?)([0-9]+)(?![0-9A-Za-z])",
                  lambda m: "e%s%s" % ("-" if m.group(1) else "+", m.group(2).rjust(2, "0")), text)


class CvodeSettings:
    """The settings of one integration with CVODE, as t2core::cvode::CvodeSettings of the Rust programs: the Adams-
    Moulton formulas up to the order 12 (lmm 0; or BDF up to the order 5, lmm 1) with fixed-point iteration, the
    tolerances rtol and atol, and at most max_steps internal steps between two output times."""

    def __init__(self, rtol, atol, lmm=0, max_order=None, max_steps=10_000_000):
        self.rtol, self.atol, self.lmm = float(rtol), float(atol), int(lmm)
        self.max_order = (12 if self.lmm == 0 else 5) if max_order is None else int(max_order)
        self.max_steps = int(max_steps)

    def describe(self):
        """the one-line description that the Rust programs print (CvodeSettings::describe), from the library"""
        lib, _ = cvode_library()
        buf = ctypes.create_string_buffer(1024)
        code = lib.t2cv_describe(self.lmm, self.rtol, self.atol, self.max_order, buf, len(buf))
        text = buf.value.decode("utf-8")
        if code != 0:
            raise InputError("invalid settings of CVODE: %s" % text)
        return py_exponents(text)

    def error_budget(self, nsteps, n, norm):
        """the accumulated local error nsteps sqrt(2) (rtol norm + atol sqrt(n)) that the error test of CVODE permits
        after nsteps steps for a state of n complex unknowns and of the 2-norm norm (CvodeSettings::error_budget)"""
        lib, _ = cvode_library()
        return float(lib.t2cv_error_budget(self.rtol, self.atol, int(nsteps), int(n), float(norm)))


def cvode_describe_stats(st):
    """the line of the counters, as CvodeStats::describe of the Rust programs"""
    return ("%d steps, %d evaluations of the right side, %d rejected steps, %d corrector iterations (%d convergence "
            "failures), last order %d, last step %.4e" % (st["nsteps"], st["nfevals"], st["netfails"], st["nniters"],
                                                          st["nncfails"], st["qlast"], st["hlast"]))


def cvode_integrate(w0, t0, touts, settings, rhs, out):
    """Integrates dw/dt = rhs(t, w) with CVODE from w(t0) = w0 and calls out(i, t_i, w(t_i)) for every output time t_i of
    touts (increasing, not below t0; an output time equal to t0 gives w0 itself).  w0 is a complex array of any shape
    (a vector for one mode, the matrix components x modes of the quench); rhs(t, w) returns dw/dt as an array of the
    shape of w.  The arrays handed to rhs and out are views of the memory of the library, valid only during the call.
    Returns the counters of CVODE (a dictionary).  A failure of CVODE raises InputError; an exception raised in rhs or
    out stops the integration and is raised again here."""
    lib, _ = cvode_library()
    w0 = np.ascontiguousarray(w0, dtype=np.complex128)
    shape, n = w0.shape, w0.size
    tv = np.ascontiguousarray(touts, dtype=np.float64)
    errors = []

    def view(ptr):
        """the 2n doubles at ptr as a complex array of the shape of w0 (no copy)"""
        return np.ctypeslib.as_array(ptr, shape=(2 * n,)).view(np.complex128).reshape(shape)

    def c_rhs(t, wp, dwp, nn, user):
        try:
            view(dwp)[...] = rhs(t, view(wp))
            return 0
        except BaseException as e:  # raised again after the integration
            errors.append(e)
            return -1

    def c_out(i, t, wp, nn, user):
        try:
            out(int(i), float(t), view(wp))
            return 0
        except BaseException as e:  # raised again after the integration
            errors.append(e)
            return 1

    f_rhs, f_out = _RHS(c_rhs), _OUT(c_out)  # the callbacks must stay alive during the call
    st = _CvStats()
    msg = ctypes.create_string_buffer(2048)
    dp = ctypes.POINTER(ctypes.c_double)
    code = lib.t2cv_integrate(n, w0.ctypes.data_as(dp), float(t0), tv.ctypes.data_as(dp), len(tv), settings.lmm,
                              settings.rtol, settings.atol, settings.max_order, settings.max_steps, f_rhs, f_out, None,
                              ctypes.byref(st), msg, len(msg))
    if errors:
        raise errors[0]
    if code != 0:
        raise InputError("the time integration with CVODE failed: %s" % py_exponents(msg.value.decode("utf-8", "replace")))
    return {k: getattr(st, k) for k, _ in _CvStats._fields_}


# ============================================================================ the tasks
def task_evolve(cfg, lat, gauge, d, rep, base):
    Hp = build_hp(lat, d)
    cache = {}

    def H_at(t):
        """the sparse lattice operator at the time t (the same matrix for all t if the fields are static)"""
        key = 0.0 if gauge.static else round(t, 12)
        if key not in cache:
            if len(cache) > 4:
                cache.clear()
            U, V0 = gauge.at(None if gauge.static else t)
            cache[key] = (build_h0(lat, U, V0) + Hp).tocsr()
        return cache[key]

    H = H_at(0.0)
    herm = hermiticity(H)
    rep.check(herm < 1e-13, "the lattice operator H_lat is Hermitian (largest deviation %.1e)" % herm)
    # the output grid: the time series at x^0 = k output_step (k = 0 .. outputs), the fields every fields_every outputs;
    # CVODE chooses its own internal steps between the output times
    ostep, nout = cfg["output_step"], cfg["outputs"]
    every = cfg["fields_every"] or max(1, nout // 50)
    sett = CvodeSettings(cfg["rtol"], cfg["atol"])
    hb = norm_bound(lat, gauge.vmax(), d)
    rep.add("time integration: %s; output every %g up to x^0 = %g (%d outputs), fields every %d outputs; Hbound = %.6g"
            % (sett.describe(), ostep, ostep * nout, nout, every, hb))
    if cfg["initial"] == "packet":
        w = packet_mode(cfg, lat)
        q = cfg["packet_momentum"]
        rep.add("initial mode: Gaussian packet, centre %s, width %g, momentum %s, branch %s, spin %s, isospin %s, %s; "
                "eps_lat(q) = %.10g" % (cfg["packet_center"] or "centre", cfg["packet_width"], q, cfg["packet_branch"],
                                        cfg["packet_spin"], cfg["packet_isospin"], cfg["packet_part"], eps_lat(q, lat)))
    elif cfg["initial"] == "eigenmode":
        if 16 * lat.ns > 6000:
            raise InputError("initial = eigenmode needs a small lattice (16 x sites <= 6000)")
        ev, vec = np.linalg.eigh(H.toarray())
        k = cfg["eigenmode_index"]
        if not (0 <= k < len(ev)):
            raise InputError("eigenmode_index must lie in 0 .. %d" % (len(ev) - 1))
        w = vec[:, k].copy()
        rep.add("initial mode: eigenvector number %d of H_lat, E_qp = %.12g" % (k, ev[k]))
    else:
        p = cfg["initial_file"]
        if not os.path.isabs(p):
            p = os.path.join(cfg["_dir"], p)
        arr = np.loadtxt(p, ndmin=2)
        if arr.shape != (16 * lat.ns, 2):
            raise InputError("the initial file must have %d lines 're im'" % (16 * lat.ns))
        w = arr[:, 0] + 1j * arr[:, 1]
        w = w / np.linalg.norm(w)
        rep.add("initial mode: from the file %s" % cfg["initial_file"])
    ns = lat.ns
    n = 16 * ns
    series, fields = [], []
    e0 = float(np.real(np.vdot(w, H @ w)))
    stats = {"dn": 0.0, "de": 0.0}

    def record(t, w, with_fields):
        """the series at every output time; the fields on the sites every fields_every outputs"""
        w2 = np.abs(w.reshape(ns, 16)) ** 2
        rp, rh = w2[:, :8].sum(1), w2[:, 8:].sum(1)
        rup = w2[:, 0:8:2].sum(1)  # the particle components with isospin up
        nrm = float(w2.sum())
        e = float(np.real(np.vdot(w, H_at(t) @ w)))
        cm = center_of_mass(lat, rp + rh)
        series.append((t, nrm, e, rh.sum() / nrm, rup.sum() / nrm, cm[0], cm[1], cm[2]))
        if with_fields:
            for s in range(ns):
                fields.append((t, lat.x[s, 0], lat.x[s, 1], lat.x[s, 2], rp[s], rh[s], rup[s]))
        stats["dn"] = max(stats["dn"], abs(nrm - 1.0))
        stats["de"] = max(stats["de"], abs(e - e0))

    def rhs(t, wv):
        """the right side of CVODE, dw/dx^0 = -i H_lat(x^0) w (one operator for static fields)"""
        return -1j * (H_at(t) @ wv)

    touts = [0.0 + k * ostep for k in range(nout + 1)]
    t0 = time.time()
    st = cvode_integrate(w, 0.0, touts, sett, rhs, lambda i, t, wv: record(t, wv, i % every == 0 or i == nout))
    el = time.time() - t0
    rep.add("CVODE: %s" % cvode_describe_stats(st))
    # the accumulated local error that the error test of CVODE permits, steps sqrt(2) (rtol ||w|| + atol sqrt(n)) with
    # ||w|| = 1 and n complex unknowns (CvodeSettings.error_budget).  CVODE estimates the local error of a step from the
    # difference of the predicted and the corrected solution; the true error may exceed the estimate by a modest factor,
    # and the checks allow 10 times the budget: the norm of the mode, which the exact evolution conserves, may change by
    # at most 10 budget, the energy <w_lat|H_lat|w_lat> (static fields) by 2 Hbound times that
    budget = sett.error_budget(st["nsteps"], n, 1.0)
    rep.check(stats["dn"] < 10.0 * budget,
              "the norm of the mode is conserved: largest change %.2e (bound: 10 times the accumulated local error permitted "
              "by CVODE, 10 steps sqrt(2) (rtol + atol sqrt(n_complex)) = %.1e)" % (stats["dn"], 10.0 * budget))
    if gauge.static:
        tol = 20.0 * hb * budget
        rep.check(stats["de"] < tol, "the energy <w_lat|H_lat|w_lat> = %.10g is conserved (static fields): largest change %.2e "
                                     "(bound: 2 Hbound times the bound of the norm, %.1e)" % (e0, stats["de"], tol))
    rep.add("results: energy <w_lat|H_lat|w_lat> at x^0 = 0: %.12g; final norm %.12g; final P_hole = %.6g; final P_up = %.6g; "
            "final centre of the density (%.6g, %.6g, %.6g); computing time %.2f s"
            % (e0, series[-1][1], series[-1][3], series[-1][4], series[-1][5], series[-1][6], series[-1][7], el))
    if cfg["initial"] == "packet" and cfg["pair"] == "none" and gauge.is_free() and \
            all(lat.periodic[j] for j in range(3) if lat.active[j]):
        w0 = packet_mode(cfg, lat)
        vexp = expected_velocity(w0, cfg, lat)
        T = series[-1][0]
        moved = []
        for j in range(3):
            if not lat.active[j]:
                moved.append(0.0)
                continue
            dxj = series[-1][5 + j] - series[0][5 + j]
            L = lat.n[j] * lat.ell
            dxj = (dxj + 0.5 * L) % L - 0.5 * L
            moved.append(dxj / T)
        q = cfg["packet_momentum"]
        g0 = grad_eps_lat(q, lat)
        dv = max(abs(moved[j] - vexp[j]) for j in range(3))
        vscale = max(abs(v) for v in vexp)
        rep.check(dv < 1e-5 * vscale,
                  "the free packet moves with its mean group velocity: expected %s (the group velocity at the central "
                  "momentum is %s), measured %s; deviation %.2e (allowed: 1e-5 of the velocity)" % (["%.8f" % v for v in vexp], ["%.8f" % v for v in g0], ["%.8f" % v for v in moved], dv))
    meta = meta_lines(cfg, lat) + [("gauge", gauge.desc)]
    write_csv(base + "_series.csv", "time series of the mode: norm, energy, probability of the hole part, probability of "
              "the particle part with isospin up, centre of the density (task evolve, %s)" % PROGRAM, meta,
              ["x_LL^0", "norm", "E_expect", "P_hole", "P_up", "x_mean_1", "x_mean_2", "x_mean_3"], series)
    write_csv(base + "_fields.csv", "the densities of the particle part, of the hole part and of the particle part with "
              "isospin up on the sites at the output times (task evolve, %s)" % PROGRAM, meta,
              ["x_LL^0", "x_LL^1", "x_LL^2", "x_LL^3", "density_p", "density_h", "density_up"], fields)


def degenerate_groups(ev, sel):
    """The selected eigenvalues (indices in ascending order) in groups of equal values (within 1e-9): a list of
    (first index, [indices])."""
    groups = []
    for k in sel:
        if groups and abs(ev[k] - ev[groups[-1][1][-1]]) < 1e-9:
            groups[-1][1].append(k)
        else:
            groups.append((k, [k]))
    return groups


def task_spectrum(cfg, lat, gauge, d, rep, base):
    if 16 * lat.ns > 6000:
        raise InputError("the task spectrum diagonalizes a dense matrix: 16 x (number of sites) must not exceed 6000")
    U, V0 = gauge.at(None)
    H = (build_h0(lat, U, V0) + build_hp(lat, d)).tocsr()
    herm = hermiticity(H)
    rep.check(herm < 1e-13, "the lattice operator H_lat is Hermitian (largest deviation %.1e)" % herm)
    t0 = time.time()
    ev, vec = np.linalg.eigh(H.toarray())
    el = time.time() - t0
    phs = float(np.max(np.abs(ev + ev[::-1])))
    rep.check(phs < 1e-10, "the spectrum is symmetric about zero (particle-hole symmetry; largest |E_n + E_(N-1-n)| = "
                           "%.1e)" % phs)
    lo, hi = cfg["spectrum_window"]
    inwin = [k for k in range(len(ev)) if lo <= ev[k] <= hi]
    sel = inwin[: cfg["spectrum_max_states"]]
    rep.add("results: %d eigenvalues from %.10g to %.10g; %d eigenvalues in the window [%g, %g]: %s; computing time %.2f s"
            % (len(ev), ev[0], ev[-1], len(inwin), lo, hi, " ".join("%.10g" % ev[k] for k in inwin), el))
    meta = meta_lines(cfg, lat) + [("gauge", gauge.desc)]
    write_csv(base + "_spectrum.csv", "all eigenvalues of the lattice operator (task spectrum, %s)" % PROGRAM, meta,
              ["index", "E_qp"], [(k, ev[k]) for k in range(len(ev))])
    rows = []
    ns = lat.ns
    for k0, kk in degenerate_groups(ev, sel):
        w2 = sum(np.abs(vec[:, k].reshape(ns, 16)) ** 2 for k in kk)
        e = float(np.mean(ev[kk]))
        for s in range(ns):
            rows.append((k0, e, len(kk), lat.x[s, 0], lat.x[s, 1], lat.x[s, 2], w2[s, :8].sum(), w2[s, 8:].sum()))
    write_csv(base + "_fields.csv", "the densities of the eigenvectors in the energy window, summed over each group of "
              "degenerate eigenvalues (equal within 1e-9; the sum does not depend on the basis that the eigensolver chooses "
              "in a degenerate eigenspace) (task spectrum, %s)" % PROGRAM, meta + [("spectrum_window", "%g %g" % (lo, hi))],
              ["index", "E_qp", "multiplicity", "x_LL^1", "x_LL^2", "x_LL^3", "density_p", "density_h"], rows)
    write_csv(base + "_series.csv", "the pair field on the sites (task spectrum, %s)" % PROGRAM, meta,
              ["x_LL^1", "x_LL^2", "x_LL^3", "d1_re", "d1_im", "d2_re", "d2_im", "d3_re", "d3_im", "abs_d"],
              [(lat.x[s, 0], lat.x[s, 1], lat.x[s, 2], d[s, 0].real, d[s, 0].imag, d[s, 1].real, d[s, 1].imag,
                d[s, 2].real, d[s, 2].imag, float(np.sqrt(np.sum(np.abs(d[s]) ** 2)))) for s in range(ns)])


def anderson(g, x0, beta, tol, maxit, memory=6):
    """Solve the fixed-point equation x = g(x) for a real vector x by Anderson acceleration (memory m = 6, damping
    beta).  With f_k = g(x_k) - x_k and the differences dX, dF of the last iterates and residuals, the coefficients
    gamma minimize |f_k - dF gamma| (normal equations with a relative regularization 1e-12), and
    x_(k+1) = x_k + beta f_k - (dX + beta dF) gamma.  Returns (g(x) of the last iterate, converged, history), the
    history holding (iteration, max |f_k|, extra) with the extra value that g stores in g.last."""
    X, F = [], []
    x = x0.copy()
    hist = []
    for k in range(1, maxit + 1):
        gx = g(x)
        f = gx - x
        ch = float(np.max(np.abs(f))) if len(f) else 0.0
        hist.append((k, ch) + tuple(g.last))
        if ch < tol:
            return gx, True, hist
        X.append(x.copy())
        F.append(f.copy())
        if len(X) > memory + 1:
            X.pop(0)
            F.pop(0)
        if len(F) >= 2:
            dF = np.stack([F[i + 1] - F[i] for i in range(len(F) - 1)], axis=1)
            dX = np.stack([X[i + 1] - X[i] for i in range(len(X) - 1)], axis=1)
            A = dF.T @ dF
            lam = 1e-12 * max(float(np.trace(A)), 1e-300)
            gam = np.linalg.solve(A + lam * np.eye(A.shape[0]), dF.T @ f)
            x = x + beta * f - (dX + beta * dF) @ gam
        else:
            x = x + beta * f
    return g(x), False, hist


def ground_state(cfg, lat, gauge, d0, rep, G):
    """The self-consistent iteration; returns the pair field, the eigenvalues and eigenvectors, and the iterations."""
    if 16 * lat.ns > 6000:
        raise InputError("the self-consistent ground state diagonalizes a dense matrix: 16 x (number of sites) must not "
                         "exceed 6000")
    U, V0 = gauge.at(None)
    H0 = build_h0(lat, U, V0).toarray()
    ns = lat.ns
    norm = lat.ell ** lat.D
    # the normal-state reference energy: -1/2 the sum of the positive eigenvalues of H_lat with d = 0
    evn = np.linalg.eigvalsh(H0)
    e_norm = -0.5 * float(np.sum(evn[evn > 0]))
    line = cfg["pair_constraint"] == "line"
    i0 = cfg["pair_direction"] - 1
    ph = complex(math.cos(cfg["pair_phase"]), math.sin(cfg["pair_phase"]))

    def to_d(x):
        if line:
            d = np.zeros((ns, 3), complex)
            d[:, i0] = ph * x
            return d
        return (x[:3 * ns] + 1j * x[3 * ns:]).reshape(ns, 3)

    def to_x(d):
        if line:
            return np.real(np.conj(ph) * d[:, i0])
        return np.concatenate([d.real.reshape(-1), d.imag.reshape(-1)])

    def full_map(d):
        """the self-consistency map d -> -(G/2) Pi(d)/dx^D, and the energy E_GS(d) measured from the normal state"""
        ev, vec = np.linalg.eigh(H0 + build_hp(lat, d).toarray())
        pos = ev > 0
        dn = -0.5 * G * anomalous(vec[:, pos], ns) / norm
        egs = -0.5 * float(np.sum(ev[pos])) + norm / G * float(np.sum(np.abs(d) ** 2)) - e_norm
        return dn, egs

    def g(x):
        d = to_d(x)
        dn, egs = full_map(d)
        g.last = (float(np.mean(np.sqrt(np.sum(np.abs(d) ** 2, axis=1)))), egs / ns)
        return to_x(dn)
    g.last = (0.0, 0.0)
    t0 = time.time()
    xs, converged, its = anderson(g, to_x(d0), cfg["mixing"], cfg["tolerance"], cfg["max_iterations"])
    el = time.time() - t0
    d = to_d(xs)
    ev, vec = np.linalg.eigh(H0 + build_hp(lat, d).toarray())
    pos = ev > 0
    egs = -0.5 * float(np.sum(ev[pos])) + norm / G * float(np.sum(np.abs(d) ** 2)) - e_norm
    rep.check(converged, "the self-consistent (Anderson) iteration converged: %d iterations, last change of d^iota %.2e "
                         "(tolerance %g, damping %g%s)" % (len(its), its[-1][1], cfg["tolerance"], cfg["mixing"],
                                                           ", constrained to the line of pair_direction" if line else ""))
    dn, _ = full_map(d)
    res = float(np.max(np.abs(dn - d)))
    rep.check(res < 10 * cfg["tolerance"], "the result satisfies the full self-consistency condition d = -(G/2) Pi/dx^D "
              "(all three isospin components, real and imaginary parts): largest residual %.2e" % res)
    nzero = int(np.sum(np.abs(ev) < 1e-9))
    rep.add("ground state at G = %g: E_GS per site (measured from the normal state) = %.12g; number of eigenvalues with "
            "|E_qp| < 1e-9: %d; computing time %.2f s" % (G, egs / ns, nzero, el))
    return d, ev, vec, its, egs


def lattice_gap_root(lat, G):
    """The root Delta of the lattice gap equation (3.21) of a periodic chain,
    1/G = (1/(n dx)) sum_q (1/E^+ + 1/E^-), by bisection (0 if there is no positive root)."""
    n = lat.n[0]
    qs = [2 * math.pi * (k if k <= n // 2 else k - n) / (n * lat.ell) for k in range(n)]
    el = [eps_lat([q, 0, 0], lat) for q in qs]

    def rhs(D):
        return sum(1 / math.sqrt((e - lat.mu) ** 2 + D * D) + 1 / math.sqrt((e + lat.mu) ** 2 + D * D)
                   for e in el) / (n * lat.ell)
    lo, hi = 1e-12, 50.0
    if rhs(lo) < 1 / G:
        return 0.0
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if rhs(mid) > 1 / G:
            lo = mid
        else:
            hi = mid
    return 0.5 * (lo + hi)


def is_uniform_chain(cfg, lat, gauge):
    return (cfg["pair"] == "uniform" and lat.D == 1 and lat.active[0] and lat.periodic[0] and gauge.is_free())


def task_ground_state(cfg, lat, gauge, d0, rep, base):
    G = cfg["coupling"]
    U, V0 = gauge.at(None)
    herm = hermiticity(build_h0(lat, U, V0) + build_hp(lat, d0))
    rep.check(herm < 1e-13, "the lattice operator H_lat is Hermitian (largest deviation %.1e)" % herm)
    rep.add("self-consistency: G = %g, initial pair field '%s' (direction %d, amplitude %g)"
            % (G, cfg["pair"], cfg["pair_direction"], cfg["pair_amplitude"]))
    d, ev, vec, its, egs = ground_state(cfg, lat, gauge, d0, rep, G)
    phs = float(np.max(np.abs(ev + ev[::-1])))
    rep.check(phs < 1e-10, "the spectrum is symmetric about zero (particle-hole symmetry; largest deviation %.1e)" % phs)
    ns = lat.ns
    absd = np.sqrt(np.sum(np.abs(d) ** 2, axis=1))
    if is_uniform_chain(cfg, lat, gauge):
        root = lattice_gap_root(lat, G)
        dev = float(np.max(np.abs(absd - root)))
        rep.check(dev < 1e-7, "the converged pair field is uniform and equals the root %.12g of the lattice gap equation "
                              "(3.21), 1/G = (1/(n_x dx)) sum_q (1/E^+ + 1/E^-) (largest deviation %.1e)" % (root, dev))
    rep.add("results: |d| from %.10g to %.10g, mean %.10g; lowest positive E_qp = %.10g"
            % (absd.min(), absd.max(), absd.mean(), ev[ev > 0].min()))
    meta = meta_lines(cfg, lat) + [("gauge", gauge.desc), ("G", "%g" % G)]
    write_csv(base + "_iterations.csv", "the self-consistent iteration: the largest change of the pair field, the mean of |d| "
              "and the energy per site E_GS measured from the normal state (task ground_state, %s)" % PROGRAM, meta,
              ["iteration", "change", "abs_d_mean", "E_GS"], its)
    write_csv(base + "_spectrum.csv", "the eigenvalues of the lattice operator in the self-consistent ground state (%s)"
              % PROGRAM, meta, ["index", "E_qp"], [(k, ev[k]) for k in range(len(ev))])
    rows = []
    for tag, dd in (("initial", d0), ("final", d)):
        for s in range(ns):
            rows.append((tag, lat.x[s, 0], lat.x[s, 1], lat.x[s, 2], dd[s, 0].real, dd[s, 0].imag, dd[s, 1].real,
                         dd[s, 1].imag, dd[s, 2].real, dd[s, 2].imag, float(np.sqrt(np.sum(np.abs(dd[s]) ** 2)))))
    write_csv(base + "_fields.csv", "the pair field on the sites: the initial guess and the self-consistent result (task "
              "ground_state, %s)" % PROGRAM, meta,
              ["stage", "x_LL^1", "x_LL^2", "x_LL^3", "d1_re", "d1_im", "d2_re", "d2_im", "d3_re", "d3_im", "abs_d"], rows)
    write_csv(base + "_series.csv", "the self-consistent iteration (task ground_state, %s)" % PROGRAM, meta,
              ["iteration", "change", "abs_d_mean", "E_GS"], its)


def task_quench(cfg, lat, gauge, d0, rep, base):
    G, G2 = cfg["coupling"], cfg["quench_coupling"]
    ns = lat.ns
    norm = lat.ell ** lat.D
    U, V0 = gauge.at(None)
    herm = hermiticity(build_h0(lat, U, V0) + build_hp(lat, d0))
    rep.check(herm < 1e-13, "the lattice operator H_lat is Hermitian (largest deviation %.1e)" % herm)
    rep.add("quench: the ground state at G = %g is evolved with G = %g after x^0 = 0" % (G, G2))
    d, ev, vec, its, egs = ground_state(cfg, lat, gauge, d0, rep, G)
    W = vec[:, ev > 0].copy()
    M = W.shape[1]
    n = 16 * ns
    # the output grid: the time series at x^0 = k output_step (k = 0 .. outputs), the fields every fields_every outputs;
    # CVODE chooses its own internal steps between the output times
    ostep, nout = cfg["output_step"], cfg["outputs"]
    every = cfg["fields_every"] or max(1, nout // 50)
    sett = CvodeSettings(cfg["rtol"], cfg["atol"])
    rep.add("time integration of the %d modes (the self-consistent mean-field equations, %d complex unknowns): %s; output "
            "every %g up to x^0 = %g (%d outputs), fields every %d outputs" % (M, n * M, sett.describe(), ostep,
                                                                              ostep * nout, nout, every))
    cacheH = {}

    def H0_at(t):
        key = 0.0 if gauge.static else round(t, 12)
        if key not in cacheH:
            if len(cacheH) > 4:
                cacheH.clear()
            Ut, V0t = gauge.at(None if gauge.static else t)
            cacheH[key] = build_h0(lat, Ut, V0t)
        return cacheH[key]

    def pair_of(W):
        return -0.5 * G2 * anomalous(W, ns) / norm

    def force(H0, W):
        """-i H_lat(d(W)) W: the right side of the self-consistent equations"""
        out = H0 @ W
        apply_pair(pair_blocks(pair_of(W)), W, out)
        return -1j * out

    def diagnostics(W, t):
        dd = pair_of(W)
        nf = float(np.sum(np.abs(W.reshape(ns, 16, M)[:, 8:, :]) ** 2))
        emf = float(np.real(np.sum(np.conj(W) * (H0_at(t) @ W)))) + 2 * norm / G2 * float(np.sum(np.abs(dd) ** 2))
        orth = float(np.max(np.abs(W.conj().T @ W - np.eye(M))))
        return dd, nf, emf, orth

    series, fields = [], []
    dd, nf0, emf0, orth = diagnostics(W, 0.0)
    stats = {"nf": 0.0, "emf": 0.0, "orth": orth}

    def record(t, W, with_fields):
        """the series at every output time; the pair field on the sites every fields_every outputs"""
        dd, nf, emf, orth = diagnostics(W, t)
        absd = np.sqrt(np.sum(np.abs(dd) ** 2, axis=1))
        series.append((t, float(absd.mean()), float(absd[0]), nf, nf - nf0, emf, orth))
        if with_fields:
            for s in range(ns):
                fields.append((t, lat.x[s, 0], lat.x[s, 1], lat.x[s, 2], dd[s, 0].real, dd[s, 0].imag, dd[s, 1].real,
                               dd[s, 1].imag, dd[s, 2].real, dd[s, 2].imag, float(absd[s])))
        stats["nf"] = max(stats["nf"], abs(nf - nf0))
        stats["emf"] = max(stats["emf"], abs(emf - emf0))
        stats["orth"] = max(stats["orth"], orth)

    def rhs(t, Wv):
        """the right side of CVODE for all modes at once, dW/dx^0 = -i H_lat(d(W)) W, with the pair field d(W) computed
        from the modes at every evaluation (the self-consistency of the mean field)"""
        return force(H0_at(t), Wv)

    touts = [0.0 + k * ostep for k in range(nout + 1)]
    t0 = time.time()
    st = cvode_integrate(W, 0.0, touts, sett, rhs, lambda i, t, Wv: record(t, Wv, i % every == 0 or i == nout))
    el = time.time() - t0
    rep.add("CVODE: %s" % cvode_describe_stats(st))
    # the exact equations conserve the fermion number, the orthonormality of the modes and (for static gauge fields) the
    # functional E_MF; CVODE keeps them up to its accumulated local error, budget = steps sqrt(2) (rtol ||W|| + atol
    # sqrt(n M)) with ||W|| = sqrt(M) for M orthonormal modes (CvodeSettings.error_budget); as in the task evolve, the
    # checks allow 10 times the budget for the change dW of the modes.  A change dW changes N_F (a sum of squared
    # moduli) by at most 2 ||W|| ||dW|| and an entry of W^dag W by at most 2 ||dW||.
    budget = sett.error_budget(st["nsteps"], n * M, math.sqrt(M))
    nf_tol = 20.0 * math.sqrt(M) * budget
    rep.check(stats["nf"] < nf_tol, "the fermion number N_F = %.12g is conserved by the self-consistent evolution: largest "
                                    "change %.2e (bound: 2 ||W_modes|| times 10 times the accumulated local error permitted by "
                                    "CVODE, %.1e)" % (nf0, stats["nf"], nf_tol))
    rep.check(stats["orth"] < 20.0 * budget, "the modes stay orthonormal (the Pauli principle): largest |W_modes^dag W_modes - 1| = %.2e "
                                             "(bound: 2 times 10 times the accumulated local error permitted by CVODE, "
                                             "%.1e)" % (stats["orth"], 20.0 * budget))
    if gauge.static:
        rep.add("the conserved mean-field functional E_MF = %.12g changes by at most %.3e (CVODE keeps it up to its "
                "accumulated local errors)" % (emf0, stats["emf"]))
    absd = np.array([r[1] for r in series])
    tt = np.array([r[0] for r in series])
    rep.add("results: |d| at x^0 = 0: %.10g; mean of |d| over the run after the quench %.10g; smallest and largest |d| "
            "after the quench %.10g and %.10g; computing time %.2f s"
            % (absd[0], absd[1:].mean(), absd[1:].min(), absd[1:].max(), el))
    wfit, afit = fit_frequency(tt[1:], absd[1:], 0.2, 6.0, 5801)
    rep.add("the amplitude of the pair field oscillates with the angular frequency %.6f (least-squares fit of a constant "
            "plus a cosine and a sine of that angular frequency of x^0 to |d| for x^0 > 0, the angular frequency scanned from "
            "0.2 to 6 in steps of 0.001 and refined by a parabola; fitted amplitude %.6g); twice the mean of |d| after the "
            "quench is %.6f" % (wfit, afit, 2 * absd[1:].mean()))
    meta = meta_lines(cfg, lat) + [("gauge", gauge.desc), ("G", "%g" % G), ("quench_coupling", "%g" % G2)]
    write_csv(base + "_iterations.csv", "the self-consistent iteration of the initial ground state (task quench, %s)"
              % PROGRAM, meta, ["iteration", "change", "abs_d_mean", "E_GS"], its)
    write_csv(base + "_spectrum.csv", "the eigenvalues of the lattice operator in the initial ground state (task quench, %s)"
              % PROGRAM, meta, ["index", "E_qp"], [(k, ev[k]) for k in range(len(ev))])
    write_csv(base + "_series.csv", "time series after the quench: the mean of |d| over the sites, |d| at the first site, the "
              "fermion number and its change since x^0 = 0, the conserved functional E_MF and the largest deviation from "
              "orthonormality (task quench, %s)"
              % PROGRAM, meta, ["x_LL^0", "abs_d_mean", "abs_d_site0", "N_F", "N_F_change", "E_MF", "ortho_error"], series)
    write_csv(base + "_fields.csv", "the pair field on the sites at the output times (task quench, %s)" % PROGRAM, meta,
              ["x_LL^0", "x_LL^1", "x_LL^2", "x_LL^3", "d1_re", "d1_im", "d2_re", "d2_im", "d3_re", "d3_im", "abs_d"], fields)


def fit_frequency(t, a, wlo, whi, nw):
    """The angular frequency w of the best least-squares fit of a0 + b cos(w t) + c sin(w t) to the samples a(t): the
    residual is computed on the grid w = wlo .. whi (nw points); the minimum is refined by the parabola through it and
    its two neighbours.  Returns (w, amplitude sqrt(b^2 + c^2))."""
    t = np.asarray(t, float)
    a = np.asarray(a, float)

    def resid(w):
        A = np.stack([np.ones_like(t), np.cos(w * t), np.sin(w * t)], axis=1)
        coef = np.linalg.solve(A.T @ A, A.T @ a)
        r = a - A @ coef
        return float(r @ r), coef
    ws = np.linspace(wlo, whi, nw)
    rs = [resid(w)[0] for w in ws]
    k = int(np.argmin(rs))
    k = min(max(k, 1), nw - 2)
    r0, r1, r2 = rs[k - 1], rs[k], rs[k + 1]
    h = ws[1] - ws[0]
    den = r0 - 2 * r1 + r2
    w = ws[k] + (0.5 * h * (r0 - r2) / den if den > 0 else 0.0)
    coef = resid(w)[1]
    return w, float(math.hypot(coef[1], coef[2]))


# ============================================================================ the self-test
def apply_reference(lat, U, V0, d, w):
    """H_lat w computed site by site with loops, exactly as t2core::Lattice::apply (independent of build_h0)."""
    ns = lat.ns
    out = np.zeros(16 * ns, complex)
    wl = w.reshape(ns, 16)
    nb = [(lat.neighbours(j, True), lat.neighbours(j, False)) for j in range(3)]

    def hmv(f, s):
        ps = f(s)
        o = lat.mass * np.kron(BETA, I2) @ ps
        for j in range(3):
            if not lat.active[j]:
                continue
            sp, sm = nb[j][0][s], nb[j][1][s]
            wf = np.kron(I4, U[s, j]) @ f(sp) if sp >= 0 else np.zeros(8, complex)
            wb = np.kron(I4, U[sm, j].conj().T) @ f(sm) if sm >= 0 else np.zeros(8, complex)
            diff = (wf - wb) / (2 * lat.ell)
            wil = wf + wb - 2 * ps
            o = o - 1j * np.kron(ALPHA[j], I2) @ diff - lat.wilson / (2 * lat.ell) * np.kron(BETA, I2) @ wil
        o = o - np.kron(I4, V0[s]) @ ps - lat.mu * ps
        return o

    def part(s):
        return wl[s, :8]

    def hole_conj(s):
        return np.conj(wl[s, 8:])

    for s in range(ns):
        P = sum(d[s, i] * KDAG[i] for i in range(3))
        out[16 * s:16 * s + 8] = hmv(part, s) + P @ wl[s, 8:]
        out[16 * s + 8:16 * s + 16] = P.conj().T @ wl[s, :8] - np.conj(hmv(hole_conj, s))
    return out


def selftest(report=None):
    rng = np.random.default_rng(20261117)
    lines = []

    def say(s):
        """print a line of the self-test and keep it for the report"""
        lines.append(s)
        print(s)
    say("self-test of %s" % PROGRAM)
    ok_all = True
    # 1. the sparse operator against the site-by-site implementation: random links, potentials and pair field, 3D
    cfg = defaults()
    cfg.update({"sites": [3, 4, 2], "spacing": 0.7, "boundary": ["periodic", "wall", "periodic"], "wilson": 0.8,
                "mass": 0.3, "mu": 0.25, "g_v": 0.7, "g_F": 0.45})
    lat = Lattice(cfg)
    U = np.zeros((lat.ns, 3, 2, 2), complex)
    for s in range(lat.ns):
        for j in range(3):
            U[s, j] = link(0.7, 0.45, rng.uniform(-1, 1, 3), rng.uniform(-1, 1), lat.ell)
    V0 = np.array([potential(0.7, 0.45, rng.uniform(-1, 1, 3), rng.uniform(-1, 1)) for _ in range(lat.ns)])
    d = rng.uniform(-1, 1, (lat.ns, 3)) + 1j * rng.uniform(-1, 1, (lat.ns, 3))
    H = build_h0(lat, U, V0) + build_hp(lat, d)
    dev = 0.0
    for _ in range(5):
        w = rng.normal(size=16 * lat.ns) + 1j * rng.normal(size=16 * lat.ns)
        dev = max(dev, float(np.max(np.abs(H @ w - apply_reference(lat, U, V0, d, w)))))
    ok = dev < 1e-13
    ok_all &= ok
    say("[%s] the sparse lattice operator equals the site-by-site implementation (3 x 4 x 2 sites, random links, "
          "potentials and pair field, periodic and wall directions): largest deviation %.1e" % ("PASS" if ok else "FAIL", dev))
    herm = hermiticity(H.tocsr())
    ok = herm < 1e-13
    ok_all &= ok
    say("[%s] it is Hermitian: largest deviation %.1e" % ("PASS" if ok else "FAIL", herm))
    # 2. the links: unitary, determinant exp(-2 i dx g_F b), equal to the matrix exponential
    from scipy.linalg import expm
    dl = 0.0
    for _ in range(20):
        a, b = rng.uniform(-2, 2, 3), rng.uniform(-2, 2)
        gen = 0.35 * sum(a[i] * SIG[i + 1] for i in range(3)) + 0.45 * b * I2
        dl = max(dl, float(np.max(np.abs(link(0.7, 0.45, a, b, 0.6) - expm(-1j * 0.6 * gen)))))
    ok = dl < 1e-14
    ok_all &= ok
    say("[%s] the closed form of the links equals the matrix exponential exp(-i dx (g_v a.tau/2 + g_F b)) (20 random "
          "potentials): largest deviation %.1e" % ("PASS" if ok else "FAIL", dl))
    # 3. plane waves with constant potentials along tau_3: exact energies (check AS-1 of item [2])
    cfg2 = defaults()
    cfg2.update({"sites": [16, 1, 1], "spacing": 0.5, "boundary": ["periodic"] * 3, "wilson": 1.0, "mass": 0.4, "mu": 0.2,
                 "potential_0": [0, 0, 0.5, 0.3], "potential_1": [0, 0, 0.8, 0.6], "_dir": "."})
    lat2 = Lattice(cfg2)
    U2, V02 = Gauge(cfg2, lat2).at(None)
    ev = np.linalg.eigvalsh((build_h0(lat2, U2, V02)).toarray())
    expect = []
    for k in range(16):
        q = 2 * math.pi * (k if k <= 8 else k - 16) / (16 * 0.5)
        for qq in (0.5, -0.5):
            a1 = 0.45 * 0.6 + qq * 0.7 * 0.8
            a0 = 0.45 * 0.3 + qq * 0.7 * 0.5
            ep = eps_lat([q - a1, 0, 0], lat2)
            eh = eps_lat([q + a1, 0, 0], lat2)
            expect += [ep - a0 - 0.2, -ep - a0 - 0.2, eh + a0 + 0.2, -eh + a0 + 0.2] * 2
    dev2 = float(np.max(np.abs(np.sort(ev) - np.sort(expect))))
    ok = dev2 < 1e-12
    ok_all &= ok
    say("[%s] plane waves in constant potentials: the 256 eigenvalues of a ring of 16 sites are the energies of check "
          "AS-1 (largest deviation %.1e)" % ("PASS" if ok else "FAIL", dev2))
    # 4. the time integration with CVODE (through the library libt2cvode): the eigenvector v of the operator of check 1
    #    that belongs to its largest eigenvalue E evolves as exp(-i E x^0) v; the largest deviation up to x^0 = 20 is
    #    compared with 10 times the accumulated local error that the error test of CVODE permits
    Hc = H.tocsr()
    evs, vecs = np.linalg.eigh(Hc.toarray())
    e_max, v = float(evs[-1]), vecs[:, -1].copy()
    sett = CvodeSettings(1e-12, 1e-14)
    worst = [0.0]

    def deviation(i, t, wv):
        """the distance of the computed state from the exact state exp(-i E x^0) v at the output time t"""
        worst[0] = max(worst[0], float(np.linalg.norm(wv - np.exp(-1j * e_max * t) * v)))
    try:
        st = cvode_integrate(v, 0.0, [0.0 + k * 0.5 for k in range(41)], sett, lambda t, wv: -1j * (Hc @ wv), deviation)
        bound = 10.0 * sett.error_budget(st["nsteps"], v.size, 1.0)
        ok = worst[0] < bound
        say("[%s] the time integration with CVODE: the eigenvector of the largest eigenvalue E = %.6f of the operator of "
            "check 1 evolves as exp(-i E x^0) up to x^0 = 20: largest deviation %.1e (bound: 10 times the accumulated local "
            "error permitted by CVODE, %.1e; %d steps)" % ("PASS" if ok else "FAIL", e_max, worst[0], bound, st["nsteps"]))
    except InputError as e:
        ok = False
        say("[FAIL] the time integration with CVODE failed: %s" % e)
    ok_all &= ok
    # 5. the examples of this program are identical to the input files of the folder test2/examples
    exdir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "examples")
    same = 0
    for name in EXAMPLES:
        path = os.path.join(exdir, name + ".t2in")
        if os.path.isfile(path):
            with open(path, encoding="utf-8") as f:
                same += f.read() == example_text(name)
    ok = same == len(EXAMPLES)
    ok_all &= ok
    say("[%s] the %d examples of this program are identical to the input files of the folder test2/examples (%d of %d)"
        % ("PASS" if ok else "FAIL", len(EXAMPLES), same, len(EXAMPLES)))
    say("self-test: %s" % ("ALL PASS" if ok_all else "FAILED"))
    if report:
        checks = [s for s in lines if s.startswith("[")]
        with open(report, "w", encoding="utf-8") as f:
            f.write("Revision_SuperLoop, folder test2, program `%s` (Test 2, item 3): the self-test of the lattice operator, "
                    "of the time integration and of the examples of the Python solver\n" % PROGRAM)
            f.write("%d of %d checks passed\n" % (sum(1 for s in checks if s.startswith("[PASS]")), len(checks)))
            for s in lines:
                f.write(s + "\n")
    return ok_all


# ============================================================================ main
def solve(path, out=None):
    cfg = read_input(path)
    lat = Lattice(cfg)
    gauge = Gauge(cfg, lat)
    d = pair_profile(cfg, lat)
    if out:
        base = os.path.abspath(out)
    else:
        base = cfg["output"]
        if not os.path.isabs(base):
            base = os.path.join(cfg["_dir"], base)
    os.makedirs(os.path.dirname(base) or ".", exist_ok=True)
    for suffix in ("_summary.txt", "_series.csv", "_fields.csv", "_spectrum.csv", "_iterations.csv"):
        if os.path.exists(base + suffix):
            os.remove(base + suffix)
    rep = Report(cfg, lat)
    rep.add("pair field: %s, direction %d, amplitude %g, phase %g, width %g, positions %s" % (
        cfg["pair"], cfg["pair_direction"], cfg["pair_amplitude"], cfg["pair_phase"], cfg["pair_width"], cfg["pair_position"]))
    rep.add("gauge fields: %s" % gauge.desc)
    if not gauge.static and cfg["task"] in ("spectrum", "ground_state"):
        rep.add("note: the task %s ignores the pulse" % cfg["task"])
    print("%s: task %s, %d sites, dimension %d" % (os.path.basename(path), cfg["task"], lat.ns, 16 * lat.ns))
    task = {"evolve": task_evolve, "spectrum": task_spectrum, "ground_state": task_ground_state,
            "quench": task_quench}[cfg["task"]]
    task(cfg, lat, gauge, d, rep, base)
    files = [base + suffix for suffix in ("_summary.txt", "_series.csv", "_fields.csv", "_spectrum.csv", "_iterations.csv")
             if suffix == "_summary.txt" or os.path.exists(base + suffix)]
    rep.add("output files: %s" % ", ".join(os.path.basename(f) for f in files))
    ok = rep.write(base + "_summary.txt")
    print("wrote %s" % ", ".join(os.path.relpath(f) for f in files))
    print("all checks passed" if ok else "SOME CHECKS FAILED (see %s)" % (base + "_summary.txt"))
    return ok


def keys_markdown(path):
    """Write the table of all keys of the input file (from SPEC, the table that the program itself uses) as a fragment
    of the document of Test 2."""
    kinds = {"str": "text", "float": "number", "float+": "positive number", "float0+": "number >= 0",
             "float01": "number in (0, 1]", "int": "integer", "int+": "positive integer", "int0123": "0, 1, 2 or 3",
             "int123": "1, 2 or 3", "ints3": "three integers", "floats2": "two numbers", "floats3": "three numbers",
             "floats4": "four numbers", "floatlist": "numbers"}
    rows = []
    for k, (kind, default, expl) in SPEC.items():
        if kind.startswith("choice:"):
            kd = "one of: " + ", ".join(kind.split(":", 1)[1].split(","))
        elif kind.startswith("words3:"):
            kd = "three words, each: " + ", ".join(kind.split(":", 1)[1].split(","))
        else:
            kd = kinds[kind]
        df = "required" if default is None else ("(see text)" if default == "" else default)
        need = ", ".join(t for t, v in TASK_KEYS.items() if k in v)
        if default is None and need:
            df = "required for " + need
        rows.append("| `%s` | %s | %s | %s |" % (k, kd, df.replace("|", "/"), expl.replace("|", "/")))
    text = ("<!-- BEGIN t2_py_keys -->\n| key | value | default | meaning |\n|-------------|--------|--------|"
            "--------------------------------|\n" + "\n".join(rows) + "\n<!-- END t2_py_keys -->\n")
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)
    print("wrote %s (%d keys)" % (path, len(rows)))


def list_examples():
    print("Examples (write one with: python3 t2_solve.py --example NAME [FILE]):")
    for k, (desc, _) in EXAMPLES.items():
        print("  %-18s %s" % (k, desc))


def example_text(name):
    """the text of the input file of the example NAME (as the function example_text of the Rust solver)"""
    return ("# example '%s' of the Test 2 solver: %s\n# every key is explained by: python3 t2_solve.py --guide, or by the "
            "Rust solver: t2_solve --guide\n%s" % (name, EXAMPLES[name][0], EXAMPLES[name][1]))


def write_example(name, path=None):
    if name not in EXAMPLES:
        raise InputError("there is no example '%s'" % name)
    path = path or (name + ".t2in")
    with open(path, "w", encoding="utf-8") as f:
        f.write(example_text(name))
    print("wrote the example input file %s" % path)
    return path


def interactive():
    print(__doc__.split("Usage:")[0].strip())
    print()
    print("How to define the initial and the boundary conditions, in short:")
    print("  * boundary conditions: 'sites = n_x n_y n_z' and 'boundary = periodic|wall periodic|wall periodic|wall';")
    print("  * initial condition of a mode: 'initial = packet' with packet_center, packet_width, packet_momentum,")
    print("    packet_branch, packet_spin, packet_isospin, packet_part (or 'eigenmode', or 'file');")
    print("  * fields: potential_0 .. potential_3, gauge_file, magnetic, the pulse, and the pair field 'pair = ...'")
    print("    (the initial guess of the self-consistent tasks).")
    print("Type '?' for the complete guide, 'examples' for the list of examples, 'example NAME' to write one, 'q' to quit.")
    while True:
        try:
            ans = input("\nName of the input file: ").strip()
        except EOFError:
            return 1
        if ans in ("q", "quit", ""):
            return 0
        if ans == "?":
            print(guide_text())
            continue
        if ans == "examples":
            list_examples()
            continue
        if ans.startswith("example "):
            try:
                p = write_example(ans.split(None, 1)[1].strip())
                print("run it by typing its name: %s" % p)
            except InputError as e:
                print("error: %s" % e)
            continue
        try:
            return 0 if solve(ans) else 1
        except InputError as e:
            print("error in the input: %s" % e)
            print("(type '?' for the guide)")


def main(argv):
    try:
        if len(argv) == 1:
            return interactive()
        a = argv[1]
        if a == "--guide":
            print(guide_text())
            return 0
        if a == "--examples":
            list_examples()
            return 0
        if a == "--example":
            if len(argv) < 3:
                raise InputError("--example needs the name of an example (see --examples)")
            write_example(argv[2], argv[3] if len(argv) > 3 else None)
            return 0
        if a == "--selftest":
            return 0 if selftest(argv[2] if len(argv) > 2 else None) else 1
        if a == "--keys-md":
            keys_markdown(argv[2])
            return 0
        out = None
        if len(argv) > 2:
            if argv[2] != "--out" or len(argv) != 4:
                raise InputError("usage: python3 t2_solve.py FILE.t2in [--out BASE]")
            out = argv[3]
        return 0 if solve(a, out) else 1
    except InputError as e:
        print("error in the input: %s" % e)
        return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
