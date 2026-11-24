# Mutacoes (cx02_18), carreira com a Grecia ate 2002-08-01. Nenhuma abre caixa.
# - uma vaga por grupo e nao duas: quem passa dos grupos muda (a carreira e a conferencia das regras);
# - as posicoes previstas em faixas de 10 e nao 11: as expectativas mudam e o comparador acusa.
# A virada so passa ao molde provado (NirLgeCup) os enderecos, os grupos e as linhas: sem mutacao propria (mudar o ano
# ou os grupos deixaria a copa sem jogos ou sem uma fase).
MUTACOES = [
    ('cx02_18_uma_vaga_por_grupo',
     'tiebreak: [4, 1, 2, 0], slots: 2, prize: Some(0x927c0), bits: None };',
     'tiebreak: [4, 1, 2, 0], slots: 1, prize: Some(0x927c0), bits: None }; // mutation'),
    ('cx02_18_faixas_de_10',
     "    italian_cup_predicted_positions(this, 0x9cdab0, (0x3f0, 0x3f8), 0xa, 0xb, \"cx02_18 greek cup predicted\")",
     "    italian_cup_predicted_positions(this, 0x9cdab0, (0x3f0, 0x3f8), 0xa, 0xa, \"cx02_18 greek cup predicted\") // mutation"),
]
