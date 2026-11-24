# Mutacoes (cx02_21), carreira com a Holanda ate 2002-08-01. Nenhuma abre caixa.
# - uma vaga por grupo e nao duas: quem passa dos grupos muda (a carreira e a conferencia das regras);
# - as posicoes previstas em faixas de 19 e nao 20: as expectativas mudam e o comparador acusa.
# Os registros de carreira so passam ao molde provado (ItaCup) a fase e a rodada: sem mutacao propria.
MUTACOES = [
    ('cx02_21_uma_vaga_por_grupo',
     'tiebreak: [4, 1, 2, 0], slots: 2, prize: Some(0xc3500), bits: None };',
     'tiebreak: [4, 1, 2, 0], slots: 1, prize: Some(0xc3500), bits: None }; // mutation'),
    ('cx02_21_faixas_de_19',
     "    italian_cup_predicted_positions(this, 0x9cedd4, (0x4be, 0x4c6), 0x13, 0x14, \"cx02_21 dutch cup predicted\")",
     "    italian_cup_predicted_positions(this, 0x9cedd4, (0x4be, 0x4c6), 0x13, 0x13, \"cx02_21 dutch cup predicted\") // mutation"),
]
