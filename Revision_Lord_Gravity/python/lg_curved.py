"""
lg_curved.py -- the independent check, in Python with numpy, of the Rust program lg_curved (PROJECT spacetimes, folder
Revision_Lord_Gravity, task C23): Part III of lord_gravity.md, the gravitational coupling of the brochon by Lord's
covariant derivative.

Independence from the Rust program: every matrix is written again from the definitions of the manuscript; the fields are
functions of the four coordinates, differentiated by central finite differences of eighth order (lgcommon.deriv; nested
for second derivatives), where the Rust program uses automatic differentiation; the spin connection is computed from the
anholonomy of the tetrad (formula (14.4) of the manuscript), without Christoffel symbols, where the Rust program uses the
tetrad postulate; local solutions are found by solving the linear equations for all unknown Taylor coefficients at once
(probing the field equation with unit vectors), where the Rust program solves them one after another; the Volkov solution
is coded again.

Checks (the counterparts in the Rust report data/hck_lg_curved_checks.txt in brackets):
  YG-1  the Clifford relation of the curved Dirac matrices and N_LG(sigma^mu, sigma^nu) = g^{mu nu}        [G-2, G-3]
  YG-2  the spin connection of the anholonomy equals that of the tetrad postulate; it keeps gamma^mu
        covariantly constant                                                                                 [G-5, G-6]
  YG-3  the spinor curvature is 1/4 R_{ab mu nu} gamma^{ab} with the Riemann tensor of the metric             [G-8, G-9e]
  YG-4  the square of the Dirac operator (Lichnerowicz) with -R/4 and the gauge field strengths              [G-10]
  YG-5  the identity of the conserved currents (fermion number)                                              [G-12c]
  YG-6  local solutions: nabla_mu T^{mu nu} = the Lorentz force; trace T = m Psi-bar Psi                     [G-13]
  YG-7  the massless brochon in a conformally flat space-time with the zeugon wave: Dirac equation,
        conservation of T, T traceless                                                                       [G-15]
  YG-8  the quasi-particle equation of the polar superfluid: C anticommutes with i Dslash (zeugon), C^2 = -1,
        and (i Dslash)^2 Psi = E_gap^2 Psi for local solutions                                               [G-17]
  YG-9  local Lorentz covariance: the spin connection of the rotated tetrad is S Omega S^{-1} - (dS) S^{-1}  [G-11d]

Run (from the folder Revision_Lord_Gravity):   python3 python/lg_curved.py
Output: data/hck_lg_curved_checks_python.txt
"""
import numpy as np

from lgcommon import ETA, Report, SmoothFn, deriv, expm

RNG = np.random.default_rng(20261123)
NPT = 4
AMP = 0.25            # deviation of the random tetrads from the identity
GAMP = 0.3            # size of the random gauge fields
M_LL, GV, GF = 0.2, 0.7, 0.45
M_CJT, D_CJT = 0.1622929325, 0.3284861084

# ------------------------------------------------------------------------------------------------ matrices
S0 = np.eye(2, dtype=complex)
S1 = np.array([[0, 1], [1, 0]], dtype=complex)
S2 = np.array([[0, -1j], [1j, 0]], dtype=complex)
S3 = np.array([[1, 0], [0, -1]], dtype=complex)
SIG = [S0, S1, S2, S3]
SIGBAR = [S0, -S1, -S2, -S3]
EL = np.array([[0, 1], [-1, 0]], dtype=complex)
Z2 = np.zeros((2, 2), dtype=complex)
GAM = [np.block([[Z2, SIG[a]], [SIGBAR[a], Z2]]) for a in range(4)]          # chiral basis (frame index up)
GCH = 1j * GAM[0] @ GAM[1] @ GAM[2] @ GAM[3]
CLL = np.block([[EL, Z2], [Z2, -EL]])
I2, I4, I8 = np.eye(2), np.eye(4), np.eye(8)


def gab(a, b):
    """gamma^{ab} = 1/2 [gamma^a, gamma^b]"""
    return 0.5 * (GAM[a] @ GAM[b] - GAM[b] @ GAM[a])


