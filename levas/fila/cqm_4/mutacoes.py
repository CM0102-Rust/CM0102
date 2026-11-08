# Mutacoes (cqm_4), nos construtores das divisoes italianas (a criacao da carreira de 8 ligas).
# - a regra de disciplina das quatro trocada (0x11 -> 0): acusada em 2001-238 na prova do agente;
# - sem as posicoes previstas na criacao (a expectativa da diretoria e a injury_word_0c partem de outro lugar);
# - na Serie A e na Serie B, a marca do historico (+0x4c) comeca em 0 (acusada na prova do agente: o jogo cai em
#   2002-131, no fim da temporada, ao gravar o historico duas vezes).
# Escapou na prova do agente: "sem o calendario real de 2001/02" (a carreira derruba as quatro listas: metade dos nomes
# de 2001/02 nao existe no banco minimo da carreira; ver cqn_5).
MUTACOES = [
    ('cqm4_outra_disciplina',
     '*((this + 0x50) as *mut u8) = 0x11; // cqm4: a regra de disciplina das ligas italianas',
     '*((this + 0x50) as *mut u8) = 0; // mutacao: outra regra de disciplina',
     'cqm_4 new italian division'),
    ('cqm4_sem_posicoes_previstas',
     'previstas(this); // cqm4: as posições previstas',
     'let _ = previstas; // mutacao: sem as posicoes previstas',
     'cqm_4 new italian division'),
    ('cqm4_historico_sem_marca',
     'cqm4_pd(this + 0x4c, 1); // cqm4: a marca do histórico da A e da B',
     'cqm4_pd(this + 0x4c, 0); // mutacao: sem a marca',
     'cqm_4 new italian division'),
]
