# PROJECT spacetimes, PAUSED STAGE (see provenance/P11_paused_stages): prototype, not a deliverable.
# Algebra of the fermion zero modes on a vortex line of the Vorton-Scalar, with the matrices of data/gamma1644.txt and data/gamma1671.txt.
# Run: python3 provenance/paused/vortex_prototype_algebra.py
import os
import numpy as np
def read(path):
    mats = {}; name = None
    for line in open(path):
        if line.startswith('#'): continue
        if line.startswith('MATRIX'):
            name = line.split()[1]; mats[name] = []
        elif line.strip() and name:
            mats[name].append([int(x) for x in line.split()])
    return {k: np.array(v) for k, v in mats.items()}
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "data") + os.sep
for tag in ('1671', '1644'):
    m = read(R + 'gamma%s.txt' % tag)
    g = [m['gamma%s[%d]' % (tag, a)].astype(complex) for a in range(8)]
    om = np.linalg.multi_dot(g)
    if tag == '1671':
        beta = g[4]; alpha = {j: g[4] @ g[j] for j in (1, 2, 3)}; chi = 1j * om
    else:
        beta = -1j * g[4]; alpha = {j: -g[4] @ g[j] for j in (1, 2, 3)}; chi = om
    b5 = 1j * beta @ chi
    five = [alpha[1], alpha[2], alpha[3], beta, b5]
    I = np.eye(16)
    dev = max(np.abs(x @ y + y @ x - 2 * (i == j) * I).max() for i, x in enumerate(five) for j, y in enumerate(five))
    herm = max(np.abs(x - x.conj().T).max() for x in five)
    A = 1j * beta @ alpha[1]; B = 1j * b5 @ alpha[2]
    Pt = 1j * alpha[1] @ alpha[2] @ alpha[3] @ chi
    print(tag, 'Clifford dev', dev, 'herm dev', herm)
    print('  A,B herm', np.abs(A - A.conj().T).max(), np.abs(B - B.conj().T).max(), 'A2,B2', np.abs(A @ A - I).max(), np.abs(B @ B - I).max(), '[A,B]', np.abs(A @ B - B @ A).max())
    for sa in (1, -1):
        for sb in (1, -1):
            Pi = (I + sa * A) @ (I + sb * B) / 4
            print('  dim(A=%+d,B=%+d) =' % (sa, sb), np.trace(Pi).real, ' alpha3 = s*Pt on it: s=+1 dev', np.abs((alpha[3] - Pt) @ Pi).max(), ' s=-1 dev', np.abs((alpha[3] + Pt) @ Pi).max(),
                  ' eig alpha3:', np.round(np.linalg.eigvalsh(Pi @ alpha[3] @ Pi), 6)[[0, 1, -2, -1]])
    print('  Pt central: max comm', max(np.abs(Pt @ x - x @ Pt).max() for x in five), 'Pt^2-1', np.abs(Pt @ Pt - I).max(), 'tr', np.trace(Pt), 'herm', np.abs(Pt - Pt.conj().T).max())
    print('  product of the five = Pt? dev', np.abs(np.linalg.multi_dot(five) - Pt).max())
    # zero-mode equation, n = +1: [i(a1 c + a2 s) + (beta c + b5 s)] Pi(-,-) = 0 for all theta
    Pi = (I - A) @ (I - B) / 4
    r = max(np.abs((1j * (alpha[1] * np.cos(t) + alpha[2] * np.sin(t)) + beta * np.cos(t) + b5 * np.sin(t)) @ Pi).max() for t in np.linspace(0, 6.28, 17))
    print('  residual of the zero-mode equation n=+1:', r)
    Pi2 = (I - A) @ (I + B) / 4
    r = max(np.abs((1j * (alpha[1] * np.cos(t) + alpha[2] * np.sin(t)) + beta * np.cos(t) - b5 * np.sin(t)) @ Pi2).max() for t in np.linspace(0, 6.28, 17))
    print('  residual of the zero-mode equation n=-1:', r)
    # real or complex? Pt real?
    print('  Pt imaginary part max', np.abs(Pt.imag).max(), ' real part max', np.abs(Pt.real).max())
