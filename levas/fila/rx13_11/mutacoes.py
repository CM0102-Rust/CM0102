# Mutacoes (rx13_11): o texto nao volta a simulacao; acusa o comparador do texto (compare_text_writer) com o modo de
# textos das noticias (CM_TEXTO_NOTICIAS, --textos) e um tecnico humano como destinatario (o grupo da Finlandia, as
# repescagens de outubro de 2001):
# - o clube da Premier que fica na repescagem de acesso recebe "win promotion".
MUTACOES = [
    ('rx13_11_premier_como_acesso',
     '(3, 1) if a7 == read_u32(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) => Some((FIN_AVOID_RELEGATION_TEXT, false)), // rx13_11: the Premier club stays up',
     '(3, 1) if a7 == read_u32(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) => Some((FIN_PROMOTION_TEXT, false)), // mutation',
     'rx13_11 finnish first headline'),
]
