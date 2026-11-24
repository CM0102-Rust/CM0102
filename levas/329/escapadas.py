# As escapadas da 329a leva. Na cadeia, a confirmacao comparando de varias delas acusou por engano a noticia 0x24
# (o escritor original 0x770d20 le um dword de pilha nunca iniciado; o comparador agora roda so a original para ela).
# Refeitas depois da correcao (levas/329/reconf.txt e reconf_pt.txt): a sx07_valor_sem_assinatura foi acusada. A
# comparacao longa (ate 2002-06-01, bancada 3: levas/329/conf_escapadas_3.txt) acusou sx07_codigo_365 e
# sx05_1_uma_decimal e mediu as outras tres; a sx07_nunca_em_casa foi acusada na dela (levas/329/conf_escapadas_3b.txt).

_MED_PT = (' Medido: comparacao ate 2002-06-01 (levas/329/conf_escapadas_3.txt) com 0 divergencias em 10^10 comparacoes '
           'do motor.')
_MED_EN = (' Measured: comparison to 2002-06-01 (levas/329/conf_escapadas_3.txt) with 0 divergences in 10^10 engine '
           'comparisons.')

ESCAPADAS = {
    'sx03_3_limites_40': (
        'LACUNA MEDIDA. O registro de 42 bytes de restricoes do elenco (as regras sem limites proprios, a Europa entre '
        'elas) e enchido de 0xff so ate 40: a ultima palavra fica com o que ja havia, e nenhuma partida da carreira le '
        'esse limite com um valor diferente.' + _MED_PT,
        'MEASURED GAP. The 42-byte squad restriction record (the rules without limits of their own, Europe among them) '
        'is filled with 0xff only up to 40: the last word keeps what was there, and no match of the career reads that '
        'limit with a different value.' + _MED_EN),
    'sx03_4_sem_jogador_mundial': (
        'LACUNA MEDIDA. Sem o jogador do ano mundial (6 de maio): o premio so grava o vencedor no historico dos premios '
        'e na reputacao, que nada da carreira le ate 2002-06-01.' + _MED_PT,
        'MEASURED GAP. No world player of the year (6 May): the award only records the winner in the award history and '
        'reputation, which nothing in the career reads up to 2002-06-01.' + _MED_EN),
    'sx07_recusa_sem_motivos': (
        'LACUNA MEDIDA. A recusa sem os motivos: o texto so e escrito quando a noticia chega a um tecnico humano, e '
        'nenhuma recusa com motivos chegou ao clube humano nessa carreira.' + _MED_PT,
        'MEASURED GAP. The refusal without the reasons: the text is only written when the item reaches a human manager, '
        'and no refusal with reasons reached the human club in this career.' + _MED_EN),
}  # nome: (explicacao pt, explicacao en)
