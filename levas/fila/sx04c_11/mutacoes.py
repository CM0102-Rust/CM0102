# Mutacoes (sx04c_11), pela carreira com 'ENG SWE FIN NOR POR' ate 2002-08-01 (os premios de 6 de junho de 2002).
# - o jogador do ano pela estatistica 2 (gols) em vez da 0x11;
# - o tecnico do ano da Premier Division com a reputacao nacional de 600;
# - os premios em 6 de agosto (fora da janela).
MUTACOES = [
    ('sx04c_11_estatistica',
     'for (k, stat) in [(2, 0xc), (3, 2), (1, 0x11)] {',
     'for (k, stat) in [(2, 0xc), (3, 2), (1, 2)] { // mutation',
     'sx04c_11 portuguese awards day'),
    ('sx04c_11_reputacao_menor',
     '        year(5, KEY_STAFF_COMP_PORTUGUESE_PREMIER_LEAGUE_MANAGER_OF_THE, 6000, 3000, d1, 0, 0),',
     '        year(5, KEY_STAFF_COMP_PORTUGUESE_PREMIER_LEAGUE_MANAGER_OF_THE, 600, 3000, d1, 0, 0), // mutation',
     'sx04c_11 portuguese awards built'),
    ('sx04c_11_agosto',
     '    if awards_day_is(today, 6, 5) {',
     '    if awards_day_is(today, 6, 7) { // mutation'),
]
