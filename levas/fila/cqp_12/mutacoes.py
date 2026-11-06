# Mutacoes (cqp_12), no calendario dos torneios de selecoes (a carreira de 8 ligas vai de janeiro de 2001 a junho de
# 2002: a Copa America de julho de 2001, com o Brasil, e a Copa Africana de fevereiro de 2002, com jogadores dos clubes
# europeus, ficam dentro da janela): a 1a rodada dos grupos da Copa America em 29/6 em vez de 6/7; a final da Copa
# America em 30/7 em vez de 23/7; a 1a rodada dos grupos da Copa Africana em 1/2 em vez de 8/2 (uma semana: as datas sao
# levadas ao dia da semana). Os contadores ficam
# colados no efeito (o ramo da fase).
MUTACOES = [
    ('cqp12_copa_america_dia_29_6',
     'Cqp12D(0, 6, 6, 0, 3, 1, 1)',
     'Cqp12D(0, 0x1d, 5, 0, 3, 1, 1)',
     'cqp_12 grupos da copa america'),
    ('cqp12_final_da_copa_america_dia_30',
     'Cqp12J(2, 0x17, 6, 0, 6, 1, 4)',
     'Cqp12J(2, 0x1e, 6, 0, 6, 1, 4)',
     'cqp_12 mata-mata da copa america'),
    ('cqp12_copa_africana_dia_1',
     'Cqp12D(0, 8, 1, 0, 6, 1, 1)',
     'Cqp12D(0, 1, 1, 0, 6, 1, 1)',
     'cqp_12 grupos da copa africana'),
]
