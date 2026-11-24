# Mutacoes (sbD_4). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (compare_text_writer:
# o texto, os vinculos e o gerador) com o modo CM_TEXTO_NOTICIAS na carreira automatica (CM_TEXTOS=1 na cadeia):
# - o premio: o jogador no lugar do tecnico;
# - a multa: o tipo invertido.
MUTACOES = [
    ('sbD_4_premio_tecnico', 'paper, m, award + 4);', 'paper, p, award + 4); /* mutation */', 'sbD_4 credits manager for award text body'),
    ('sbD_4_multa_tipo', '        if kind == 0 {\n            story_lang!(o, 0x9c6ee0,', '        if kind != 0 { // mutation\n            story_lang!(o, 0x9c6ee0,', 'sbD_4 fa disciplinary fine text body'),
]
