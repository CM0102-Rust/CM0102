# Mutacoes (mwd_19): a Copa da Turquia com a regra de disciplina inglesa (9): as suspensoes da copa mudam desde setembro de
# 2001; a primeira fase da Copa da Russia uma semana depois (7/7/2002 em vez de 30/6: a Copa da Russia de 2002/03).
MUTACOES = [
    ('mwd19_disciplina_copa_turca',
     'discipline: 0x26, // mwd19: the Turkish Cup discipline rule',
     'discipline: 9, // mutation: the English rule',
     'mwd_19 turkish cup new'),
    ('mwd19_copa_russa_depois',
     'cql12r((0x17, 5, 0, 2), (0x1e, 5, 0, 2, 2, 0), 0x14,',
     'cql12r((0x17, 5, 0, 2), (7, 6, 0, 2, 2, 0), 0x14,',
     'mwd_19 russian cup rounds'),
]
