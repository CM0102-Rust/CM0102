# Mutacoes (mwd_20): a primeira fase da Copa da Turquia uma semana depois (1/11/2001 em vez de 25/10); a Supercopa da
# Grecia uma semana depois (9/8/2001 em vez de 2/8).
MUTACOES = [
    ('mwd20_copa_turca_depois',
     'cql12r((0x11, 9, 0, 1), (0x19, 9, 0, 2, 2, 0), 0xa,',
     'cql12r((0x11, 9, 0, 1), (1, 0xa, 0, 2, 2, 0), 0xa,',
     'mwd_20 turkish cup rounds'),
    ('mwd20_supercopa_grega_depois',
     'cql12r((5, 6, 0, 5), (2, 7, 0, 2, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwd20: the Greek Super Cup match',
     'cql12r((5, 6, 0, 5), (9, 7, 0, 2, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mutation: 9 August',
     'mwd_20 greek super cup round'),
]
