# Mutacoes (cqq_12), nas copas de clubes da Oceania (clubes australianos jogaveis: o Campeonato de Clubes de janeiro de
# 2001 fica dentro da janela da carreira de 8 ligas) e da Asia (clubes japoneses): a 1a rodada dos grupos da Oceania em
# 3/1 em vez de 10/1 (uma semana: as datas sao levadas ao dia da semana); os times da Liga dos Campeoes asiatica tirados dos classificados de tras para a frente; os da
# Recopa asiatica idem (os confrontos mudam). Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqq12_oceania_dia_3',
     'Cqq12D(0, 0xa, 0, 1, 4, 2, 1)',
     'Cqq12D(0, 3, 0, 1, 4, 2, 1)',
     'cqq_12 grupos da oceania'),
    ('cqq12_liga_asiatica_de_tras_para_frente',
     '|i| i) // cqq12: os times da Liga dos Campeões asiática',
     '|i| 15 - i) // mutacao: de tras para a frente',
     'cqq_12 times da liga dos campeoes asiatica'),
    ('cqq12_recopa_asiatica_de_tras_para_frente',
     '|i| i) // cqq12: os times da Recopa asiática',
     '|i| 15 - i) // mutacao: de tras para a frente',
     'cqq_12 times da recopa asiatica'),
]
