# Mutacoes (cx01_12), carreira de 8 ligas ate 2002-06-01. Nenhuma abre caixa.
# - os inscritos 5 e 6 trocados de grupo: os jogos dos grupos mudam e o comparador acusa (o plano contra o construido);
# - o terceiro do grupo A nas semifinais no lugar do segundo do B: o comparador acusa e os jogos mudam.
# A virada fica sem mutacao propria: o que ela monta so e usado na edicao seguinte (junho de 2002, depois da data da
# cadeia); o molde da virada (League::free_table, o arquivo das estatisticas, as etapas) ja e provado nas outras ligas.
MUTACOES = [
    ('cx01_12_grupos_trocados',
     '    [[row(0), entrant(3), entrant(4), entrant(5)], [row(1), entrant(6), entrant(7), entrant(8)]]',
     '    [[row(0), entrant(3), entrant(4), entrant(6)], [row(1), entrant(5), entrant(7), entrant(8)]] // mutation'),
    ('cx01_12_terceiro_nas_semis',
     '    [row(0, 0), row(1, 1), row(0, 1), row(1, 0)]',
     '    [row(0, 0), row(0, 2), row(0, 1), row(1, 0)] // mutation'),
]
