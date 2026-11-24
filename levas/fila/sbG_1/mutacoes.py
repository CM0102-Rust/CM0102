# Mutacoes (sbG_1), com as 8 ligas: as minitabelas so decidem depois de junho, entao as duas sao acusadas pelo
# comparador lado a lado (o objeto de rascunho), quando a comparacao chega as eliminatorias de marco de 2002:
# - basta um dos clubes estar na tabela (entram os jogos contra clubes de fora);
# - a data final ignorada (entram os jogos depois dela).
MUTACOES = [
    ('sbG_1_um_clube_basta',
     'if home && away { // sbG_1: both clubs in the table',
     'if home || away { // mutation: one club is enough',
     'sbG_1 sub league result added'),
    ('sbG_1_sem_data_final',
     'year > read_u16(to + 2) || (year == read_u16(to + 2) && day > read_i16(to))',
     '{ let _ = (day, year); false } // mutation: no end date',
     'sbG_1 sub league result added'),
]
