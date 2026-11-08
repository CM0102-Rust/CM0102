# Mutacoes (cqq_11), nos clubes das supercopas (a Supercopa Europeia de 24/8/2001, com o campeao da Copa da UEFA,
# e a Asiatica de agosto ficam dentro da janela da carreira de 8 ligas, de janeiro de 2001 a junho de 2002): o vice da
# Liga dos Campeoes no lugar do campeao; o vice da Copa da UEFA no lugar do campeao; o vice da Recopa asiatica no lugar
# do campeao. Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqq11_vice_da_liga_dos_campeoes',
     'let a = cqm12_colocado(0x9cf6e8, 0); // cqq11: o campeão da Liga dos Campeões',
     'let a = cqm12_colocado(0x9cf6e8, 1); // mutacao: o vice',
     'cqq_11 european super cup clubs'),
    ('cqq11_vice_da_copa_da_uefa',
     'let b = cqm12_colocado(copa, 0); // cqq11: o campeão da Copa da UEFA',
     'let b = cqm12_colocado(copa, 1); // mutacao: o vice',
     'cqq_11 european super cup clubs'),
    ('cqq11_vice_da_recopa_asiatica',
     'let b = cqm12_colocado(0x9cf650, 0); // cqq11: o campeão da Recopa asiática',
     'let b = cqm12_colocado(0x9cf650, 1); // mutacao: o vice',
     'cqq_11 asian super cup clubs'),
]
