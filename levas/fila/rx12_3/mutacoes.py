# Mutacoes (rx12_3), pela carreira com a Alemanha ate 2002-08-01 (os premios de 6 de junho de 2002 sobem
# reputacoes).
# - o jogador do ano com a reputacao nacional de 750;
# - o goleiro do ano escolhido entre os defensores;
# - os premios uma semana depois (13 de junho).
MUTACOES = [
    ('rx12_3_reputacao_menor',
     '        year(5, KEY_STAFF_COMP_GERMAN_PLAYER_OF_THE_YEAR, 7500, 2500, u32::MAX, 0, 1), // rx12_3: the player of the year',
     '        year(5, KEY_STAFF_COMP_GERMAN_PLAYER_OF_THE_YEAR, 750, 2500, u32::MAX, 0, 1), // mutation',
     'rx12_3 german awards built'),
    ('rx12_3_goleiro_defensor',
     '        for (k, mask) in [(0, 1), (1, 2), (2, 4), (3, 0x10), (4, 0x40)] { // rx12_3: the positions of the year',
     '        for (k, mask) in [(0, 4), (1, 2), (2, 4), (3, 0x10), (4, 0x40)] { // mutation',
     'rx12_3 german awards day'),
    ('rx12_3_semana_depois',
     '    if awards_day_is(today, 6, 5) { // rx12_3: 6 June',
     '    if awards_day_is(today, 0xd, 5) { // mutation'),
]
