# Mutacoes (rx09_8), com a Espanha e a Suecia ate 2002-09-01 e a base NORMAL:
# - as linhas da secao Norte na ordem inversa (o clube k na linha 11 - k): o comparador da tabela acusa na criacao do
#   jogo;
# - o time B sem a marca 0xfc (fica 0xff, nao decidido): um time B pode subir no acesso de novembro de 2001 (a carreira
#   acusa se houver time B entre os campeoes de secao; o comparador da tabela nao confere a marca).
# A virada usa o molde do rx09_2 (grouped_league_new_season, com a mutacao do ano la) e o advance_stage so constroi as
# fases do rx09_9 (sem mutacao propria, como o da Segunda B).
MUTACOES = [
    ('rx09_8_linhas_invertidas',
     'exe::league_init_table_row(l, k as u32, c, 0, 0); // rx09_8: the row of the k-th North club',
     'exe::league_init_table_row(l, 11 - k as u32, c, 0, 0); // mutation: rows reversed',
     'rx09_8 swedish second north table'),
    ('rx09_8_time_b_sem_marca',
     'write_u8(row + league_table_row::STATUS, 0xfc); // rx09_8: a B team cannot go up',
     'write_u8(row + league_table_row::STATUS, 0xff); // mutation: no mark',
     'rx09_8 swedish second north table'),
]
