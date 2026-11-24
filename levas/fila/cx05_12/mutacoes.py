# Mutacoes (cx05_12), carreira ENG ate 2002-06-01 (as minicopas dos torneios de pre-temporada de julho de 2001: 3
# construcoes na cobertura).
# - o banco da minicopa com 5 reservas e nao 9: as escalacoes das partidas do torneio mudam;
# - a final e o terceiro lugar dois dias depois da semi e nao um: as datas dos jogos mudam;
# - o estadio da minicopa e o do segundo clube e nao o da sede: o mando dos jogos muda.
MUTACOES = [
    ('cx05_12_banco_5',
     '    write_u8(this + cup::SUBSTITUTES_NAMED, 9); // cx05_12: the friendly bench',
     '    write_u8(this + cup::SUBSTITUTES_NAMED, 5); // mutation'),
    ('cx05_12_final_dois_dias_depois',
     '    let n = exe::date_add_days(d, next.as_mut_ptr() as usize, 1); // cx05_12: the final the next day',
     '    let n = exe::date_add_days(d, next.as_mut_ptr() as usize, 2); // mutation'),
    ('cx05_12_estadio_do_segundo',
     "    write_i32(this + cup::VENUE_STADIUM_ID, venue_of_club(read_usize(clubs))); // cx05_12: the host's ground",
     "    write_i32(this + cup::VENUE_STADIUM_ID, venue_of_club(read_usize(clubs + 4))); // mutation"),
]
