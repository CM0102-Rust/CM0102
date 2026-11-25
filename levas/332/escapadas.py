# As escapadas da 332a leva (sx05: as ordens dos candidatos aos premios do ano). A confirmacao da cadeia no modo
# comparando vai ate 2001-08-20 (e a carga do save de ferias em portugues); a comparacao longa (ate 2002-08-01, a data
# da leva, bancada 4: levas/332/conf_escapadas_4.txt) acusou sx05_2_cinco_partidas; a outra ainda roda.

_PT = (' As ordens so pesam quando os premios do ano escolhem os vencedores, no fim da temporada; a carreira ate 2002-08-01 '
       'nao mudou de resultado. A comparacao longa decide.')
_EN = (' The orders only matter when the year awards pick their winners, at the season\'s end; the career up to 2002-08-01 '
       'did not change its result. The long comparison decides.')

ESCAPADAS = {
    'sx05_2_cinco_partidas': (
        'ACUSADA NA COMPARACAO LONGA. O minimo de partidas para concorrer passa de 3 a 5 em quem esta a frente na '
        'ordem: o comparador da ordem (year_ratings_compare_mode_2) a acusou antes de 2002-08-01 '
        '(levas/332/conf_escapadas_4.txt).',
        'CAUGHT BY THE LONG COMPARISON. The minimum matches to compete goes from 3 to 5 for the one ahead in the order: '
        'the order comparer (year_ratings_compare_mode_2) caught it before 2002-08-01 (levas/332/conf_escapadas_4.txt).'),
    'sx05_3_assistencias_valem_um': (
        'EM CONFIRMACAO. No modo 3, as assistencias valem 1 em vez de 3 na nota do candidato.' + _PT,
        'BEING CONFIRMED. In mode 3, the assists count 1 instead of 3 in the candidate\'s score.' + _EN),
}  # nome: (explicacao pt, explicacao en)
