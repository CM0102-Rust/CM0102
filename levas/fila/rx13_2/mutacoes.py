# Mutacoes (rx13_2), pela carreira ate 2002-08-01 (os premios internacionais nascem na criacao do jogo e saem em maio e
# junho de 2002; a reputacao ganha com um premio muda o mercado):
# - o futebolista do mundo sem reputacao (9000/8000 -> 0/0).
# O construtor fica sem mutacao propria (o molde comum do sx05b_5); o comparador confere a vtable, o id, o tipo e a
# contagem contra a original.
MUTACOES = [
    ('rx13_2_mundo_sem_reputacao',
     '(KEY_STAFF_COMP_WORLD_FOOTBALLER_OF_THE_YEAR, 9000, 8000),',
     '(KEY_STAFF_COMP_WORLD_FOOTBALLER_OF_THE_YEAR, 0, 0), // mutation',
     'rx13_2 international awards built'),
]