def d8(m):
    return np.kron(m, I2)


def i8(t):
    return np.kron(I4, t)


def ndet(x, y):
    return 0.5 * (np.linalg.det(x + y) - np.linalg.det(x) - np.linalg.det(y))


# ------------------------------------------------------------------------------------------------ random fields
def rand_coframe():
    """e^a_mu(x) = delta + AMP r(x), r smooth and real"""
    fs = [[SmoothFn(RNG, AMP, real=True) for _ in range(4)] for _ in range(4)]

    def co(x):
        return np.eye(4) + np.array([[fs[a][m](x).real for m in range(4)] for a in range(4)])
    return co


def rand_gauge(amp=GAMP, zeugon_only=False):
    """W^iota_mu(x) (3 x 4) and B_mu(x) (4), real"""
    ws = [[SmoothFn(RNG, amp, real=True) for _ in range(4)] for _ in range(3)]
    bs = [SmoothFn(RNG, amp, real=True) for _ in range(4)]

    def w(x):
        out = np.array([[ws[i][m](x).real for m in range(4)] for i in range(3)])
        if zeugon_only:
            out[0] = 0.0
            out[1] = 0.0
        return out

    def b(x):
        return np.zeros(4) if zeugon_only else np.array([bs[m](x).real for m in range(4)])
    return w, b


def rand_spinor():
    fs = [SmoothFn(RNG, 1.0) for _ in range(8)]
    return lambda x: np.array([f(x) for f in fs])


def rand_sl2c():
    fa, fb, fc = SmoothFn(RNG, 0.5), SmoothFn(RNG, 0.5), SmoothFn(RNG, 0.5)

    def s(x):
        return expm(np.array([[fa(x), fb(x)], [fc(x), -fa(x)]]))
    return s


# ------------------------------------------------------------------------------------------------ geometry
class Geo:
    """the geometry of a coframe function co(x) (co[a, mu] = e^a_mu)"""

    def __init__(self, co):
        self.co = co

    def fr(self, x):
        return np.linalg.inv(self.co(x)).T                   # fr[a, mu] = e_a^mu

    def g(self, x):
        c = self.co(x)
        return c.T @ ETA @ c

    def gi(self, x):
        f = self.fr(x)
        return f.T @ ETA @ f

    def chris(self, x):
        """Gamma^rho_{mu nu} from finite differences of the metric"""
        dg = np.array([deriv(self.g, x, l) for l in range(4)])      # dg[la, mu, nu]
        gi = self.gi(x)
        t = np.einsum('mln->lmn', dg) + np.einsum('nlm->lmn', dg) - dg  # [la, mu, nu]: d_mu g_la nu + d_nu g_la mu - d_la g_mu nu
        return 0.5 * np.einsum('rl,lmn->rmn', gi, t)

    def omega(self, x):
        """omega_{mu ab} from the anholonomy, without Christoffel symbols:
        C_{a cd} = eta_aa (d_mu e^a_nu - d_nu e^a_mu) e_c^mu e_d^nu, omega_{c ad} = 1/2 (-C_{a cd} - C_{c ad} + C_{d ca}),
        omega_{mu ad} = e^c_mu omega_{c ad}"""
        dco = np.array([deriv(self.co, x, m) for m in range(4)])     # dco[mu, a, nu] = d_mu e^a_nu
        anh = np.einsum('man->amn', dco) - np.einsum('nam->amn', dco)  # [a, mu, nu]
        fr = self.fr(x)
        cl = np.einsum('ab,bmn,cm,dn->acd', ETA, anh, fr, fr)        # C_{a cd}
        wf = 0.5 * (-cl + -np.einsum('cad->acd', cl) + np.einsum('dca->acd', cl))  # wf[a, c, d] = omega_{c ad}
        # wf indices: [a, c, d] -> omega_{c a d}; reorder to om[c, a, d]
        om_c = np.einsum('acd->cad', wf)
        return np.einsum('cm,cad->mad', self.co(x), om_c)         # omega_{mu ad}

    def omega_postulate(self, x):
        """omega_mu^a_b = e^a_nu (d_mu e_b^nu + Gamma^nu_{mu la} e_b^la), lowered: omega_{mu ab}"""
        ch = self.chris(x)
        dfr = np.array([deriv(self.fr, x, m) for m in range(4)])     # [mu, b, nu]
        fr = self.fr(x)
        t = dfr + np.einsum('nml,bl->mbn', ch, fr)                  # [mu, b, nu]
        om_up = np.einsum('an,mbn->mab', self.co(x), t)              # omega_mu^a_b
        return np.einsum('ac,mcb->mab', ETA, om_up)

    def gam(self, x):
        fr = self.fr(x)
        return np.array([sum(fr[a, m] * GAM[a] for a in range(4)) for m in range(4)])

    def spc(self, x):
        om = self.omega(x)
        return np.array([0.25 * sum(om[m, a, b] * gab(a, b) for a in range(4) for b in range(4) if a != b)
                         for m in range(4)])

    def riemann(self, x):
        """R^rho_{sigma mu nu} from nested finite differences of the metric"""
        ch = self.chris(x)
        dch = np.array([deriv(self.chris, x, l) for l in range(4)])  # dch[la, rho, mu, nu]
        r = (np.einsum('mrns->rsmn', dch) - np.einsum('nrms->rsmn', dch)
             + np.einsum('rml,lns->rsmn', ch, ch) - np.einsum('rnl,lms->rsmn', ch, ch))
        return r

    def scalar(self, x):
        r = self.riemann(x)
        ric = np.einsum('rsrn->sn', r)
        return np.einsum('sn,sn->', self.gi(x), ric)


