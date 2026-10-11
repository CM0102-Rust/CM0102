# A rodada conjunta da 116a e da 117a levas (uma base, um jogo real e uma carreira para as duas).
MUTACOES = []
for _n in (116, 117):
    _d = {}
    exec(open('mutacoes/leva%d.py' % _n, encoding='utf-8').read(), _d)
    MUTACOES += _d['MUTACOES']
