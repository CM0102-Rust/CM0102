# Mutacoes (sx05b_12), pela carreira com a Suecia ate 2002-06-01 (os premios de 1o de dezembro sobem reputacoes).
# - o goleiro do ano escolhido entre os defensores;
# - o jogador do ano com a reputacao nacional de 500;
# - os premios em 1o de janeiro.
MUTACOES = [
    ('sx05b_12_goleiro_defensor',
     '        for (k, mask) in [(3, 1), (4, 6), (5, 0x38), (6, 0x40)] {',
     '        for (k, mask) in [(3, 6), (4, 6), (5, 0x38), (6, 0x40)] { // mutation',
     'sx05b_12 swedish awards day'),
    ('sx05b_12_reputacao_menor',
     '        year(1, KEY_STAFF_COMP_SWEDISH_PLAYER_OF_THE_YEAR, 5000, 2500, 1),',
     '        year(1, KEY_STAFF_COMP_SWEDISH_PLAYER_OF_THE_YEAR, 500, 2500, 1), // mutation',
     'sx05b_12 swedish awards built'),
    ('sx05b_12_janeiro',
     '    if awards_day_is(today, 1, 0xb) {',
     '    if awards_day_is(today, 1, 0) { // mutation'),
]
