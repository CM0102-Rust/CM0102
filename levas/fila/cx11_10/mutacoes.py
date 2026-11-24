# Mutacoes (cx11_10), com as 8 ligas ate 2002-06-01 e o modo CM_TEXTO_NOTICIAS:
# - o vencedor da repescagem recebe o texto de rebaixado. O texto nao volta a simulacao: a carreira nao acusa; acusa o
#   comparador do texto (compare_text_writer), se a noticia do destino de um clube da repescagem da C1 for escrita
#   (precisa do modo CM_TEXTO_NOTICIAS e do tecnico humano como destinatario).
MUTACOES = [
    ('cx11_10_vencedor_rebaixado',
     '(1, 0) => group_qualify_text(club_id, WIN_PROMOTION_TEXT, text), // cx11_10: the play-off winner goes up',
     '(1, 0) => group_qualify_text(club_id, RELEGATED_TEXT, text), // mutation: the relegated text',
     'cx11_10 serie c own fate text'),
]
