# A rodada conjunta da 121a e da 122a levas (com a 0x6db500, que saiu da fila; uma base, um jogo real e uma carreira).
MUTACOES = []
for _n in (121, 122):
    _d = {}
    exec(open('mutacoes/leva%d.py' % _n, encoding='utf-8').read(), _d)
    MUTACOES += _d['MUTACOES']
