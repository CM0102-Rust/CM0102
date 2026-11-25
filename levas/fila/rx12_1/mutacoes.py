# Mutacoes (rx12_1), pela carreira com a Finlandia ate 2002-08-01 (os premios de 15 de novembro de 2001 sobem
# reputacoes).
# - o jogador do ano com a reputacao nacional de 300;
# - o goleiro do ano escolhido entre os defensores;
# - os premios uma semana depois (22 de novembro).
MUTACOES = [
    ('rx12_1_reputacao_menor',
     '        year(0, KEY_STAFF_COMP_FINNISH_PLAYER_OF_THE_YEAR, 3000, 1500, u32::MAX, 0, 1), // rx12_1: the player of the year',
     '        year(0, KEY_STAFF_COMP_FINNISH_PLAYER_OF_THE_YEAR, 300, 1500, u32::MAX, 0, 1), // mutation',
     'rx12_1 finnish awards built'),
    ('rx12_1_goleiro_defensor',
     '[-1, fin, -1, -1, -1, 1, 0x11]); // rx12_1: the goalkeeper',
     '[-1, fin, -1, -1, -1, 6, 0x11]); // mutation',
     'rx12_1 finnish awards day'),
    ('rx12_1_semana_depois',
     '    if awards_day_is(today, 0xf, 0xa) { // rx12_1: 15 November',
     '    if awards_day_is(today, 0x16, 0xa) { // mutation'),
]
