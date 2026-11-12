# PROJECT spacetimes, PAUSED STAGE (see provenance/P11_paused_stages): prototype, not a deliverable.
# Profile of a vortex line: f'' + f'/rho - n^2 f/rho^2 = f (f^2-1)/2, by shooting and by a boundary-value solver (needs numpy, scipy).
# Run: python3 provenance/paused/vortex_prototype_profile.py
import numpy as np
from scipy.integrate import solve_bvp
def rhs(n, r, y):
    f, p = y
    return np.array([p, -p / r + n * n * f / r**2 + 0.5 * f * (f * f - 1)])
def rk4(n, r, y, h):
    k1 = rhs(n, r, y); k2 = rhs(n, r + h / 2, y + h / 2 * k1); k3 = rhs(n, r + h / 2, y + h / 2 * k2); k4 = rhs(n, r + h, y + h * k3)
    return y + h / 6 * (k1 + 2 * k2 + 2 * k3 + k4)
def classify(n, r0, y0, r1, h):
    r, y = r0, y0.copy()
    while r < r1 - 1e-12:
        y = rk4(n, r, y, h); r += h
        if y[0] > 1: return +1
        if y[1] < 0: return -1
    return 0
def start(n, a, r0):
    b = -a / (8 * (n + 1))
    return np.array([a * r0**n + b * r0**(n + 2), n * a * r0**(n - 1) + (n + 2) * b * r0**(n + 1)])
def slope(n, h=0.002, r0=0.002, look=30.0):
    lo, hi = 0.0, 2.0
    for it in range(200):
        mid = 0.5 * (lo + hi)
        if mid == lo or mid == hi: break
        c = classify(n, r0, start(n, mid, r0), look, h)
        if c > 0: hi = mid
        elif c < 0: lo = mid
        else: break
    return 0.5 * (lo + hi), it
for n in (1, 2, 3):
    for h in (0.004, 0.002, 0.001):
        a, it = slope(n, h, r0=h)
        print(n, h, repr(a), it)
print('GP: 0.58319/sqrt2 =', 0.58319 / np.sqrt(2))
# solve_bvp on [1e-6, 60]
for n in (1, 2):
    R = 60.0
    def fun(r, y): return np.vstack([y[1], -y[1] / r + n * n * y[0] / r**2 + 0.5 * y[0] * (y[0]**2 - 1)])
    def bc(ya, yb): return np.array([ya[0], yb[0] - (1 - n * n / R**2 - n * n * (n * n + 8) / 2 / R**4 - (n*n*(n*n+8)**2 - n**6/2) / R**6)])
    x = np.concatenate([[0], np.geomspace(1e-4, R, 4000)])
    x[0] = 1e-8
    y0 = np.vstack([np.tanh(x / 2)**n, np.zeros_like(x)])
    sol = solve_bvp(fun, bc, x, y0, tol=1e-10, max_nodes=400000)
    xs = np.array([1e-3, 2e-3])
    print('bvp n=%d' % n, sol.status, 'slope ~', sol.sol(1e-3)[0] / 1e-3**n * (1 + 1e-6 / (8 * (n + 1))), sol.sol(1.0)[0], sol.sol(5.0)[0])
    # energy constant
    rr = np.geomspace(1e-6, R, 400001)
    f, p = sol.sol(rr)
    integrand = rr * (p**2 + n * n * f**2 / rr**2 + 0.25 * (f**2 - 1)**2)
    I = np.trapezoid(integrand, rr)
    print('   c_n =', I - n * n * np.log(R) - n**4 / (2 * R**2), ' compare ln(1.4643/sqrt2) =', np.log(1.4643 / np.sqrt(2)))
