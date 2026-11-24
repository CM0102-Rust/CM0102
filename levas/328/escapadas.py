# As escapadas da 328a leva (tel44: o texto da noticia de recorde e a lista de candidatos a premio).
# A confirmacao da cadeia no modo comparando vai so ate 2001-08-20; a comparacao longa (ate 2002-06-01, bancada 4:
# levas/328/conf_escapadas_4.txt) acusou tel44_2_name_order e tel44_3_votes_cleared e mediu as outras duas.

_PT = (' Medido: comparacao ate 2002-06-01 (levas/328/conf_escapadas_4.txt) com 0 divergencias em 10^10 comparacoes do '
       'motor; a ordem so pesa com mais de um candidato na escolha dos vencedores, e nessa carreira ela nao mudou nenhum '
       'resultado. Fica com o comparador da escolha (award_shortlist_pick_winners), que compara os vencedores e o retorno.')
_EN = (' Measured: comparison to 2002-06-01 (levas/328/conf_escapadas_4.txt) with 0 divergences in 10^10 engine '
       'comparisons; the order only matters with more than one candidate when picking the winners, and in this career it '
       'changed no result. It stays with the picking comparer (award_shortlist_pick_winners), which compares the winners '
       'and the return.')

ESCAPADAS = {
    'tel44_2_score_order': (
        'LACUNA MEDIDA. A ordem por nota da lista de candidatos fica invertida.' + _PT,
        'MEASURED GAP. The shortlist\'s order by score is inverted.' + _EN),
    'tel44_2_votes_order': (
        'LACUNA MEDIDA. A ordem por votos da lista de candidatos fica invertida.' + _PT,
        'MEASURED GAP. The shortlist\'s order by votes is inverted.' + _EN),
}  # nome: (explicacao pt, explicacao en)
