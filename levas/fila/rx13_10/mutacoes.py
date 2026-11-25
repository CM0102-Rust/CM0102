# Mutacoes (rx13_10): o texto nao volta a simulacao; acusa o comparador do texto (compare_text_writer) com o modo de
# textos das noticias (CM_TEXTO_NOTICIAS, --textos) e um tecnico humano como destinatario, na repescagem da Terceira
# Divisao (maio de 2002):
# - a manchete do acesso com o vinculo da competicao.
MUTACOES = [
    ('rx13_10_acesso_com_vinculo',
     'ENG_THIRD_PROMOTION_TEXT, false, false);',
     'ENG_THIRD_PROMOTION_TEXT, false, true); // mutation',
     'rx13_10 third division promotion headline'),
]
