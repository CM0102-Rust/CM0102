# Mutacoes (sx04c_16), pela carreira com as 8 ligas ate 2002-08-01 (os premios espanhois de 16 de junho de 2002).
# - o goleiro do ano da First Division escolhido entre os defensores;
# - o jogador do ano com a reputacao nacional de 800;
# - os premios em 16 de agosto (fora da janela).
MUTACOES = [
    ('sx04c_16_goleiro_defensor',
     '(d1, 4, 1, 0x12)',
     '(d1, 4, 6, 0x12) /* mutation */',
     'sx04c_16 spanish awards day'),
    ('sx04c_16_reputacao_menor',
     '        year(1, KEY_STAFF_COMP_SPANISH_PLAYER_OF_THE_YEAR, 8000, 4500, u32::MAX, 0, 1),',
     '        year(1, KEY_STAFF_COMP_SPANISH_PLAYER_OF_THE_YEAR, 800, 4500, u32::MAX, 0, 1), // mutation',
     'sx04c_16 spanish awards built'),
    ('sx04c_16_agosto',
     '    if awards_day_is(today, 0x10, 5) {',
     '    if awards_day_is(today, 0x10, 7) { // mutation'),
]
