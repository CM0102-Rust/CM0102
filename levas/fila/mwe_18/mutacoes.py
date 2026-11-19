# Mutacoes (mwe_18): a primeira rodada da Copa da Finlandia uma semana depois (6/5 em vez de 29/4); o 5o colocado da
# Premier entra uma rodada antes (p > 3 em vez de p > 4: outro clube na terceira rodada e um repetido na quarta).
MUTACOES = [
    ('mwe18_primeira_rodada_depois',
     'cql12r((0xf, 3, 0, 5), (0x1d, 3, 0, 5, 1, 0), 0x1e, (0, 3, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwe18: the first round',
     'cql12r((0xf, 3, 0, 5), (6, 4, 0, 5, 1, 0), 0x1e, (0, 3, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mutation: 6 May',
     'mwe_18 finnish cup rounds'),
    ('mwe18_quinto_antes',
     '                1 => p > 4,',
     '                1 => p > 3,',
     'mwe_18 finnish cup entrants'),
]
