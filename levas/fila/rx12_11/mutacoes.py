# Mutacoes (rx12_11), pela carreira com a Grecia ate 2002-08-01 (os premios de 30 de maio de 2002 sobem reputacoes).
# - o jogador do ano com a reputacao nacional de 400;
# - o jogador do ano das divisoes so ate 21 anos;
# - o estrangeiro do ano so entre os goleiros.
MUTACOES = [
    ('rx12_11_reputacao_menor',
     '        year(0, KEY_STAFF_COMP_GREEK_PLAYER_OF_THE_YEAR, 4000, 2000, u32::MAX, 0, 1), // rx12_11: the player of the year',
     '        year(0, KEY_STAFF_COMP_GREEK_PLAYER_OF_THE_YEAR, 400, 2000, u32::MAX, 0, 1), // mutation',
     'rx12_11 greek awards built'),
    ('rx12_11_divisao_so_jovens',
     '[-1, 0, 0x11, -1, -1, -1, -1]); // rx12_11: the division player',
     '[0x15, 0, 0x11, -1, -1, -1, -1]); // mutation',
     'rx12_11 greek awards day'),
    ('rx12_11_estrangeiro_goleiro',
     '[gre, -1, gre, -1, -1, 0, 0x11]); // rx12_11: the foreign player',
     '[gre, -1, gre, -1, -1, 1, 0x11]); // mutation',
     'rx12_11 greek awards day'),
]
