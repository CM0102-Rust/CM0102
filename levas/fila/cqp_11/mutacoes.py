# Mutacoes (cqp_11), no calendario das copas de clubes da Asia (clubes japoneses jogaveis) e do LDV Vans Trophy (a
# Inglaterra jogavel); a carreira de 8 ligas vai de janeiro de 2001 a junho de 2002: os grupos finais asiaticos de
# marco, as quartas da Recopa de marco e as chaves do LDV de dezembro a marco ficam dentro da janela. A 1a rodada dos
# grupos finais da Liga dos Campeoes asiatica em 14/3 em vez de 21/3; as quartas da Recopa asiatica em 17/3 em vez de
# 24/3; a 1a rodada das chaves do LDV em 3/12 em vez de 10/12 (uma semana antes: as datas sao levadas ao dia da semana). Os contadores ficam colados no efeito (o ramo da fase).
MUTACOES = [
    ('cqp11_grupos_asiaticos_dia_14',
     'Cqp11D(0, 0x15, 2, 1, 2, 1, 1)',
     'Cqp11D(0, 0xe, 2, 1, 2, 1, 1)',
     'cqp_11 grupos da liga dos campeoes asiatica'),
    ('cqp11_quartas_da_recopa_dia_17',
     'Cqp11J(2, 0x18, 2, 1, 2, 2, 0)',
     'Cqp11J(2, 0x11, 2, 1, 2, 2, 0)',
     'cqp_11 eliminatorias da recopa asiatica'),
    ('cqp11_trofeu_dia_3',
     'Cqp11J(0, 0xa, 0xb, 0, 1, 2, 0)',
     'Cqp11J(0, 3, 0xb, 0, 1, 2, 0)',
     'cqp_11 chaves do trofeu'),
]
