# Mutacoes (cx02_23), carreira com a Coreia ate 2002-08-01 (com o modo de textos). Nenhuma abre caixa.
# - as posicoes previstas em faixas de 3 e nao 2: as expectativas mudam e o comparador acusa;
# - o 3o lugar de um grupo vira 4 e nao 5: o comparador acusa a cada posicao gravada;
# - a linha de quem passa do grupo com "through to second round": o comparador de texto acusa.
MUTACOES = [
    ('cx02_23_faixas_de_3',
     "    italian_cup_predicted_positions(this, 0x9eb15c, (0x38d, 0x395), 1, 2, \"cx02_23 korean league cup predicted\")",
     "    italian_cup_predicted_positions(this, 0x9eb15c, (0x38d, 0x395), 1, 3, \"cx02_23 korean league cup predicted\") // mutation"),
    ('cx02_23_terceiro_4',
     'GroupCupProgress { knockout: 1, current: &[1, 1, 5, 7, 9],',
     'GroupCupProgress { knockout: 1, current: &[1, 1, 4, 7, 9], // mutation\n   '),
    ('cx02_23_modelo_segunda_fase',
     '    group_cup_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 1, 0x9a7dac)',
     '    group_cup_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 1, 0x9a7e24) // mutation'),
]
