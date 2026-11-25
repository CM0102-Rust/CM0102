# Mutacoes (rx09_2), com a Espanha e a Suecia ate 2002-09-01 e a base NORMAL:
# - as linhas do grupo 1 na ordem inversa (o clube k na linha 19 - k): o comparador da tabela acusa na criacao do jogo;
# - a virada pula um ano (ano + 2): o comparador da virada acusa em 2002 e a temporada 2002/03 nao comeca em agosto.
# O advance_stage fica sem mutacao propria: so constroi fases (as mutacoes possiveis param a liga ou derrubam o jogo);
# as fases que ele chama tem as suas no rx09_3 e no rx09_4, e o comparador o confere nas fases que nao constroem.
MUTACOES = [
    ('rx09_2_linhas_invertidas',
     'exe::league_init_table_row(l, k as u32, c, 0, 0); // rx09_2: the row of the k-th club',
     'exe::league_init_table_row(l, 19 - k as u32, c, 0, 0); // mutation: rows reversed',
     'rx09_2 segunda b group 1 table'),
    ('rx09_2_virada_dois_anos',
     'this.set_year(this.year().wrapping_add(1)); // rx09_2: the next season',
     'this.set_year(this.year().wrapping_add(2)); // mutation: two years',
     'rx09_2 segunda b turnover'),
]
