# Mutacoes (gra_28) pela prova do save (mutacoes.py --prova-save N, com a carga do save de ferias: CARGA). Sem o 4o
# campo (contador): a base da prova do save roda com as rotinas desligadas.
# - o clube de cada time da copa um clube adiante (as copas jogadas depois da carga mudam de times);
# - o clube de cada vaga do livro de inscritos um clube adiante (o comparador, mutacoes.py --carga-comparando);
# - o registro da competicao um adiante (nome e reputacao de outra competicao; o comparador).
MUTACOES = [
    ('gra28_clube_do_time',
     'cup_load_club_from_id(read_usize(this + cup::TEAM_TABLE) + 6 * i as usize); // gra28: the club of a cup team',
     '{ let p = read_usize(this + cup::TEAM_TABLE) + 6 * i as usize; cup_load_club_from_id(p); if read_usize(p) != 0 { write_usize(p, read_usize(p) + club::SIZE); } } // mutation: the next club'),
    ('gra28_clube_do_livro',
     'cup_load_club_from_id(read_usize(this + cup::ENTRANT_BOOK) + 6 * i as usize); // gra28: the club of an entrant book place',
     '{ let p = read_usize(this + cup::ENTRANT_BOOK) + 6 * i as usize; cup_load_club_from_id(p); if read_usize(p) != 0 { write_usize(p, read_usize(p) + club::SIZE); } } // mutation: the next club'),
    ('gra28_competicao_adiante',
     'wrapping_mul(0x6b)) }); // gra28: the competition record',
     'wrapping_mul(0x6b)).wrapping_add(0x6b) }); // mutation: the next competition record'),
]
