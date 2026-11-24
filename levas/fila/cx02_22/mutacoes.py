# Mutacoes (cx02_22), carreira com a Holanda ate 2002-08-01 (com o modo de textos). Nenhuma abre caixa.
# - o 4o lugar de um grupo vira 76 e nao 77: o comparador acusa a cada posicao gravada;
# - a Copa da Holanda com o mata-mata na fase 18 no texto: o texto de quem passa do ultimo grupo muda (o comparador de
#   texto).
MUTACOES = [
    ('cx02_22_quarto_76',
     'GroupCupProgress { knockout: 0x13, current: &[17, 17, 47, 77],',
     'GroupCupProgress { knockout: 0x13, current: &[17, 17, 47, 76], // mutation\n   '),
    ('cx02_22_texto_fase_18',
     '    group_cup_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 0x13, 0x9a7e24)',
     '    group_cup_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 0x12, 0x9a7e24) // mutation'),
]
