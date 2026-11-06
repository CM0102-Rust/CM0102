# Mutacoes (cqp_10), no calendario de mais copas de clubes (a carreira de 8 ligas comeca em janeiro de 2001: a Copa dos
# Campeoes de junho e julho de 2001, a Merconorte de setembro a dezembro e a Supercopa Europeia de agosto ficam dentro
# da janela): a 1a rodada da preliminar da Copa dos Campeoes em 12/6 em vez de 19/6; a 1a rodada dos grupos da
# Merconorte em 2/9 em vez de 9/9 (uma semana antes: as datas sao levadas ao dia da semana); a Supercopa Europeia em 31/8 em vez de 24/8. Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqp10_preliminar_dos_campeoes_dia_12',
     'const CQP10_CAMPEOES_PRELIMINAR: [Cqp10G; 3] = [(0x13, 5, 1, 2), (0x1a, 5, 1, 2), (3, 6, 1, 2)]; // cqp10: a preliminar da Copa dos Campeões',
     'const CQP10_CAMPEOES_PRELIMINAR: [Cqp10G; 3] = [(0xc, 5, 1, 2), (0x1a, 5, 1, 2), (3, 6, 1, 2)]; // mutacao: 12/6',
     'cqp_10 preliminar da copa dos campeoes'),
    ('cqp10_merconorte_dia_2',
     'const CQP10_MERCONORTE_GRUPOS: [Cqp10G; 6] = [(9, 8, 2, 2), (0x17, 8, 2, 2), (7, 9, 2, 2), (0x15, 9, 2, 2), (4, 0xa, 2, 2), (0x12, 0xa, 2, 2)]; // cqp10: os grupos da Merconorte',
     'const CQP10_MERCONORTE_GRUPOS: [Cqp10G; 6] = [(2, 8, 2, 2), (0x17, 8, 2, 2), (7, 9, 2, 2), (0x15, 9, 2, 2), (4, 0xa, 2, 2), (0x12, 0xa, 2, 2)]; // mutacao: 2/9',
     'cqp_10 grupos da merconorte'),
    ('cqp10_supercopa_europeia_semana_depois',
     'cql10r((4, 6, 0, 5), (0x18, 7, 0, 4, 2, 0x8000), 0, (0, 0x83, 0), 8, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqp10: a Supercopa Europeia',
     'cql10r((4, 6, 0, 5), (0x1f, 7, 0, 4, 2, 0x8000), 0, (0, 0x83, 0), 8, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mutacao: 31/8',
     'cqp_10 tabela da supercopa europeia'),
]
