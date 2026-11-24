# Mutacoes (sbE_11), com a Alemanha e a base NORMAL ate 2002-08-01 (a virada alema no verao de 2002):
# - os rebaixados da Regionalliga mantem a secao (a divisao do sbE_3 os coloca de outro jeito: a carreira);
# - o time B nao cai com o clube principal (outra Regionalliga em 2002/03: a carreira).
MUTACOES = [
    ('sbE_11_rebaixados_com_secao',
     'write_usize(row.club() as usize + club::RESERVE_DIVISION, 0); // sbE_11: out of its section',
     'let _ = row; // mutation: the section kept',
     'sbE_11 german lower division up'),
    ('sbE_11_time_b_fica',
     'write_u8(found + league_table_row::STATUS, 3); // sbE_11: the B team goes down with its senior club',
     'write_u8(found + league_table_row::STATUS, read_u8(found + league_table_row::STATUS)); // mutation: it stays',
     'sbE_11 german b team relegated'),
]
