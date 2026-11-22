# Mutacoes (gra_29) pela prova do save (mutacoes.py --prova-save N, com a carga do save de ferias: CARGA). Sem o 4o
# campo (contador): a base da prova do save roda com as rotinas desligadas.
# - o registro da competicao um adiante (reputacao e nacao de outra competicao em cada liga; o comparador e talvez a
#   impressao da carga);
# - o adversario de cada registro do confronto direto um clube adiante (o comparador, mutacoes.py --carga-comparando);
# - o clube de cada vaga do livro de inscritos um clube adiante (o comparador).
MUTACOES = [
    ('gra29_competicao_adiante',
     'wrapping_mul(0x6b)) }); // gra29: the competition record',
     'wrapping_mul(0x6b)).wrapping_add(0x6b) }); // mutation: the next competition record'),
    ('gra29_adversario_adiante',
     'league_load_club_from_id(read_usize(row(k) + league_table_row::H2H_ENTRIES) + 0xe * j as usize); // gra29: the opponent of a head-to-head entry',
     '{ let p = read_usize(row(k) + league_table_row::H2H_ENTRIES) + 0xe * j as usize; league_load_club_from_id(p); if read_usize(p) != 0 { write_usize(p, read_usize(p) + club::SIZE); } } // mutation: the next club'),
    ('gra29_clube_do_livro',
     'league_load_club_from_id(read_usize(this + league::ENTRANT_BOOK) + 6 * i as usize); // gra29: the club of an entrant book place',
     '{ let p = read_usize(this + league::ENTRANT_BOOK) + 6 * i as usize; league_load_club_from_id(p); if read_usize(p) != 0 { write_usize(p, read_usize(p) + club::SIZE); } } // mutation: the next club'),
]
