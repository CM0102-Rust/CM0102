# A rodada conjunta da 123a e 124a levas (uma base, um jogo real e uma carreira; ate 01/10/2001 por causa dos penaltis).
MUTACOES = []
for _n in (123, 124):
    _d = {}
    exec(open('mutacoes/leva%d.py' % _n, encoding='utf-8').read(), _d)
    MUTACOES += _d['MUTACOES']
