# As escapadas da 340a leva (competicoes de ENG ESP SWE ate 2002-09-01, base NORMAL). A confirmacao da cadeia comparou
# essas ligas e passou pela carga do save de ferias em portugues; como leva de competicoes, a comparacao longa de
# varias ligas e a da auditoria das 8 ligas (acumulada, de madrugada).

_PT = (' A confirmacao comparando ate 2002-09-01 (10^10 comparacoes do motor) e a carga em portugues nao viram diferenca. '
       'A auditoria das 8 ligas decide.')
_EN = (' The confirmation comparing to 2002-09-01 (10^10 engine comparisons) and the Portuguese load saw no difference. The '
       '8-league audit decides.')

ESCAPADAS = {
    'rx09_6_repescagem_0x3c': (
        'EM CONFIRMACAO. O codigo de progresso de quem vence a repescagem de rebaixamento da Segunda B espanhola passa de '
        '0x3b a 0x3c: so muda a posicao gravada nas estatisticas do clube.' + _PT,
        'BEING CONFIRMED. The progress code of the Spanish Segunda B relegation play-off winner goes from 0x3b to 0x3c: it '
        'only changes the position stored in the club\'s statistics.' + _EN),
    'rx09_8_time_b_sem_marca': (
        'EM CONFIRMACAO. O time B da Segunda Divisao sueca (secao Norte) deixa de ser marcado como impedido de subir: so '
        'pesa quando um time B termina numa posicao de acesso.' + _PT,
        'BEING CONFIRMED. The B team of the Swedish Second Division (North section) is no longer marked as barred from '
        'going up: it only matters when a B team finishes in a promotion place.' + _EN),
    'rx09_11_repescagem_0x3e': (
        'EM CONFIRMACAO. O codigo de progresso da repescagem de rebaixamento da Segunda sueca passa de 0x3d a 0x3e: so '
        'muda a posicao gravada nas estatisticas do clube.' + _PT,
        'BEING CONFIRMED. The progress code of the Swedish Second Division relegation play-off goes from 0x3d to 0x3e: it '
        'only changes the position stored in the club\'s statistics.' + _EN),
    'rx13_16_time_b_sobe': (
        'EM CONFIRMACAO. Na subida da Segunda B espanhola, o time B cujo time A esta na Segunda B deixa de ser '
        'mantido embaixo: so pesa na virada em que um time B fica entre os que sobem com o seu time A na mesma divisao.' + _PT,
        'BEING CONFIRMED. In the Spanish Segunda B promotion, a B team whose A team is in the Segunda B is no longer kept '
        'down: it only matters at a turnover where a B team is among those going up with its A team in the same division.' + _EN),
}  # nome: (explicacao pt, explicacao en)