def nabla(geo, w, b, psi, x, mu, spc=None):
    """nabla_mu Psi = d_mu Psi + (Omega_mu (x) 1_2) Psi + i g^v (1_4 (x) W_mu) Psi + i g^F B_mu Psi"""
    sp = geo.spc(x) if spc is None else spc
    wx, bx = w(x), b(x)
    wm = sum(wx[i, mu] * SIG[i + 1] / 2 for i in range(3))
    conn = d8(sp[mu]) + 1j * GV * i8(wm) + 1j * GF * bx[mu] * I8
    return deriv(psi, x, mu) + conn @ psi(x)


def dslash(geo, w, b, psi, x):
    g = geo.gam(x)
    sp = geo.spc(x)
    return sum(d8(g[m]) @ nabla(geo, w, b, psi, x, m, sp) for m in range(4))


def bar(v):
    return v.conj() @ d8(GAM[0])


def resid(geo, w, b, psi, x, m):
    return 1j * dslash(geo, w, b, psi, x) - m * psi(x)


def fstr(w, b, x):
    """W^iota_{mu nu} (with the term -g^v eps W W) and B_{mu nu}, by finite differences"""
    dw = np.array([deriv(w, x, m) for m in range(4)])              # dw[mu, iota, nu]
    db = np.array([deriv(b, x, m) for m in range(4)])              # db[mu, nu]
    wx = w(x)
    eps3 = np.zeros((3, 3, 3))
    eps3[0, 1, 2] = eps3[1, 2, 0] = eps3[2, 0, 1] = 1
    eps3[0, 2, 1] = eps3[2, 1, 0] = eps3[1, 0, 2] = -1
    wf = np.einsum('min->imn', dw) - np.einsum('nim->imn', dw) - GV * np.einsum('ijk,jm,kn->imn', eps3, wx, wx)
    bf = db - db.T
    return wf, bf


def stress(geo, w, b, psi, x, m):
    """T_{mu nu} = -1/2 Im(X_{mu nu} + X_{nu mu}) - g_{mu nu} L, X_{mu nu} = Psi-bar gamma_mu nabla_nu Psi"""
    g = geo.g(x)
    gu = geo.gam(x)
    gl = [sum(g[m2, n] * gu[n] for n in range(4)) for m2 in range(4)]
    sp = geo.spc(x)
    p = psi(x)
    pb = bar(p)
    nab = [nabla(geo, w, b, psi, x, n, sp) for n in range(4)]
    xx = np.array([[pb @ d8(gl[a]) @ nab[n] for n in range(4)] for a in range(4)])
    lag = -np.imag(sum(pb @ d8(gu[n]) @ nab[n] for n in range(4))) - m * (pb @ p).real
    return -0.5 * np.imag(xx + xx.T) - g * lag


