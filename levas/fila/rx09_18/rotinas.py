# A terceira fase, a repescagem e as posicoes previstas das eliminatorias asiaticas da Copa de 2006 (wc_asia_league.cpp;
# leitor rx09, rodada 6, 25/11/2026). Simulacao com a Inglaterra ate 2005-12-31: as previsoes no dia do sorteio
# (marco de 2005); a terceira fase no fim da segunda (outubro de 2005) e a repescagem no fim dela (novembro).
# - 0x913b60 WcAsiaLeague::create_third_round_tie thiscall() ret 0: malloc 8 (caixa 0x552); sem isentos a linha 2 das
#   fases 9 e 10 (League::get_table_row 0x6826d0), senao a 1; a virtual +0x3c(11, ...); Cup_Stage 0x522e00 (this, 2,
#   lista, rodadas, registro, tabela, ano, 0xb, 4, extra, 0x14, 0, 0, 0, 0) em +0xc[11]; free da lista e da tabela antes
#   de conferir; caixa 0x572; Cup::set_up_round(0) 0x51c800 e +0x30 = 11.
# - 0x913d50 WcAsiaLeague::create_playoff_tie thiscall() ret 0: malloc 8 (caixa 0x591); a primeira entrada de status 2
#   (com um isento) ou 1 da tabela de times da fase 11 (+0xa7, +0x3e relido); a europeia pela 0x91def0 (WcEuropeLeague,
#   sem nome no mapa) no objeto da secao europeia ([0xadadfc + 4 x [0x9cf780]]); Cup_Stage(.., 0xc, 4, ..) em +0xc[12];
#   caixa 0x5be; Cup::set_up_round(0) e +0x30 = 12.
# - 0x9144d0 WcAsiaLeague::set_predicted_positions thiscall() ret 0: sem estatisticas a caixa 0x6bb; qsort(+0xef, +0xf3,
#   4, 0x4b07d0); NationalTeams::release_competitions_of_continent 0x775070(o continente asiatico [0xae23a4] + 0xc6 x
#   [0x9cfa0c], 0); para cada inscrita k (palavra): a posicao pelas faixas do byte +0xee relido (conferidas uma a uma),
#   set_club_final_position 0x4a2540, e NationalTeams::set_current_competition 0x775220(nacao +0x53, registro).
# Prologos mov eax, fs:[0] (6 bytes) e sub esp, 0x200 (6 bytes). Iguais no cm0102-1080.exe.
# Comparadores: so a original nas tres; os planos depois (os dois lados de cada confronto e a fase atual; a posicao
# prevista de cada inscrita).
# Depende do rx09_16 (WC_ASIA_EXEMPT, WC_ASIA_ENTRANT_COUNT), do rx09_3 (league_cup_stage, compare_league_cup_stage) e do
# cx14_6 (stage_round_table).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_asia_league_create_third_round_tie', 0x913b60, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('wc_asia_league_create_playoff_tie', 0x913d50, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('wc_asia_league_set_predicted_positions', 0x9144d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a terceira fase, a repescagem e as posições previstas das eliminatórias asiáticas',
        'the third round, the play-off and the predicted positions of the Asian World Cup qualifying',
        [('wc_asia_league_create_third_round_tie', 'WcAsiaLeague::create_third_round_tie', '0x913b60', 'wc_asia_league.cpp', 'thiscall, ret 0',
          'regra: os terceiros colocados dos dois grupos da segunda fase (os segundos, quando há isentos) jogam a terceira fase em ida e volta',
          'rule: the third-placed teams of the two second round groups (the runners-up when there are exempt nations) play the third round, home and away'),
         ('wc_asia_league_create_playoff_tie', 'WcAsiaLeague::create_playoff_tie', '0x913d50', 'wc_asia_league.cpp', 'thiscall, ret 0',
          'regra: o vencedor da terceira fase (o perdedor, com um isento) joga a repescagem em ida e volta contra a seleção europeia escolhida para ela',
          'rule: the third round winner (the loser, with one exempt nation) plays the play-off, home and away, against the European nation picked for it'),
         ('wc_asia_league_set_predicted_positions', 'WcAsiaLeague::set_predicted_positions', '0x9144d0', 'wc_asia_league.cpp', 'thiscall, ret 0',
          'regra: as inscritas são ordenadas pela reputação e recebem a posição prevista por faixas (as quatro primeiras 1 sem isentos, depois 5, 6, 7, 9, 11, 21 e 31); as seleções asiáticas saem das competições do continente e passam a jogar esta',
          'rule: the entrants are sorted by reputation and get their predicted place by bands (the top four 1 with no exempt nation, then 5, 6, 7, 9, 11, 21 and 31); the Asian national teams leave the continent\'s competitions and play this one')])
CAMPOS = []
INTERESSA = []
