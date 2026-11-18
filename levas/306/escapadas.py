# As escapadas da 306a leva (itens mwf_1 a mwf_13), 18/11/2026.
#
# As mutacoes "ano + 2" das viradas de temporada rodaram com a carreira ate 2002-12-31 (a janela padrao, 2002-08-01,
# termina antes da primeira rodada da edicao seguinte; ver docs/COMO_ESCREVER_UM_ITEM.md). A que sobra aqui foi medida
# ate essa data.

ESCAPADAS = {
    'mwf3_itac1super_ano_mais_2': (
        'ESCAPADA EXPLICADA pela janela, medida: a mutacao poe o ano + 2 na virada da Supercopa da Serie C1 '
        '(`ita_c1_super_start_new_season`, 0x63caa0; o ramo `mwf_3 Serie C1 Super Cup rollover` marca 1 ate 2002-08-01). '
        'Com a carreira ate 2002-12-31 a mutante termina igual a base (12405153896 sorteios, impressao dbce6b4aeca802f7), '
        'enquanto as outras 19 viradas da leva sao acusadas entre 2002-083 e 2002-313. Os participantes sao os campeoes '
        'dos dois grupos da Serie C1 (`ItaC1Super::init_team_list`, 0x63c9d0), entao o jogo da edicao 2002-03 so acontece '
        'depois do fim da temporada da C1, em 2003, fora ate da janela estendida. O comparador '
        '`compare_ita_c1_super_start_new_season` confere o ano na virada (roda so a original).',
        'EXPLAINED ESCAPE by the window, measured: the mutation sets year + 2 in the turnover of the Serie C1 Super Cup '
        '(`ita_c1_super_start_new_season`, 0x63caa0; the branch `mwf_3 Serie C1 Super Cup rollover` counts 1 up to '
        '2002-08-01). With the career up to 2002-12-31 the mutant ends equal to the base (12405153896 draws, fingerprint '
        'dbce6b4aeca802f7), while the other 19 turnovers of the batch are caught between 2002-083 and 2002-313. The entrants '
        'are the winners of the two Serie C1 groups (`ItaC1Super::init_team_list`, 0x63c9d0), so the match of the 2002-03 edition '
        'only happens after the C1 season ends, in 2003, outside even the extended window. The comparer '
        '`compare_ita_c1_super_start_new_season` checks the year at the turnover (it runs only the original).'),
    'mwf13_premio_200_mil': (
        'ESCAPADA EXPLICADA, medida: a mutacao troca o fundo de premios do Auto Windscreens Shield (400.000 -> 200.000) em '
        '`eng_auto_cup_set_up_rounds` (0x56b700). Na base ate 2002-08-01 os ramos `mwf_13 auto windscreens rounds` e '
        '`rollover` marcam 1 cada: a rotina so roda dentro da virada, e o fundo novo vale para a edicao 2002-03. Mesmo '
        'com a carreira ate 2002-12-31, a mutante termina igual a base (12405153896 sorteios, 82431 partidas, impressao '
        'dbce6b4aeca802f7): o fundo so e lido em rodada sem premio fixo (`pay_round_prize_money`, `winner_prize == -1`), '
        'e dinheiro de premio nao muda sorteio nessa janela. Quem pega a mutacao e o comparador '
        '`compare_eng_auto_cup_set_up_rounds`, lado a lado, que compara o proprio campo `cup::PRIZE_FUND` na virada, '
        'dentro da janela do jogo real (0 divergencias com o codigo da leva). A confirmacao pelo jogo real com a mutacao '
        '(`--confirmar`) nao foi rodada: custa a comparacao inteira, umas 6 horas.',
        'EXPLAINED ESCAPE, measured: the mutation changes the Auto Windscreens Shield prize fund (400,000 -> 200,000) in '
        '`eng_auto_cup_set_up_rounds` (0x56b700). In the base up to 2002-08-01 the branches `mwf_13 auto windscreens '
        'rounds` and `rollover` count 1 each: the routine only runs inside the turnover, and the new fund is for the '
        '2002-03 edition. Even with the career up to 2002-12-31, the mutant ends equal to the base (12405153896 draws, '
        '82431 matches, fingerprint dbce6b4aeca802f7): the fund is only read in a round without a fixed prize '
        '(`pay_round_prize_money`, `winner_prize == -1`), and prize money changes no draw in that window. What catches '
        'the mutation is the comparer `compare_eng_auto_cup_set_up_rounds`, side by side, which compares the '
        '`cup::PRIZE_FUND` field itself at the turnover, inside the real game\'s window (0 divergences with the batch\'s '
        'code). The confirmation by the real game with the mutation (`--confirmar`) was not run: it costs the whole '
        'comparison, about 6 hours.'),
}  # nome: (explicacao pt, explicacao en)
