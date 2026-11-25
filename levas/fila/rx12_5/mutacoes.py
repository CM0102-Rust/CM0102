# Mutacoes (rx12_5), pela carreira com as 8 ligas ate 2002-08-01 (os premios de 20 de junho de 2002 sobem
# reputacoes).
# - o jogador do ano com a reputacao nacional de 200;
# - o sub-21 do ano sem o limite de idade;
# - o goleiro do ano escolhido entre os defensores.
MUTACOES = [
    ('rx12_5_reputacao_menor',
     '        year(4, KEY_STAFF_COMP_AUSTRALIAN_PLAYER_OF_THE_YEAR, 2000, 1000, 0, 1), // rx12_5: the player of the year',
     '        year(4, KEY_STAFF_COMP_AUSTRALIAN_PLAYER_OF_THE_YEAR, 200, 1000, 0, 1), // mutation',
     'rx12_5 australian awards built'),
    ('rx12_5_sub21_sem_idade',
     '[0x15, 0, 0x11, -1, -1, -1, -1]); // rx12_5: the under 21 player',
     '[-1, 0, 0x11, -1, -1, -1, -1]); // mutation',
     'rx12_5 australian awards day'),
    ('rx12_5_goleiro_defensor',
     '[-1, 1, 0x11, -1, -1, -1, -1]); // rx12_5: the goalkeeper',
     '[-1, 6, 0x11, -1, -1, -1, -1]); // mutation',
     'rx12_5 australian awards day'),
]
