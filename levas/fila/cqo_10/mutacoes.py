# Mutacoes (cqo_10), no calendario das copas italianas (a Italia jogavel na carreira de 8 ligas): o primeiro jogo do
# mata-mata da Copa da Italia uma semana depois (26/9 em vez de 19/9); a primeira rodada dos grupos da Copa da Serie C
# em 5/8 em vez de 12/8 (uma semana antes: as datas sao levadas ao dia da semana); a Supercopa da Italia em 26/8 em vez de 19/8. As tres tabelas sao montadas no construtor
# (comeco do jogo) e nas fases de agosto e setembro de 2001, dentro da janela. Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqo10_mata_mata_da_italia_semana_depois',
     'cql10r((0x1e, 7, 0, 3), (0x13, 8, 0, 2, 2, 0), 0xa, (0, 0x400, 3), 4, (0x10, 8, 0x10, 0), (0, 2, 0x23), (0, 0xffff_ffff, 0)), // cqo10: a 1ª rodada do mata-mata da Copa da Itália',
     'cql10r((0x1e, 7, 0, 3), (0x1a, 8, 0, 2, 2, 0), 0xa, (0, 0x400, 3), 4, (0x10, 8, 0x10, 0), (0, 2, 0x23), (0, 0xffff_ffff, 0)), // mutacao: 26/9',
     'cqo_10 mata-mata da copa da italia'),
    ('cqo10_grupos_da_serie_c_dia_5',
     'const CQO10_COPA_C_GRUPOS: [Cqo10G; 5] = [(0xc, 7, 6, 1), (0x13, 7, 6, 1), (0x16, 7, 2, 2), (0x1a, 7, 6, 1), (0x1d, 7, 2, 2)]; // cqo10: os grupos da Copa da Série C',
     'const CQO10_COPA_C_GRUPOS: [Cqo10G; 5] = [(5, 7, 6, 1), (0x13, 7, 6, 1), (0x16, 7, 2, 2), (0x1a, 7, 6, 1), (0x1d, 7, 2, 2)]; // mutacao: 5/8',
     'cqo_10 grupos da copa da serie c'),
    ('cqo10_supercopa_uma_semana_depois',
     'cql10r((5, 6, 0, 5), (0x13, 7, 0, 6, 1, 0), 0, (0, 0x203, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqo10: a Supercopa da Itália',
     'cql10r((5, 6, 0, 5), (0x1a, 7, 0, 6, 1, 0), 0, (0, 0x203, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mutacao: 26/8',
     'cqo_10 tabela da supercopa'),
]
