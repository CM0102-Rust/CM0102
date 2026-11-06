# Mutacoes (cqp_13), no calendario das copas intercontinentais e da Intertoto (a carreira de 8 ligas vai de janeiro de
# 2001 a junho de 2002: a Intertoto de junho a agosto de 2001, com clubes de ENG, ESP, ITA e FRA, a Copa dos Campeoes
# da CONCACAF de agosto e setembro, com clubes da MLS, e a Copa Intercontinental de dezembro ficam dentro da janela): o
# 1o jogo da Intertoto em 19/6 em vez de 26/6 (sorteio em 18/6); a Intercontinental em 8/12 em vez de 1/12; as quartas da CONCACAF em
# 21/8 em vez de 28/8 (uma semana: as datas sao levadas ao dia da semana). Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqp13_intertoto_dia_19',
     'cql10r((0x19, 5, 0, 4), (0x1a, 5, 0, 5, 1, 0), 0xa, (0, 4, 7), 7, (0x28, 0x14, 0x28, 0), (0, 2, 4), (0, 0, 0)), // cqp13: a 1ª rodada da Intertoto',
     'cql10r((0x12, 5, 0, 4), (0x13, 5, 0, 5, 1, 0), 0xa, (0, 4, 7), 7, (0x28, 0x14, 0x28, 0), (0, 2, 4), (0, 0, 0)), // mutacao: 19/6',
     'cqp_13 tabela da intertoto'),
    ('cqp13_intercontinental_dia_8',
     'cql10r((0xa, 9, 0, 5), (1, 0xb, 0, 1, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqp13: a Copa Intercontinental',
     'cql10r((0xa, 9, 0, 5), (8, 0xb, 0, 1, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mutacao: 8/12',
     'cqp_13 tabela da intercontinental'),
    ('cqp13_concacaf_dia_21',
     'cql10r((1, 7, 0, 5), (0x1c, 8, 0, 3, 2, 0), 0xa, (0, 3, 0), 8, (8, 4, 8, 0), (0, 0, 0), (0, 0xffff_ffff, 0)), // cqp13: as quartas da CONCACAF',
     'cql10r((1, 7, 0, 5), (0x15, 8, 0, 3, 2, 0), 0xa, (0, 3, 0), 8, (8, 4, 8, 0), (0, 0, 0), (0, 0xffff_ffff, 0)), // mutacao: 21/8',
     'cqp_13 tabela da concacaf'),
]
