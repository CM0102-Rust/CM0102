# Mutacoes (rx13_1), pela carreira ate 2002-08-01 (os premios europeus nascem na criacao do jogo e saem em 16 de junho
# de 2002; a reputacao ganha com um premio muda o mercado):
# - os quatro premios sem reputacao (8000/7000 -> 0/0);
# - o goleiro do ano escolhido entre os defensores (mascara 1 -> 6).
# O construtor fica sem mutacao propria (o molde comum do sx05b_5); o comparador confere a vtable, o id, o tipo e a
# contagem contra a original.
MUTACOES = [
    ('rx13_1_europeus_sem_reputacao',
     'read_u32(key), 8000, 7000, u32::MAX, 1, 0, 0, file as u32, 1])) // rx13_1: 8000 and 7000',
     'read_u32(key), 0, 0, u32::MAX, 1, 0, 0, file as u32, 1])) // mutation',
     'rx13_1 european awards built'),
    ('rx13_1_goleiro_entre_defensores',
     '(KEY_STAFF_COMP_EUROPEAN_GOALKEEPER_OF_THE_YEAR, 1),',
     '(KEY_STAFF_COMP_EUROPEAN_GOALKEEPER_OF_THE_YEAR, 6), // mutation',
     'rx13_1 european awards given'),
]
