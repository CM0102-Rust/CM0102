# As escapadas da 332a leva (sx05: as ordens dos candidatos aos premios do ano). A confirmacao da cadeia no modo
# comparando vai ate 2001-08-20 (e a carga do save de ferias em portugues); a comparacao longa (ate 2002-08-01, a data
# da leva, bancada 4) acusou as duas.

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
        'ACUSADA NA COMPARACAO LONGA. No modo 3, as assistencias valem 1 em vez de 3 na nota do candidato: o comparador '
        'da ordem (year_ratings_compare_mode_3) a acusou antes de 2002-08-01 (levas/332/conf_escapadas_4b.txt; a primeira '
        'rodada, conf_escapadas_4.txt, parou num falso alarme do elogio corrigido na propria 332a).',
        'CAUGHT BY THE LONG COMPARISON. In mode 3, the assists count 1 instead of 3 in the candidate's score: the order '
        'comparer (year_ratings_compare_mode_3) caught it before 2002-08-01 (levas/332/conf_escapadas_4b.txt; the first run, '
        'conf_escapadas_4.txt, stopped at a false alarm of the praise fixed in batch 332 itself).'),
}  # nome: (explicacao pt, explicacao en)
