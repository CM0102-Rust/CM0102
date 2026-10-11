# Retomada da rodada conjunta 118a-120a (a sessao caiu depois de f_favorita): o resto das mutacoes.
MUTACOES = []
for _n in (119, 120):
    _d = {}
    exec(open('mutacoes/leva%d.py' % _n, encoding='utf-8').read(), _d)
    MUTACOES += _d['MUTACOES']
MUTACOES = MUTACOES[[n for n, _, _ in MUTACOES].index('f_goleiro'):]
