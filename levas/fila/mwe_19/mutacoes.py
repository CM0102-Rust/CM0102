# Mutacoes (mwe_19): a primeira rodada da Copa da Noruega uma semana depois (12/5 em vez de 5/5); os times reservas
# (com "2" no nome) entram com os das ligas (outros clubes na copa).
MUTACOES = [
    ('mwe19_primeira_rodada_depois',
     'cql12r((7, 3, 0, 2), (5, 4, 0, 2, 2, 0), 0xa, (0, 0x403, 0), 4, (0x80, 0x40, 0x80, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwe19: the first round',
     'cql12r((7, 3, 0, 2), (0xc, 4, 0, 2, 2, 0), 0xa, (0, 0x403, 0), 4, (0x80, 0x40, 0x80, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mutation: 12 May',
     'mwe_19 norwegian cup rounds'),
    ('mwe19_reservas_entram',
     'exe::crt_strstr(club + club::SHORT_NAME, TWO) == 0 { // mwe19: the reserve sides left out',
     'true { // mutation: the reserve sides in',
     'mwe_19 norwegian cup entrants'),
]
