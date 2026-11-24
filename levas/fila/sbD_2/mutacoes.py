# Mutacoes (sbD_2). O texto nao volta para a simulacao: acusadas pela comparacao do escritor (compare_text_writer:
# o texto, os vinculos e o gerador) com o modo CM_TEXTO_NOTICIAS na carreira automatica (CM_TEXTOS=1 na cadeia):
# - o treino: os modelos trocados;
# - as pazes: o vinculo do outro com o tipo 0x7d7;
# - o relatorio: o elogio 0 com os nomes trocados.
MUTACOES = [
    ('sbD_2_treino_redacoes', '|wording| if wording != 0 { 0x9c5ccc } else { 0x9c5c5c }', '|wording| if wording != 0 { 0x9c5c5c } else { 0x9c5ccc } /* mutation */', 'sbD_2 unhappy with training text'),
    ('sbD_2_pazes_vinculo', '    other.link(text, &[b], 0x7da);\n}', '    other.link(text, &[b], 0x7d7); // mutation\n}', 'sbD_2 feud settled text'),
    ('sbD_2_relatorio_elogio', '                0 => story_lang!(out, 0x9c5ea8, 0x64, c, p),', '                0 => story_lang!(out, 0x9c5ea8, 0x64, p, c), // mutation', 'sbD_2 training report text body'),
]
