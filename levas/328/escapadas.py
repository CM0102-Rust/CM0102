# As escapadas da 328a leva (tel44: o texto da noticia de recorde e a lista de candidatos a premio).
# A confirmacao da cadeia no modo comparando vai so ate 2001-08-20; a comparacao longa (ate 2002-06-01, bancada 4:
# levas/328/conf_escapadas_4.txt) decide, e o resultado entra num commit seguinte, como na 320a e na 324a.

_PT = (' As ordens e os votos da lista de candidatos so agem no anuncio (por nome) e na escolha dos vencedores (por nota e '
       'por votos), perto do fim da temporada; ate 2001-08-20 nao ha lista com mais de um candidato. A comparacao longa decide.')
_EN = (' The orders and the votes of the shortlist only act at the announcement (by name) and at picking the winners (by '
       'score and by votes), near the season\'s end; up to 2001-08-20 there is no list with more than one candidate. The '
       'long comparison decides.')

ESCAPADAS = {
    'tel44_2_name_order': (
        'EM CONFIRMACAO. A ordem por nome da lista de candidatos fica invertida.' + _PT,
        'BEING CONFIRMED. The shortlist\'s order by name is inverted.' + _EN),
    'tel44_2_score_order': (
        'EM CONFIRMACAO. A ordem por nota da lista de candidatos fica invertida.' + _PT,
        'BEING CONFIRMED. The shortlist\'s order by score is inverted.' + _EN),
    'tel44_2_votes_order': (
        'EM CONFIRMACAO. A ordem por votos da lista de candidatos fica invertida.' + _PT,
        'BEING CONFIRMED. The shortlist\'s order by votes is inverted.' + _EN),
    'tel44_3_votes_cleared': (
        'EM CONFIRMACAO. A limpeza da lista deixa os votos em 0 em vez de -1.' + _PT,
        'BEING CONFIRMED. Clearing the list leaves the votes at 0 instead of -1.' + _EN),
}  # nome: (explicacao pt, explicacao en)
