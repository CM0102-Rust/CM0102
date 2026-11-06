# Mutacoes (cqp_10), no calendario de mais copas de clubes (a carreira de 8 ligas comeca em janeiro de 2001: a
# Merconorte de setembro a dezembro e as Supercopas Europeia e Asiatica de agosto ficam dentro da janela): a 1a rodada
# dos grupos da Merconorte em 2/9 em vez de 9/9 (uma semana antes: as datas sao levadas ao dia da semana); a Supercopa
# Europeia em 31/8 em vez de 24/8; a Supercopa Asiatica em 11/8 em vez de 4/8.
# A Copa dos Campeoes ficou passageira: as fases rodam (contadores > 0, 0 divergencias na comparacao), mas a preliminar
# uma semana antes (12/6) e a final uma semana depois (3/8) escaparam na carreira Rust ate 2002-06-01 (impressao igual a
# da base) [inferencia: os jogos da Copa dos Campeoes nao chegam a mudar nada que a carreira le].
MUTACOES = [
    ('cqp10_merconorte_dia_2',
     'const CQP10_MERCONORTE_GRUPOS: [Cqp10G; 6] = [(9, 8, 2, 2), (0x17, 8, 2, 2), (7, 9, 2, 2), (0x15, 9, 2, 2), (4, 0xa, 2, 2), (0x12, 0xa, 2, 2)]; // cqp10: os grupos da Merconorte',
     'const CQP10_MERCONORTE_GRUPOS: [Cqp10G; 6] = [(2, 8, 2, 2), (0x17, 8, 2, 2), (7, 9, 2, 2), (0x15, 9, 2, 2), (4, 0xa, 2, 2), (0x12, 0xa, 2, 2)]; // mutacao: 2/9',
     'cqp_10 grupos da merconorte'),
    ('cqp10_supercopa_europeia_semana_depois',
     'cql10r((4, 6, 0, 5), (0x18, 7, 0, 4, 2, 0x8000), 0, (0, 0x83, 0), 8, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqp10: a Supercopa Europeia',
     'cql10r((4, 6, 0, 5), (0x1f, 7, 0, 4, 2, 0x8000), 0, (0, 0x83, 0), 8, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mutacao: 31/8',
     'cqp_10 tabela da supercopa europeia'),
    ('cqp10_supercopa_asiatica_semana_depois',
     'cql10r((4, 6, 0, 5), (4, 7, 0, 5, 2, 0), 0, (0, 0, 0x83), 6, (2, 1, 2, 0), (0, 2, 0xe), (0, 0, 0)), // cqp10: a Supercopa Asiática',
     'cql10r((4, 6, 0, 5), (0xb, 7, 0, 5, 2, 0), 0, (0, 0, 0x83), 6, (2, 1, 2, 0), (0, 2, 0xe), (0, 0, 0)), // mutacao: 11/8',
     'cqp_10 tabela da supercopa asiatica'),
]
