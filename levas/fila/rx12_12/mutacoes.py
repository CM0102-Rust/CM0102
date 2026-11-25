# Mutacoes (rx12_12), pela carreira com a Croacia ate 2002-08-01 (os premios de 30 de maio de 2002 sobem reputacoes).
# - o jogador do ano da primeira divisao com a reputacao nacional de 150;
# - o jogador do ano so entre os goleiros;
# - os premios uma semana antes (23 de maio).
MUTACOES = [
    ('rx12_12_reputacao_menor',
     '1500, 800, d1, 0), // rx12_12: the player of the year',
     '150, 800, d1, 0), // mutation',
     'rx12_12 croatian awards built'),
    ('rx12_12_so_goleiros',
     '[-1, 0, 0x11, -1, -1, -1, -1]); // rx12_12: the player of the year',
     '[-1, 1, 0x11, -1, -1, -1, -1]); // mutation',
     'rx12_12 croatian awards day'),
    ('rx12_12_semana_antes',
     '    if awards_day_is(today, 0x1e, 4) { // rx12_12: 30 May',
     '    if awards_day_is(today, 0x17, 4) { // mutation'),
]