def div_t(geo, tfun, x):
    """nabla_mu T^{mu nu} of a tensor function with upper indices"""
    dt = np.array([deriv(tfun, x, m) for m in range(4)])           # dt[la, mu, nu]
    ch = geo.chris(x)
    t = tfun(x)
    return (np.einsum('mmn->n', dt) + np.einsum('mml,ln->n', ch, t) + np.einsum('nml,ml->n', ch, t))


def force(geo, w, b, psi, x):
    wf, bf = fstr(w, b, x)
    gi = geo.gi(x)
    gu = geo.gam(x)
    p = psi(x)
    pb = bar(p)
    jf = np.array([(pb @ d8(gu[l]) @ p).real for l in range(4)])
    ji = np.array([[(pb @ np.kron(gu[l], SIG[i + 1] / 2) @ p).real for l in range(4)] for i in range(3)])
    return GV * np.einsum('na,ial,il->n', gi, wf, ji) + GF * np.einsum('na,al,l->n', gi, bf, jf)


def local_solution(geo, rfun, x0, nre, antilinear=False):
    """a quadratic polynomial spinor Psi(x) = p0 + p1 . (x - x0) + 1/2 (x - x0) p2 (x - x0) whose residual rfun(psi, x)
    and its first derivatives vanish at x0: the unknown coefficients d_0 Psi, d_0 d_mu Psi (40 complex numbers) are
    solved from the 40 linear equations at once (probing the residual with unit vectors; for an antilinear residual the
    real and imaginary parts are unknowns separately)"""
    p0 = RNG.normal(size=8) + 1j * RNG.normal(size=8)
    p1 = RNG.normal(size=(4, 8)) + 1j * RNG.normal(size=(4, 8))
    p2 = RNG.normal(size=(4, 4, 8)) + 1j * RNG.normal(size=(4, 4, 8))
    p2 = 0.5 * (p2 + np.transpose(p2, (1, 0, 2)))

    def build(u):
        q1 = p1.copy()
        q2 = p2.copy()
        q1[0] = u[0:8]
        q2[0, 0] = u[8:16]
        for j in range(1, 4):
            q2[0, j] = q2[j, 0] = u[8 * (j + 1):8 * (j + 2)]

        def psi(x):
            d = x - x0
            return p0 + d @ q1 + 0.5 * np.einsum('m,mnc,n->c', d, q2, d)
        return psi

    def eqs(u):
        psi = build(u)
        r0 = rfun(psi, x0)
        rd = [deriv(lambda y: rfun(psi, y), x0, l) for l in range(4)]
        return np.concatenate([r0] + rd)

    if not antilinear:
        z = np.zeros(40, dtype=complex)
        e0 = eqs(z)
        a = np.zeros((40, 40), dtype=complex)
        for k in range(40):
            u = z.copy()
            u[k] = 1.0
            a[:, k] = eqs(u) - e0
        u = np.linalg.solve(a, -e0)
    else:
        z = np.zeros(40, dtype=complex)
        e0 = eqs(z)
        e0r = np.concatenate([e0.real, e0.imag])
        a = np.zeros((80, 80))
        for k in range(80):
            u = z.copy()
            u[k % 40] = 1.0 if k < 40 else 1j
            d = eqs(u) - e0
            a[:, k] = np.concatenate([d.real, d.imag])
        sol = np.linalg.solve(a, -e0r)
        u = sol[:40] + 1j * sol[40:]
    psi = build(u)
    return psi, np.abs(eqs(u)).max() / np.abs(p0).max()


# ------------------------------------------------------------------------------------------------ Volkov
def mdot(a, b):
    return a[0] * b[0] - a[1:] @ b[1:]


def slash(v):
    return sum(ETA[a, a] * v[a] * GAM[a] for a in range(4))


