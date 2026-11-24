# Mutacoes (sx04c_12), pela carreira com 'ENG POL TUR GRE HRV RUS' ate 2002-06-01 (os premios de 30 de novembro de
# 2001).
# - o jogador do ano entre os goleiros (mascara 1);
# - o jogador do ano com a reputacao nacional de 400;
# - os premios em 30 de dezembro.
MUTACOES = [
    ('sx04c_12_goleiro',
     '[-1, -1, -1, read_i32(KEY_NATION_RUSSIA), -1, 0, 0x11]);',
     '[-1, -1, -1, read_i32(KEY_NATION_RUSSIA), -1, 1, 0x11]); // mutation',
     'sx04c_12 russian awards day'),
    ('sx04c_12_reputacao_menor',
     'k(KEY_STAFF_COMP_RUSSIAN_PLAYER_OF_THE_YEAR), 4000, 2000,',
     'k(KEY_STAFF_COMP_RUSSIAN_PLAYER_OF_THE_YEAR), 400, 2000, /* mutation */',
     'sx04c_12 russian awards built'),
    ('sx04c_12_dezembro',
     '    if awards_day_is(today, 0x1e, 0xa) {',
     '    if awards_day_is(today, 0x1e, 0xb) { // mutation'),
]
