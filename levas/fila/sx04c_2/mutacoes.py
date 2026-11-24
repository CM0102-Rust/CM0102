# Mutacoes (sx04c_2), pela carreira com as 8 ligas ate 2002-06-01 (os premios japoneses de 20 de dezembro de 2001).
# - o jovem do ano sem limite de idade;
# - o jogador do ano com a reputacao nacional de 500;
# - os premios em 20 de janeiro (nenhum premio japones em dezembro).
MUTACOES = [
    ('sx04c_2_jovem_sem_idade',
     '(3, 0x15, 0x11)] {',
     '(3, -1, 0x11)] { // mutation',
     'sx04c_2 japanese awards day'),
    ('sx04c_2_reputacao_menor',
     '        year(1, KEY_STAFF_COMP_JAPANESE_PLAYER_OF_THE_YEAR, 5500, 1500, 1),',
     '        year(1, KEY_STAFF_COMP_JAPANESE_PLAYER_OF_THE_YEAR, 500, 1500, 1), // mutation',
     'sx04c_2 japanese awards built'),
    ('sx04c_2_janeiro',
     '    if awards_day_is(today, 0x14, 0xb) {',
     '    if awards_day_is(today, 0x14, 0) { // mutation'),
]
