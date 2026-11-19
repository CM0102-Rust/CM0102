# Mutacoes (mwe_20): a primeira rodada da Taca de Portugal uma semana depois (26/9 em vez de 19/9); so 120 clubes das
# divisoes de baixo (outros clubes na copa e a lista nao enche).
MUTACOES = [
    ('mwe20_primeira_rodada_depois',
     'cql12r((0xa, 8, 0, 4), (0x13, 8, 0, 6, 1, 0), 0x14, (0, 2, 3), 4, (0x80, 0x40, 0x80, 0), (1, 1, 3), (0, 0xffffffff, 0)), // mwe20: the first round',
     'cql12r((0xa, 8, 0, 4), (0x1a, 8, 0, 6, 1, 0), 0x14, (0, 2, 3), 4, (0x80, 0x40, 0x80, 0), (1, 1, 3), (0, 0xffffffff, 0)), // mutation: 26 September',
     'mwe_20 portuguese cup rounds'),
    ('mwe20_menos_de_baixo',
     'if wanted && 6 * k < 0x300 && portuguese_cup_takes(club, &mut parent) { // mwe20: up to 128 lower clubs',
     'if wanted && 6 * k < 0x2d0 && portuguese_cup_takes(club, &mut parent) { // mutation: up to 120',
     'mwe_20 portuguese cup entrants'),
]
