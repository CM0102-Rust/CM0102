# Mutacoes (sx15_3). O texto nao volta para a simulacao: acusadas pela comparacao (compare_text_writer compara o texto
# e os vinculos), como as de texto aceitas em 12/11/2026.
# - o jogador sozinho numa vitoria, sem mascara que case: o modelo do empate;
# - a frase da lista de jogadores numa vitoria: a do empate.
MUTACOES = [
    ('sx15_3_sozinho_vitoria',
     'fallback: ((0xa5e72c, INTL_FORM_FULL), (0xa5e678, INTL_FORM_NO_FIXTURE_SCORE_LAST)), // sx15_3: a lone player in a win',
     'fallback: ((0xa5d6c8, INTL_FORM_FULL), (0xa5e678, INTL_FORM_NO_FIXTURE_SCORE_LAST)), // mutation: the draw template',
     'sx15_3 lone player'),
    ('sx15_3_lista_vitoria',
     'let template = [0xa5f968, 0xa5f878, 0xa5f788][result]; // sx15_3: the list sentence by the result',
     'let template = [0xa5f878, 0xa5f878, 0xa5f788][result]; // mutation: the draw sentence after a win',
     'sx15_3 player list'),
]
