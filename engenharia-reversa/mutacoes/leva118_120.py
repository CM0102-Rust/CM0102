# A rodada conjunta da 118a a 120a levas (uma base, um jogo real e uma carreira para as tres).
MUTACOES = []
for _n in (118, 119, 120):
    _d = {}
    exec(open('mutacoes/leva%d.py' % _n, encoding='utf-8').read(), _d)
    MUTACOES += _d['MUTACOES']
