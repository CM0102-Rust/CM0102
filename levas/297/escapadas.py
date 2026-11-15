# As escapadas da 297a leva (explicadas pelo codigo e pelo diario da base, copias/bancada2/jogo_testes/dias_base.txt,
# com os contadores de ramo da linha "ramos na base" de levas/297/cadeia.txt).
#
# As duas que escaparam sao do tipo "a virada da copa pula um ano", o mesmo padrao ja provado na 295a leva
# (levas/295/escapadas.py): o ano novo da copa so e lido quando a primeira rodada da edicao seguinte chega, e na Copa
# da Escocia e na de Gales essa rodada cai no outono, meses depois do ultimo dia que a cadeia processa (2002-212,
# ATE = 2002-08-01). A mesma troca na Copa da Liga da Escocia (`mwb4_challenge_pula_um_ano`, a mesma rotina
# `start_new_cup_season` com o mesmo argumento) foi acusada em 2002-181, porque a edicao nova dela e sorteada em 28/6
# e jogada em 10/7: a diferenca entre as tres e so a data da primeira rodada.
_VIRADA_PT = (
    'o argumento trocado e o passo do ano da virada (`start_new_cup_season`, o ano `cup::YEAR` (+0x40) sobe 2 em vez '
    'de 1). O unico leitor de +0x40 na simulacao e a atualizacao diaria da copa (`Cup::update_for_date` 0x51f890): '
    'ela monta a data de cada rodada com o dia do ano (`cup_round::DAY`, +0x3 do registro de 0x68 bytes) e o '
    'deslocamento de ano (`cup_round::YEAR_OFFSET`, +0x5) somado a +0x40 e, '
    'enquanto a data nao chegou (`ano > hoje.ano || (ano == hoje.ano && dia > hoje.dia)`), devolve 0 sem sortear '
    '(`exe::cup_draw_round` 0x51c410) nem montar a rodada; os outros usos de +0x40 sao a gravacao e a leitura do '
    '`.tmp` da competicao e os rotulos de texto, que nao voltam para a simulacao. A virada em si roda uma vez so, '
    'pela passada das temporadas (`game_season_update_pass` 0x5d5ae0 -> `season_pass_restart_comp` -> '
    '`comp_vtable::START_NEW_SEASON`): o contador "{ramo}" marcou 1 na base. Logo depois dela o que e refeito no '
    'mesmo dia (os inscritos, a tabela de rodadas e a arvore dos confrontos) nao le o ano: a tabela de rodadas grava '
    'so o dia do ano e o deslocamento de ano, e e a atualizacao diaria que soma +0x40. '
    'O comparador da rotina (`compare_cup_new_season`: so a original roda, e confere `cup::YEAR` = anterior + 1) '
    'pegaria a troca, mas a comparacao para em 2001-232 (20/8/2001), quase um ano antes da virada. '
)
_VIRADA_EN = (
    "the swapped argument is the year step of the turnover (`start_new_cup_season`: `cup::YEAR` (+0x40) goes up by 2 "
    "instead of 1). The only reader of +0x40 in the simulation is the cup's daily update (`Cup::update_for_date` "
    "0x51f890): it builds each round's date from the day of the year (`cup_round::DAY`, +0x3 of the 0x68-byte "
    'record) and the year offset (`cup_round::YEAR_OFFSET`, +0x5) added to +0x40 and, while the date has not arrived (`year > today.year || (year == today.year && day > today.day)`), '
    'returns 0 without drawing (`exe::cup_draw_round` 0x51c410) or building the round; the other uses of +0x40 are '
    "the writing and reading of the competition's `.tmp` and the text labels, which do not flow back into the "
    'simulation. The turnover itself runs only once, from the season pass (`game_season_update_pass` 0x5d5ae0 -> '
    '`season_pass_restart_comp` -> `comp_vtable::START_NEW_SEASON`): the counter "{ramo}" read 1 in the base. What '
    'is rebuilt right after it on the same day (the entrants, the round table and the tie tree) does not read the '
    'year: the round table stores only the day of the year and the year offset, and it is the daily update that '
    "adds +0x40. The routine's comparer (`compare_cup_new_season`: only the original runs, and "
    'it checks `cup::YEAR` = previous + 1) would catch the swap, but the comparison stops at 2001-232 (20 August '
    '2001), almost a year '
    'before the turnover. '
)
ESCAPADAS = {
    'mwb4_copa_da_escocia_pula_um_ano': (
        _VIRADA_PT.format(ramo='mwb_4 scottish cup turnover') +
        'A Copa da Escocia so comeca no fim do outono: a rodada 0 da tabela (`SCOTTISH_CUP_ROUNDS`) e sorteada em 8/11 '
        'e jogada em 5/12 do ano da copa, isto e 8/11/2002 e 5/12/2002 na edicao nova (e um ano mais com a mutacao), '
        'tres e quatro meses depois do ultimo dia da janela (2002-212, 31/7/2002); nenhuma rodada da edicao nova e '
        'sorteada dentro da janela. O contraste mede a discriminacao do teste: a mesma troca na Copa da Liga da '
        'Escocia (`mwb4_challenge_pula_um_ano`), cuja edicao nova e sorteada em 28/6 e jogada em 10/7, foi acusada em '
        '2002-181 (provado pelo codigo)',
        _VIRADA_EN.format(ramo='mwb_4 scottish cup turnover') +
        'The Scottish Cup only starts in late autumn: round 0 of the table (`SCOTTISH_CUP_ROUNDS`) is drawn on 8 '
        "November and played on 5 December of the cup's year, that is 8 November 2002 and 5 December 2002 in the new "
        'edition (and a year later with the mutation), three and four months past the last day of the window '
        '(2002-212, 31 July 2002); no round of the new edition is drawn inside the window. The contrast measures the '
        "test's reach: the same swap in the Scottish Challenge Cup (`mwb4_challenge_pula_um_ano`), whose new edition "
        'is drawn on 28 June and played on 10 July, was caught on 2002-181 (proved from the code)'),
    'mwb4_copa_de_gales_pula_um_ano': (
        _VIRADA_PT.format(ramo='mwb_4 welsh cup turnover') +
        'A Copa de Gales tambem comeca no outono: a rodada 0 da tabela (`WELSH_CUP_ROUNDS`) e sorteada em 19/10 e '
        'jogada em 18/11 do ano da copa, isto e 19/10/2002 e 18/11/2002 na edicao nova (e um ano mais com a mutacao), '
        'dois e tres meses depois do ultimo dia da janela (2002-212, 31/7/2002); nenhuma rodada da edicao nova e '
        'sorteada dentro da janela. O contraste mede a discriminacao do teste: a mesma troca na Copa da Liga da '
        'Escocia (`mwb4_challenge_pula_um_ano`), cuja edicao nova e sorteada em 28/6 e jogada em 10/7, foi acusada em '
        '2002-181 (provado pelo codigo)',
        _VIRADA_EN.format(ramo='mwb_4 welsh cup turnover') +
        'The Welsh Cup also starts in the autumn: round 0 of the table (`WELSH_CUP_ROUNDS`) is drawn on 19 October '
        "and played on 18 November of the cup's year, that is 19 October 2002 and 18 November 2002 in the new edition "
        '(and a year later with the mutation), two and three months past the last day of the window (2002-212, 31 '
        "July 2002); no round of the new edition is drawn inside the window. The contrast measures the test's reach: "
        'the same swap in the Scottish Challenge Cup (`mwb4_challenge_pula_um_ano`), whose new edition is drawn on 28 '
        'June and played on 10 July, was caught on 2002-181 (proved from the code)'),
}  # nome: (explicacao pt, explicacao en)
