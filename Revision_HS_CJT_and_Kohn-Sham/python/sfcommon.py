"""
sfcommon.py -- common definitions of the Python programs of Revision_HS_CJT_and_Kohn-Sham (PROJECT spacetimes).

The same objects as the Rust library sfcore, written independently with numpy (the Python programs are an independent
second computation of the Rust results):
  * bi-quaternions (arrays of four complex coefficients) and bi-octonions (pairs (a, b) = a + b e4, Cayley-Dickson
    product (a + b e4)(c + d e4) = (a c - dbar b) + (d a + b cbar) e4);
  * the matrix map M(b) = xi_0 sigma_0 - i sum_j xi_j sigma_j, the points of M31 and the Lorentz matrices L(g);
  * the Dirac matrices of the chiral basis and the 8x8 matrices of the Dirac isodoublet (component 2 d + s, Dirac
    index d = 0..3, isospin index s = 0 up, 1 down);
  * dense Jordan-Wigner matrices of fermion operators;
  * a report of checks written to data/.
Requires: Python 3, numpy (scipy and sympy where a program says so).
"""
import os

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.environ.get("SF_DATA", os.path.join(HERE, "..", "data"))

# ----------------------------------------------------------------------------------------------- Pauli, Dirac
S0 = np.eye(2, dtype=complex)
S1 = np.array([[0, 1], [1, 0]], dtype=complex)
S2 = np.array([[0, -1j], [1j, 0]], dtype=complex)
S3 = np.array([[1, 0], [0, -1]], dtype=complex)
SIG = [S0, S1, S2, S3]
SIGBAR = [S0, -S1, -S2, -S3]
E2 = np.array([[0, 1], [-1, 0]], dtype=complex)  # E = i sigma_2
ETA = np.diag([1.0, -1.0, -1.0, -1.0])


def levi(a, b, c):
    """Levi-Civita symbol of indices 0..2"""
    if len({a, b, c}) < 3:
        return 0.0
    return 1.0 if (a, b, c) in ((0, 1, 2), (1, 2, 0), (2, 0, 1)) else -1.0


def gamma(p):
    """Dirac matrices of the chiral basis: ((0, sigma_p), (sigmabar_p, 0))"""
    z = np.zeros((2, 2), complex)
    return np.block([[z, SIG[p]], [SIGBAR[p], z]])


GCH = 1j * gamma(0) @ gamma(1) @ gamma(2) @ gamma(3)
CC = 1j * gamma(2) @ gamma(0)
BETA = gamma(0)
ALPHA = [None] + [gamma(0) @ gamma(j) for j in (1, 2, 3)]


def d8(a4):
    """a Dirac matrix on the eight components: A (x) 1_2"""
    return np.kron(a4, np.eye(2))


def i8(t2):
    """an isospin matrix on the eight components: 1_4 (x) t"""
    return np.kron(np.eye(4), t2)


def t_gen(iota):
    return i8(SIG[iota] / 2)


def k_scalar(iota):
    """K^iota = C gamma^ch (x) E tau_iota"""
    return np.kron(CC @ GCH, E2 @ SIG[iota])


def h_free(q, m):
    """h(q) = sum_j alpha^j q_j + beta m on the eight components"""
    h = BETA * m
    for j in range(3):
        h = h + ALPHA[j + 1] * q[j]
    return d8(h)


# ----------------------------------------------------------------------------------------------- bi-quaternions
def q_mul(x, y):
    a0, a1, a2, a3 = x
    b0, b1, b2, b3 = y
    return np.array([a0 * b0 - a1 * b1 - a2 * b2 - a3 * b3,
                     a0 * b1 + a1 * b0 + a2 * b3 - a3 * b2,
                     a0 * b2 - a1 * b3 + a2 * b0 + a3 * b1,
                     a0 * b3 + a1 * b2 - a2 * b1 + a3 * b0])


def q_bar(x):
    return np.array([x[0], -x[1], -x[2], -x[3]])


def q_dag(x):
    return np.conj(q_bar(x))


def q_norm(x):
    return np.sum(np.asarray(x) ** 2)


def mll(b):
    return b[0] * S0 - 1j * (b[1] * S1 + b[2] * S2 + b[3] * S3)


def biq_of(m):
    return np.array([np.trace(m) / 2] + [0.5j * np.trace(SIG[j] @ m) for j in (1, 2, 3)])


