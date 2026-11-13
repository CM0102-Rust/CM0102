# Mutacoes (mwf_18), na virada inglesa de meados de 2002: o resultado do estadio ao contrario (o clube da Conference
# que sobe fica e o ultimo da Third fica, ou o contrario: a divisao de dois clubes muda; o caminho recusado e um caminho
# do proprio original). Acusada pelo estado no fim da carreira, 2002-08-01.
MUTACOES = [
    ('mwf18_estadio_ao_contrario',
     'read_usize(third.competition().addr()), 1) != 0; // mwf18: the Conference club\'s ground',
     'read_usize(third.competition().addr()), 1) == 0; // mutation: the ground test reversed',
     'mwf_18 conference exchange'),
]
