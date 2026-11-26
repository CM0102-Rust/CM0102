# As escapadas da 337a leva. A confirmacao da cadeia no modo comparando vai ate 2001-08-20 (e a carga do save de
# ferias em portugues); a comparacao longa (ate 2002-06-01, bancada 4: levas/337/conf_escapadas_4.txt) decide, e o
# resultado entra num commit seguinte.

_PT = ' A comparacao longa decide.'
_EN = ' The long comparison decides.'

ESCAPADAS = {
    'gra2c_treino_ate_40': (
        'EM CONFIRMACAO. O fator de treino do dia do jogador passa a ir ate 40 em vez de 25: so muda o jogador que treina '
        'acima do teto, o que quase nunca acontece.' + _PT,
        'BEING CONFIRMED. The player\'s daily training factor goes up to 40 instead of 25: it only changes a player who '
        'trains above the cap, which almost never happens.' + _EN),
    'sx14_2_furia_do_adversario': (
        'EM CONFIRMACAO. A manchete da furia da diretoria cita o adversario em vez do clube: so muda o texto dessa noticia, '
        'que so sai depois de uma derrota que a diretoria nao aceita.' + _PT,
        'BEING CONFIRMED. The board fury headline names the opponent instead of the club: it only changes the text of that '
        'news item, which only comes after a defeat the board does not accept.' + _EN),
    'sx04b_1_sorteio_de_dois': (
        'EM CONFIRMACAO. O sorteio do modelo da noticia do premio fica entre 2 em vez de 3: so muda o texto da noticia, '
        'quando o premio e entregue.' + _PT,
        'BEING CONFIRMED. The award news template draw picks among 2 instead of 3: it only changes the news text, when the '
        'award is given.' + _EN),
    'sx04b_1_manchete_por_cinco': (
        'EM CONFIRMACAO. A manchete da noticia do premio e escolhida por resto de 5 em vez de 6: so muda o texto da '
        'manchete, quando o premio e entregue.' + _PT,
        'BEING CONFIRMED. The award news headline is picked by remainder 5 instead of 6: it only changes the headline text, '
        'when the award is given.' + _EN),
    'sx04b_1_modelos_trocados': (
        'EM CONFIRMACAO. O modelo que poe o clube primeiro sai no sorteio 2 em vez do 1: so muda o texto da noticia do '
        'premio.' + _PT,
        'BEING CONFIRMED. The club-first template comes on draw 2 instead of 1: it only changes the award news text.' + _EN),
    'sx04b_3_nota_do_modo_seguinte': (
        'EM CONFIRMACAO. A lista de candidatos ao jogador do ano usa a soma de notas do modo seguinte: so pesa quando o '
        'jogador do ano e escolhido pelas notas, no fim da temporada.' + _PT,
        'BEING CONFIRMED. The player of the year shortlist uses the next mode\'s rating sum: it only matters when the '
        'player of the year is picked by the ratings, at the season\'s end.' + _EN),
    'sx04b_3_um_a_mais': (
        'EM CONFIRMACAO. O jogador do ano pelas notas pede um candidato a mais: so pesa no fim da temporada, quando ha '
        'poucos candidatos.' + _PT,
        'BEING CONFIRMED. The player of the year from the ratings asks for one more candidate: it only matters at the '
        'season\'s end, when there are few candidates.' + _EN),
}  # nome: (explicacao pt, explicacao en)
