# Mutacoes (sx04c_9), pela carreira com 'ENG POL TUR GRE HRV RUS' ate 2002-06-01 (os premios de 30 de maio de 2002).
# - o jovem do ano sem o limite de idade;
# - o jogador do ano da First Division com a reputacao nacional de 300;
# - os premios em 30 de junho (fora da janela).
MUTACOES = [
    ('sx04c_9_jovem_sem_idade',
     'nation_award(this, first + 2), [0x15, 0, 0x11, -1, -1, -1, -1]);',
     'nation_award(this, first + 2), [-1, 0, 0x11, -1, -1, -1, -1]); // mutation',
     'sx04c_9 polish awards day'),
    ('sx04c_9_reputacao_menor',
     '        year(1, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_PLAYER_OF_THE_YEAR, 3000, 0, d1, 1),',
     '        year(1, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_PLAYER_OF_THE_YEAR, 300, 0, d1, 1), // mutation',
     'sx04c_9 polish awards built'),
    ('sx04c_9_junho',
     '    if awards_day_is(today, 0x1e, 4) {\n        covered!("sx04c_9 polish awards day");',
     '    if awards_day_is(today, 0x1e, 5) { // mutation\n        covered!("sx04c_9 polish awards day");'),
]
