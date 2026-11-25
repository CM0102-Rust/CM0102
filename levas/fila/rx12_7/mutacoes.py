# Mutacoes (rx12_7), pela carreira com a Belgica ate 2002-08-01 (a chuteira de ouro em 29/12/2001 e os premios do ano
# em 6/6/2002 sobem reputacoes).
# - a chuteira de ouro com a reputacao nacional de 500;
# - o goleiro do ano escolhido entre os defensores;
# - os premios do ano uma semana depois (13 de junho).
MUTACOES = [
    ('rx12_7_reputacao_menor',
     '5000, 2500, u32::MAX, 0, 0, 0, f, 1]), // rx12_7: the golden boot',
     '500, 2500, u32::MAX, 0, 0, 0, f, 1]), // mutation',
     'rx12_7 belgian awards built'),
    ('rx12_7_goleiro_defensor',
     '[-1, 1, 0x11, -1, -1, -1, -1]); // rx12_7: the goalkeeper',
     '[-1, 6, 0x11, -1, -1, -1, -1]); // mutation',
     'rx12_7 belgian year awards'),
    ('rx12_7_semana_depois',
     '    if awards_day_is(today, 6, 5) { // rx12_7: 6 June',
     '    if awards_day_is(today, 0xd, 5) { // mutation'),
]