def volkov(m, amp):
    """the brochon in a plane wave of the zeugon (flat space-time): returns psi(x) (8 components) and A_a(x) (lower)"""
    kd = RNG.normal(size=3)
    kd /= np.linalg.norm(kd)
    kk = 0.7
    k = np.concatenate([[kk], kk * kd])
    r = RNG.normal(size=3)
    av = r - (r @ kd) * kd
    av = amp * av / np.linalg.norm(av)
    a = np.concatenate([[0.0], av])
    ps, us = [], []
    for s in range(2):
        pd = RNG.normal(size=3)
        pd /= np.linalg.norm(pd)
        pm = 0.5
        p = np.concatenate([[np.sqrt(pm * pm + m * m)], pm * pd])
        wv = RNG.normal(size=4) + 1j * RNG.normal(size=4)
        ps.append(p)
        us.append((slash(p) + m * I4) @ wv)

    def amu(x):
        return ETA @ a * np.cos(mdot(k, x))

    def psi(x):
        phi = mdot(k, x)
        out = np.zeros(8, dtype=complex)
        for s in range(2):
            e = GV * (0.5 if s == 0 else -0.5)
            kp, pa, aa = mdot(k, ps[s]), mdot(ps[s], a), mdot(a, a)
            sph = mdot(ps[s], x) + e * pa / kp * np.sin(phi) - e * e * aa / (2 * kp) * (phi / 2 + np.sin(2 * phi) / 4)
            comp = (us[s] + e / (2 * kp) * np.cos(phi) * (slash(k) @ slash(a) @ us[s])) * np.exp(-1j * sph)
            out[s::2] = comp
        return out
    return psi, amu


