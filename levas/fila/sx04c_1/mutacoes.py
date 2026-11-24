# Mutacoes (sx04c_1), pela carreira com as 8 ligas ate 2002-08-01 (os premios italianos de 6 de junho sobem
# reputacoes; o init roda na criacao do jogo).
# - o jovem do ano sem o limite de 23 anos (outro vencedor);
# - o jogador do ano da Serie A com a reputacao nacional de 500;
# - os premios em 6 de agosto (fora da janela).
MUTACOES = [
    ('sx04c_1_jovem_sem_idade',
     '(4, [0x17, 0, 0x11, -1, -1, -1, -1])',
     '(4, [-1, 0, 0x11, -1, -1, -1, -1]) /* mutation */',
     'sx04c_1 italian awards day'),
    ('sx04c_1_reputacao_menor',
     '        year(3, KEY_STAFF_COMP_ITALIAN_SERIE_A_PLAYER_OF_THE_YEAR, 8500, 5000, a, 1),',
     '        year(3, KEY_STAFF_COMP_ITALIAN_SERIE_A_PLAYER_OF_THE_YEAR, 500, 5000, a, 1), // mutation',
     'sx04c_1 italian awards built'),
    ('sx04c_1_agosto',
     '    if awards_day_is(today, 6, 5) {\n        covered!("sx04c_1 italian awards day");',
     '    if awards_day_is(today, 6, 7) { // mutation\n        covered!("sx04c_1 italian awards day");'),
]
