# As escapadas da 336a leva. A confirmacao da cadeia no modo comparando vai ate 2001-08-20 (e a carga do save de
# ferias em portugues); a comparacao longa (ate 2002-06-01, bancada 4: levas/336/conf_escapadas_4.txt) decide, e o
# resultado entra num commit seguinte.

_PT = ' A comparacao longa decide.'
_EN = ' The long comparison decides.'

ESCAPADAS = {
    'sx12_mais_antiga_primeiro': (
        'EM CONFIRMACAO. A ordem do historico de jogos do humano poe a entrada mais nova primeiro: so a tela le essa ordem, '
        'e a comparacao so a ve quando dois registros sao ordenados.' + _PT,
        'BEING CONFIRMED. The human\'s match history order puts the newer entry first: only the screen reads that order, '
        'and the comparison only sees it when two records are sorted.' + _EN),
    'sx09b_inicio_hoje': (
        'EM CONFIRMACAO. O registro do humano conta como ainda nao iniciado tambem no proprio dia do inicio: so muda a '
        'conta no dia exato em que um tecnico humano assume.' + _PT,
        'BEING CONFIRMED. The human record counts as not started also on the start day itself: it only changes the count '
        'on the exact day a human manager takes over.' + _EN),
    'sx05b_5_quatro_reservas': (
        'EM CONFIRMACAO. A equipe da semana nasce com 4 reservas em vez de 3: so pesa quando a equipe e escolhida e '
        'mostrada.' + _PT,
        'BEING CONFIRMED. The team of the week is built with 4 substitutes instead of 3: it only matters when the team is '
        'picked and shown.' + _EN),
    'sx05b_5_sem_tipo': (
        'EM CONFIRMACAO. O premio do ano nasce sem o tipo de registro que os registros de pessoal guardam: so pesa quando '
        'o premio e entregue, no fim da temporada.' + _PT,
        'BEING CONFIRMED. The year award is built without the record kind the staff records keep: it only matters when the '
        'award is given, at the season\'s end.' + _EN),
    'sx05b_21_libra_ponto': (
        'EM CONFIRMACAO. A libra passa a separar os milhares com ponto: so muda o dinheiro mostrado em libras, e o jogo de '
        'teste e em portugues (moeda local).' + _PT,
        'BEING CONFIRMED. The pound separates thousands with a dot: it only changes money shown in pounds, and the test '
        'game is in Portuguese (local currency).' + _EN),
    'sx05b_21_sem_sufixo': (
        'EM CONFIRMACAO. A troca de moeda deixa de copiar o sufixo: so muda o dinheiro mostrado numa moeda com sufixo.' + _PT,
        'BEING CONFIRMED. The currency change stops copying the suffix: it only changes money shown in a currency with a '
        'suffix.' + _EN),
}  # nome: (explicacao pt, explicacao en)