# ------------------------------------------------------------------------------------------------ the checks
def main():
    rep = Report("PROJECT spacetimes, Revision_Lord_Gravity: lg_curved.py -- the independent check (Python, finite "
                 "differences) of Part III of lord_gravity.md, the gravitational coupling of the brochon",
                 ident="the identifier of the check (YG and a number; the counterparts of the Rust program lg_curved in "
                       "brackets)")
    rep.note("%d random points per check; finite differences of eighth order with the step 1e-2 (nested for second "
             "derivatives), so that identities hold to about 1e-12 relative; tolerances 1e-9" % NPT)
    e1a = e1b = e2a = e2b = e3 = 0.0
    e3lord = np.inf
    for _ in range(NPT):
        x = RNG.uniform(-1, 1, 4)
        geo = Geo(rand_coframe())
        gu = geo.gam(x)
        gi = geo.gi(x)
        fr = geo.fr(x)
        for m in range(4):
            for n in range(4):
                e1a = max(e1a, np.abs(0.5 * (gu[m] @ gu[n] + gu[n] @ gu[m]) - gi[m, n] * I4).max())
                sm = sum(fr[a, m] * SIG[a] for a in range(4))
                sn = sum(fr[a, n] * SIG[a] for a in range(4))
                e1b = max(e1b, abs(ndet(sm, sn) - gi[m, n]))
        om1, om2 = geo.omega(x), geo.omega_postulate(x)
        e2a = max(e2a, np.abs(om1 - om2).max())
        sp = geo.spc(x)
        ch = geo.chris(x)
        for m in range(4):
            dg = deriv(geo.gam, x, m)
            for n in range(4):
                v = dg[n] + sum(ch[n, m, l] * gu[l] for l in range(4)) + sp[m] @ gu[n] - gu[n] @ sp[m]
                e2b = max(e2b, np.abs(v).max())
        # the spinor curvature against the Riemann tensor of the metric
        riem = geo.riemann(x)
        co = geo.co(x)
        rscale = np.abs(np.einsum('rk,ksmn->rsmn', geo.g(x), riem)).max()
        dsp = [deriv(geo.spc, x, m) for m in range(4)]
        for m in range(4):
            for n in range(4):
                lhs = dsp[m][n] - dsp[n][m] + sp[m] @ sp[n] - sp[n] @ sp[m]
                rab = np.einsum('ar,bs,rs->ab', co, fr, riem[:, :, m, n])
                rhs = 0.25 * sum(ETA[a, a] * rab[a, b] * gab(a, b) for a in range(4) for b in range(4) if a != b)
                e3 = max(e3, np.abs(lhs - rhs).max() / rscale)
                if np.abs(rhs).max() > 1e-3 * rscale:
                    e3lord = min(e3lord, np.abs(lhs - 2 * rhs).max() / np.abs(rhs).max())
    rep.add("YG-1a", "[G-2] 1/2 {gamma^mu, gamma^nu} = g^{mu nu} for the curved Dirac matrices gamma^mu = e_a^mu gamma^a", e1a, 1e-12)
    rep.add("YG-1b", "[G-3b] N_LG(sigma^mu, sigma^nu) = g^{mu nu}, sigma^mu = e_a^mu sigma_a", e1b, 1e-12)
    rep.add("YG-2a", "[G-5] the spin connection from the anholonomy of the tetrad (no Christoffel symbols) equals that of the "
            "tetrad postulate with the Christoffel symbols", e2a, 1e-9)
    rep.add("YG-2b", "[G-6] d_mu gamma^nu + Gamma^nu_{mu la} gamma^la + [Omega_mu, gamma^nu] = 0", e2b, 1e-9)
    rep.add("YG-3a", "[G-8a] d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu] = 1/4 R_{ab mu nu} gamma^{ab} with the Riemann "
            "tensor of the metric (nested differences; relative)", e3, 1e-9)
    rep.add_bool("YG-3b", "[G-8b] Lord's printed 1/2 R gamma is twice the spinor curvature", 0.99 < e3lord < 1.01,
                 "smallest relative deviation %.4f (1 for a factor 2)" % e3lord)

    # YG-4: Lichnerowicz
    e4 = 0.0
    for _ in range(NPT):
        x = RNG.uniform(-1, 1, 4)
        geo = Geo(rand_coframe())
        w, b = rand_gauge()
        psi = rand_spinor()
        chi = lambda y: dslash(geo, w, b, psi, y)
        lhs = dslash(geo, w, b, chi, x)
        gi = geo.gi(x)
        ch = geo.chris(x)
        sp = geo.spc(x)
        nabf = [lambda y, n=n: nabla(geo, w, b, psi, y, n) for n in range(4)]
        nab0 = [f(x) for f in nabf]
        boxp = np.zeros(8, dtype=complex)
        for m in range(4):
            for n in range(4):
                nn = nabla(geo, w, b, nabf[n], x, m, sp)
                boxp += gi[m, n] * (nn - sum(ch[l, m, n] * nab0[l] for l in range(4)))
        rs = geo.scalar(x)
        wf, bf = fstr(w, b, x)
        gu = geo.gam(x)
        ft = np.zeros(8, dtype=complex)
        for m in range(4):
            for n in range(4):
                gmn = 0.5 * (gu[m] @ gu[n] - gu[n] @ gu[m])
                fmat = 1j * GV * i8(sum(wf[i, m, n] * SIG[i + 1] / 2 for i in range(3))) + 1j * GF * bf[m, n] * I8
                ft += 0.5 * d8(gmn) @ fmat @ psi(x)
        rhs = boxp - rs / 4 * psi(x) + ft
        e4 = max(e4, np.abs(lhs - rhs).max() / max(np.abs(lhs).max(), np.abs(boxp).max()))
    rep.add("YG-4", "[G-10a] Dslash Dslash Psi = Box Psi - R/4 Psi + i/2 gamma^{mu nu} (g^v W_{mu nu} + g^F B_{mu nu}) Psi in "
            "a curved space-time with non-abelian gauge fields (nested differences; relative)", e4, 1e-9)

    # YG-5: the current identity
    e5 = 0.0
    for _ in range(NPT):
        x = RNG.uniform(-1, 1, 4)
        geo = Geo(rand_coframe())
        w, b = rand_gauge()
        psi = rand_spinor()

        def jvec(y):
            gu = geo.gam(y)
            p = psi(y)
            return np.array([bar(p) @ d8(gu[m]) @ p for m in range(4)])
        ch = geo.chris(x)
        div = sum(deriv(jvec, x, m)[m] for m in range(4)) + np.einsum('mml,l->', ch, jvec(x))
        e = resid(geo, w, b, psi, x, M_LL)
        p = psi(x)
        rhs = 1j * (bar(e) @ p - bar(p) @ e)
        e5 = max(e5, abs(div - rhs) / max(1.0, abs(rhs)))
    rep.add("YG-5", "[G-12c] nabla_mu j^{F,mu} = i (E-bar Psi - Psi-bar E) for random Psi (relative)", e5, 1e-9)

    # YG-6: local solutions, conservation of T with the Lorentz force
    e6r = e6a = e6b = 0.0
    for _ in range(NPT):
        x0 = RNG.uniform(-1, 1, 4)
        geo = Geo(rand_coframe())
        w, b = rand_gauge()
        psi, res = local_solution(geo, lambda p, y: resid(geo, w, b, p, y, M_LL), x0, 8)
        e6r = max(e6r, res)
        tup = lambda y: geo.gi(y) @ stress(geo, w, b, psi, y, M_LL) @ geo.gi(y)
        div = div_t(geo, tup, x0)
        f = force(geo, w, b, psi, x0)
        tsc = np.abs(tup(x0)).max()
        e6a = max(e6a, np.abs(div - f).max() / tsc)
        tr = np.einsum('mn,mn->', geo.gi(x0), stress(geo, w, b, psi, x0, M_LL))
        p = psi(x0)
        e6b = max(e6b, abs(tr - M_LL * (bar(p) @ p).real) / tsc)
    rep.add("YG-6a", "[G-13a] local solutions of the Dirac equation (all 40 unknown Taylor coefficients solved at once; "
            "residual relative to the field)", e6r, 1e-9)
    rep.add("YG-6b", "[G-13b] nabla_mu T^{mu nu} = g^v W^iota nu_la j^{iota la} + g^F B^nu_la j^{F la} (relative)", e6a, 1e-9)
    rep.add("YG-6c", "[G-13c] g^{mu nu} T_{mu nu} = m_LL Psi-bar Psi (relative)", e6b, 1e-9)

    # YG-7: conformal massless brochon with the zeugon wave
    e7a = e7b = e7c = 0.0
    for _ in range(NPT):
        x = RNG.uniform(-1, 1, 4)
        phi = SmoothFn(RNG, 0.25, real=True)
        s = rand_sl2c()

        def s4(y):
            sy = s(y)
            return np.block([[sy, Z2], [Z2, np.linalg.inv(sy.conj().T)]])

        def lam(y):
            m4 = s4(y)
            mi = np.linalg.inv(m4)
            return np.array([[0.25 * np.trace(mi @ GAM[a] @ m4 @ GAM[bb]) * ETA[bb, bb] for bb in range(4)]
                             for a in range(4)]).real

        geo = Geo(lambda y: np.exp(phi(y).real) * lam(y))
        pf, amu = volkov(0.0, 0.6)
        w = lambda y: np.array([np.zeros(4), np.zeros(4), amu(y)])
        b = lambda y: np.zeros(4)
        psi = lambda y: np.exp(-1.5 * phi(y).real) * (d8(s4(y)) @ pf(y))
        r = resid(geo, w, b, psi, x, 0.0)
        e7a = max(e7a, np.abs(r).max() / np.abs(psi(x)).max())
        tup = lambda y: geo.gi(y) @ stress(geo, w, b, psi, y, 0.0) @ geo.gi(y)
        div = div_t(geo, tup, x)
        f = force(geo, w, b, psi, x)
        tsc = np.abs(tup(x)).max()
        e7b = max(e7b, np.abs(div - f).max() / tsc)
        e7c = max(e7c, abs(np.einsum('mn,mn->', geo.gi(x), stress(geo, w, b, psi, x, 0.0))) / tsc)
    rep.add("YG-7a", "[G-15a] in g = Omega^2 eta with a random local frame, Omega^{-3/2} S Psi_flat (Volkov, massless) solves "
            "the massless Dirac equation with the zeugon wave (relative residual)", e7a, 1e-9)
    rep.add("YG-7b", "[G-15c] nabla_mu T^{mu nu} = the force of the zeugon (relative)", e7b, 1e-9)
    rep.add("YG-7c", "[G-15d] T is traceless (relative)", e7c, 1e-9)

    # YG-8: the quasi-particle equation of the polar superfluid
    k3 = np.kron(CLL @ GCH, EL @ S3)
    pmat = d8(GAM[0]) @ k3.conj().T
    cmap = lambda v: pmat @ v.conj()
    dpair = D_CJT * np.exp(0.7j)
    egap2 = M_CJT ** 2 + D_CJT ** 2
    e8a = e8b = e8c = e8d = 0.0
    for _ in range(NPT):
        x = RNG.uniform(-1, 1, 4)
        geo = Geo(rand_coframe())
        w, b = rand_gauge(zeugon_only=True)
        psi = rand_spinor()
        idp = 1j * dslash(geo, w, b, psi, x)
        idc = 1j * dslash(geo, w, b, lambda y: cmap(psi(y)), x)
        e8a = max(e8a, np.abs(cmap(idp) + idc).max() / np.abs(idp).max())
        e8b = max(e8b, np.abs(cmap(cmap(psi(x))) + psi(x)).max() / np.abs(psi(x)).max())
        rq = lambda p, y: 1j * dslash(geo, w, b, p, y) - M_CJT * p(y) - dpair * cmap(p(y))
        q, res = local_solution(geo, rq, x, 8, antilinear=True)
        e8c = max(e8c, res)
        chi = lambda y: 1j * dslash(geo, w, b, q, y)
        sq = 1j * dslash(geo, w, b, chi, x)
        e8d = max(e8d, np.abs(sq - egap2 * q(x)).max() / (egap2 * np.abs(q(x)).max()))
    rep.add("YG-8a", "[G-17a] C Psi = (gamma^0 (x) 1_2)(K^3)^dag Psi^* anticommutes with i Dslash in a curved space-time with "
            "the zeugon (relative)", e8a, 1e-9)
    rep.add("YG-8b", "[G-17c] C^2 = -1", e8b, 1e-14)
    rep.add("YG-8c", "[G-17e] local solutions of i Dslash Psi = M Psi + D C Psi (the antilinear equations solved for the real "
            "and imaginary parts; residual relative to the field)", e8c, 1e-9)
    rep.add("YG-8d", "[G-17f] (i Dslash)^2 Psi = (M^2 + |D|^2) Psi for them, E_gap = %.10f (relative)" % np.sqrt(egap2), e8d, 1e-8)

    # YG-9: local Lorentz covariance of the spin connection
    e9 = 0.0
    for _ in range(NPT):
        x = RNG.uniform(-1, 1, 4)
        co = rand_coframe()
        s = rand_sl2c()

        def s4(y):
            sy = s(y)
            return np.block([[sy, Z2], [Z2, np.linalg.inv(sy.conj().T)]])

        def lam(y):
            m4 = s4(y)
            mi = np.linalg.inv(m4)
            return np.array([[0.25 * np.trace(mi @ GAM[a] @ m4 @ GAM[bb]) * ETA[bb, bb] for bb in range(4)]
                             for a in range(4)]).real
        geo = Geo(co)
        geo2 = Geo(lambda y: lam(y) @ co(y))
        sp, sp2 = geo.spc(x), geo2.spc(x)
        m4 = s4(x)
        mi = np.linalg.inv(m4)
        for m in range(4):
            law = m4 @ sp[m] @ mi - deriv(s4, x, m) @ mi
            e9 = max(e9, np.abs(law - sp2[m]).max())
    rep.add("YG-9", "[G-11d] the spin connection of the rotated tetrad Lambda e equals S4 Omega S4^{-1} - (d S4) S4^{-1} (the law "
            "of GEE, Lord's corrected (5.5))", e9, 1e-9)

    rep.write("hck_lg_curved_checks_python.txt")
    print("lg_curved.py: %d of %d checks passed" % (rep.npass, rep.n))
    raise SystemExit(0 if rep.npass == rep.n else 1)


if __name__ == "__main__":
    main()
