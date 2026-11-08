# Mutacoes (cqm_12): na Supercopa da Espanha, o vice da Copa do Rei no lugar do campeao (outro clube joga em agosto de
# 2001); na Supercopa do Japao, o vice da J1 no lugar do campeao (o jogo e em marco de 2002); o Jogo das Estrelas da MLS
# uma semana depois (2/8 -> 9/8 de 2001, dentro da janela: a carreira comeca em 2001-205). Os contadores ficam colados
# no efeito.
MUTACOES = [
    ('cqm12_supercopa_da_espanha_com_o_vice',
     'let a = cqm12_colocado(0x9cf740, 0); // cqm12: o campeão da Copa do Rei',
     'let a = cqm12_colocado(0x9cf740, 1); // mutacao: o vice',
     'cqm_12 spanish super cup clubs'),
    ('cqm12_supercopa_do_japao_com_o_vice',
     'let a = cqm12_colocado(0x9cf65c, 0); // cqm12: o campeão da J1',
     'let a = cqm12_colocado(0x9cf65c, 1); // mutacao: o vice',
     'cqm_12 japanese super cup clubs'),
    ('cqm12_jogo_das_estrelas_uma_semana_depois',
     'jogo(t, 0, 2, 7, 0, 6, 1, ano, 0); // cqm12: o Jogo das Estrelas em 2 de agosto',
     'jogo(t, 0, 9, 7, 0, 6, 1, ano, 0); // mutacao: uma semana depois',
     'cqm_12 all-star game fixture'),
]
