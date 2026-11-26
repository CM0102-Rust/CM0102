# As escapadas da 338a leva. A confirmacao da cadeia no modo comparando vai ate 2001-08-20 (e a carga do save de
# ferias em portugues); a comparacao longa (ate 2002-06-01, bancada 4: levas/338/conf_escapadas_4.txt) decide, e o
# resultado entra num commit seguinte.

_PT = ' A comparacao longa decide.'
_EN = ' The long comparison decides.'

ESCAPADAS = {
    'sx05b_1_ano_a_menos': (
        'EM CONFIRMACAO. As datas da temporada da equipe do ano comecam um ano antes: so pesa quando a equipe do ano e '
        'montada, no fim da temporada.' + _PT,
        'BEING CONFIRMED. The team of the year season dates start one year earlier: it only matters when the team of the '
        'year is built, at the season\'s end.' + _EN),
    'sx05b_2_sem_competicao': (
        'EM CONFIRMACAO. O texto da equipe premiada perde o vinculo da competicao: so muda os vinculos da noticia.' + _PT,
        'BEING CONFIRMED. The team award text loses the competition link: it only changes the news item\'s links.' + _EN),
    'sx05b_2_atualizacao_do_inicio': (
        'EM CONFIRMACAO. A data de atualizacao da equipe premiada vira a do inicio: so pesa quando a equipe e atualizada, '
        'perto do fim da temporada.' + _PT,
        'BEING CONFIRMED. The team award update date becomes the start date: it only matters when the team is updated, '
        'near the season\'s end.' + _EN),
    'sx05b_4_simples_sem_periodo': (
        'EM CONFIRMACAO. No texto simples do premio do ano o periodo e o segundo nome trocam de lugar: so muda o texto da '
        'noticia do premio.' + _PT,
        'BEING CONFIRMED. In the plain year award text the period and the second name swap places: it only changes the '
        'award news text.' + _EN),
    'sx05b_4_manchete_sem_dois': (
        'EM CONFIRMACAO. A manchete 2 do premio do ano usa o modelo de outra: so muda o texto da manchete.' + _PT,
        'BEING CONFIRMED. The year award headline 2 uses another\'s template: it only changes the headline text.' + _EN),
    'sx05b_4_sem_nome_curto': (
        'EM CONFIRMACAO. O texto do premio do ano perde o vinculo do nome curto do vencedor: so muda os vinculos da '
        'noticia.' + _PT,
        'BEING CONFIRMED. The year award text loses the winner\'s short name link: it only changes the news item\'s '
        'links.' + _EN),
    'sx04b_5_um_jogo_a_mais': (
        'EM CONFIRMACAO. O candidato a jogador do torneio pede um jogo a mais como titular: so pesa quando o torneio '
        'escolhe o seu jogador, no fim dele.' + _PT,
        'BEING CONFIRMED. The tournament player candidate needs one more start: it only matters when the tournament picks '
        'its player, at its end.' + _EN),
    'sx04b_5_equipe_na_fracao': (
        'EM CONFIRMACAO. O candidato a equipe do torneio com titularidades exatamente na fracao passa a ficar de fora: so '
        'pesa no fim do torneio, num caso de empate exato.' + _PT,
        'BEING CONFIRMED. The tournament team candidate with starts exactly at the share is left out: it only matters at '
        'the tournament\'s end, in an exact tie.' + _EN),
    'cx16_3_sem_ordenar': (
        'EM CONFIRMACAO. A parte da America Central e do Norte da lista de entrada da copa da CONCACAF fica sem ordenar: '
        'so pesa quando a copa monta os participantes.' + _PT,
        'BEING CONFIRMED. The Central and North American part of the CONCACAF cup entry list stays unsorted: it only '
        'matters when the cup builds its entrants.' + _EN),
}  # nome: (explicacao pt, explicacao en)
