# Mutacoes (mwc_24): o segundo periodo da Segunda Divisao belga comeca na rodada 12 em vez da 10, e o da Eerste Divisie
# na 10 em vez da 8: as tabelas dos periodos, os seus vencedores e as vagas do mata-mata mudam (o comparador tambem pega).
MUTACOES = [
    ('mwc24_periodos_belgas',
     'add_result_by_period(this, fixture, arg, &[10, 22], 0x4204d0) // mwc24: the Belgian periods',
     'add_result_by_period(this, fixture, arg, &[12, 22], 0x4204d0) // mutation: the second period from round 12',
     'mwc_24 belgian second division result'),
    ('mwc24_periodos_holandeses',
     'add_result_by_period(this, fixture, arg, &[8, 16, 26], 0x5f6ad0) // mwc24: the Dutch periods',
     'add_result_by_period(this, fixture, arg, &[10, 16, 26], 0x5f6ad0) // mutation: the second period from round 10',
     'mwc_24 eerste divisie result'),
]
