# Mutacoes (cx02_12), carreira de 8 ligas ate 2002-06-01 (com o modo de textos). Nenhuma abre caixa.
# - o 2o lugar de um grupo da Copa da Italia vira 24 e nao 25: o comparador acusa nos grupos (agosto e setembro);
# - o pior lugar 4 da Copa da Serie C vira 75 e nao 71: o comparador acusa;
# - a Copa da Serie C com o mata-mata na fase 14: o texto de quem passa do ultimo grupo muda (o comparador de texto).
MUTACOES = [
    ('cx02_12_coppa_segundo_24',
     'GroupCupProgress { knockout: 7, current: &[1, 25, 33, 41],',
     'GroupCupProgress { knockout: 7, current: &[1, 24, 33, 41], // mutation\n   '),
    ('cx02_12_serie_c_pior_4',
     'best: &[1, 33, 59, 75], worst: &[17, 33, 59, 71] };',
     'best: &[1, 33, 59, 75], worst: &[17, 33, 59, 75] }; // mutation'),
    ('cx02_12_serie_c_mata_mata_14',
     '    group_cup_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 15, 0x9a7e24)',
     '    group_cup_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a8, news, text, 14, 0x9a7e24) // mutation'),
]
