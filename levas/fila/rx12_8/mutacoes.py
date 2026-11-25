# Mutacoes (rx12_8), pela carreira com a Holanda ate 2002-08-01 (as chuteiras de ouro em 20/12/2001 e os premios do
# ano em 25/3/2002 sobem reputacoes).
# - o jogador do ano da primeira divisao com a reputacao nacional de 600;
# - o goleiro do ano escolhido entre os defensores;
# - os premios do ano uma semana depois (1o de abril).
MUTACOES = [
    ('rx12_8_reputacao_menor',
     '6000, 2500, d1, 1, 0, 1), // rx12_8: the player of the year',
     '600, 2500, d1, 1, 0, 1), // mutation',
     'rx12_8 dutch awards built'),
    ('rx12_8_goleiro_defensor',
     '[-1, 1, 0x11, -1, -1, -1, -1]); // rx12_8: the goalkeeper',
     '[-1, 6, 0x11, -1, -1, -1, -1]); // mutation',
     'rx12_8 dutch year awards'),
    ('rx12_8_semana_depois',
     '    if awards_day_is(today, 0x19, 2) { // rx12_8: 25 March',
     '    if awards_day_is(today, 1, 3) { // mutation'),
]