def o_mul(x, y):
    """bi-octonion product; x = (a, b) means a + b e4"""
    a, b = x
    c, d = y
    return (q_mul(a, c) - q_mul(q_bar(d), b), q_mul(d, a) + q_mul(b, q_bar(c)))


def o_unit(p):
    a = np.zeros(4, complex)
    b = np.zeros(4, complex)
    if p < 4:
        a[p] = 1
    else:
        b[p - 4] = 1
    return (a, b)


def o_sub(x, y):
    return (x[0] - y[0], x[1] - y[1])


def o_add(x, y):
    return (x[0] + y[0], x[1] + y[1])


def o_scale(x, s):
    return (x[0] * s, x[1] * s)


def o_bar(x):
    return (q_bar(x[0]), -x[1])


def o_norm(x):
    return q_norm(x[0]) + q_norm(x[1])


def o_max(x):
    return max(np.max(np.abs(x[0])), np.max(np.abs(x[1])))


def exp_traceless2(x):
    """exp(X) of a traceless 2x2 matrix: cosh(s) 1 + sinh(s)/s X, s^2 = -det X"""
    s2 = -np.linalg.det(x)
    s = np.sqrt(complex(s2))
    if abs(s) < 1e-8:
        return np.eye(2) + x + x @ x / 2
    return np.cosh(s) * np.eye(2) + np.sinh(s) / s * x


def lorentz_generator(rot, boost):
    return sum((0.5 * boost[j] - 0.5j * rot[j]) * SIG[j + 1] for j in range(3))


def lorentz_matrix(mg):
    md = mg.conj().T
    return np.array([[0.5 * np.trace(SIG[p] @ mg @ SIG[q] @ md).real for q in range(4)] for p in range(4)])


def s_spin(mg):
    """S(g) = diag((M^dag)^-1, M)"""
    z = np.zeros((2, 2), complex)
    return np.block([[np.linalg.inv(mg.conj().T), z], [z, mg]])


def s_gen(mu):
    z = np.zeros((2, 2), complex)
    return np.block([[-mu.conj().T, z], [z, mu]])


# ----------------------------------------------------------------------------------------------- Fock space
def fock_ops(n):
    """dense annihilation operators c_0..c_{n-1} on 2^n states (bit k = mode k; Jordan-Wigner sign from the modes
    below k)"""
    dim = 1 << n
    ops = []
    for k in range(n):
        c = np.zeros((dim, dim), complex)
        for b in range(dim):
            if b >> k & 1:
                sign = -1.0 if bin(b & ((1 << k) - 1)).count("1") % 2 else 1.0
                c[b ^ (1 << k), b] = sign
        ops.append(c)
    return ops


# ----------------------------------------------------------------------------------------------- report
class Report:
    def __init__(self, title):
        self.title = title
        self.lines = []
        self.notes = []
        self.npass = 0
        self.n = 0

    def add(self, cid, text, dev, tol):
        ok = bool(dev <= tol)
        self.n += 1
        self.npass += ok
        line = "%s %-7s deviation %.3e (tolerance %.1e): %s" % ("PASS" if ok else "FAIL", cid, dev, tol, text)
        print(line)
        self.lines.append(line)
        return ok

    def add_bool(self, cid, text, ok, detail):
        self.n += 1
        self.npass += bool(ok)
        line = "%s %-7s %s [%s]" % ("PASS" if ok else "FAIL", cid, text, detail)
        print(line)
        self.lines.append(line)
        return ok

    def note(self, s):
        print("NOTE", s)
        self.notes.append(s)

    def write(self, name):
        os.makedirs(DATA, exist_ok=True)
        with open(os.path.join(DATA, name), "w") as f:
            f.write(self.title + "\n")
            f.write("Each line: PASS or FAIL, the identifier of the check (the same identifier as in the Rust program),\n")
            f.write("the measured deviation and its tolerance (or the facts that decide it), and the statement.\n\n")
            for line in self.lines:
                f.write(line + "\n")
            if self.notes:
                f.write("\nNotes:\n")
                for s in self.notes:
                    f.write("  " + s + "\n")
            if self.npass == self.n:
                f.write("\nVERDICT: ALL PASS (%d of %d)\n" % (self.npass, self.n))
            else:
                f.write("\nVERDICT: FAIL (%d of %d passed)\n" % (self.npass, self.n))
