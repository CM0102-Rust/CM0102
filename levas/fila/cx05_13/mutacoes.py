# Mutacoes (cx05_13), carreira ENG ate 2002-06-01 (as minligas dos torneios de pre-temporada de julho de 2001: 5
# construcoes na cobertura).
# - o banco da minliga com 5 reservas e nao 9: as escalacoes das partidas mudam;
# - uma rodada a cada dois dias e nao a cada dia: as datas dos jogos mudam;
# - os clubes trocados dois a dois nas linhas da tabela: os confrontos de cada rodada mudam.
MUTACOES = [
    ('cx05_13_banco_5',
     '    write_u8(this + league::SUBSTITUTES_NAMED, 9); // cx05_13: the friendly bench',
     '    write_u8(this + league::SUBSTITUTES_NAMED, 5); // mutation'),
    ('cx05_13_rodada_a_cada_dois_dias',
     '            let n = exe::date_add_days(d, next.as_mut_ptr() as usize, 1); // cx05_13: one round a day',
     '            let n = exe::date_add_days(d, next.as_mut_ptr() as usize, 2); // mutation'),
    ('cx05_13_clubes_trocados',
     '        let club = if clubs != 0 { read_usize(clubs + i as isize as usize * 4) } else { 0 }; // cx05_13: the club of the row',
     '        let club = if clubs != 0 { read_usize(clubs + (i as isize as usize ^ 1) * 4) } else { 0 }; // mutation'),
]
