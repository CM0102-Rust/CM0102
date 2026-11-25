# Mutacoes (rx12_9), pela carreira com a Dinamarca ate 2002-08-01 (o jogador do ano em 20/12/2001 e os premios das
# divisoes em 6/6/2002 sobem reputacoes).
# - o jogador do ano com a reputacao nacional de 600;
# - os jovens do ano sem o limite de idade;
# - os premios das divisoes uma semana depois (13 de junho).
MUTACOES = [
    ('rx12_9_reputacao_menor',
     '6000, 2500, u32::MAX, 0, 0, 0, f, 1]), // rx12_9: the player of the year',
     '600, 2500, u32::MAX, 0, 0, 0, f, 1]), // mutation',
     'rx12_9 danish awards built'),
    ('rx12_9_jovem_sem_idade',
     '[0x15, 0, 0x11, -1, -1, -1, -1]); // rx12_9: the young player, up to 21',
     '[-1, 0, 0x11, -1, -1, -1, -1]); // mutation',
     'rx12_9 danish division awards'),
    ('rx12_9_semana_depois',
     '    if awards_day_is(today, 6, 5) { // rx12_9: 6 June',
     '    if awards_day_is(today, 0xd, 5) { // mutation'),
]
