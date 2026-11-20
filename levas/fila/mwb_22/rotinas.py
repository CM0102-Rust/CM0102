# A Copa da Liga galesa (wel_lge_cup.cpp, em cups\): os inscritos, as posições previstas e a virada da temporada.
# Leitor r4_mwab (20/11/2026), lista do mundo (Escócia e País de Gales), continuação do mwb.
# - 0x927010 (WelLgeCup::collect_entrants): calloc (0x945501) de 18; os clubes da Liga de Gales na ordem da tabela, sem
#   limite; qsort (0x945333, cdecl) pela reputação (0x4b07d0); 0x4afb40 nas seis faixas de 3; menos de 18: a caixa da
#   linha 0x2e7.
# - 0x9274f0 (WelLgeCup::set_predicted_positions): italian_cup_predicted_positions com a fase 5 e faixas de 6 (same.py
#   contra 0x63e580: 6 diferenças).
# - 0x9268a0 (WelLgeCup::start_new_season): start_irish_league_cup_season (mwa_18) com as rotinas galesas e 5 grupos
#   (same.py contra 0x637760: 11 diferenças, rotinas, arquivo, linhas e contagem).
# Convenções conferidas: thiscall, ret 0.
# Comparadores: os inscritos lado a lado com o gerador restaurado (compare_select_entrants; 18 entradas); as posições
# lado a lado; a virada só a original (compare_state_league_new_season: o ano sobe um).
# A virada fica sem mutação própria: a edição nova só joga depois de 2002-08-01 (o molde já está provado no mwa_18).
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('wel_lge_cup_collect_entrants', 0x927010, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wel_lge_cup_set_predicted_positions', 0x9274f0, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('wel_lge_cup_start_new_season', 0x9268a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa da Liga galesa: os inscritos, as posições previstas e a virada da temporada',
        "the Welsh League Cup: the entrants, the predicted positions and the season turnover",
        [('wel_lge_cup_collect_entrants', 'WelLgeCup::collect_entrants', '0x927010', 'wel_lge_cup.cpp', 'thiscall, ret 0',
          'regra: os 18 clubes da Liga de Gales, pela reputação e embaralhados dentro de cada faixa de 3 (os potes do sorteio: cada um dos seis grupos leva um clube de cada pote [inferência]); com menos de 18 na liga, a caixa de erro',
          "rule: the 18 clubs of the League of Wales, by reputation and shuffled within each band of 3 (the draw pots: each of the six groups takes one club from each pot [inference]); with fewer than 18 in the league, the error box"),
         ('wel_lge_cup_set_predicted_positions', 'WelLgeCup::set_predicted_positions', '0x9274f0', 'wel_lge_cup.cpp', 'thiscall, ret 0',
          'regra: as posições previstas dos 18 inscritos, pela reputação: o mata-mata (fase 5) dá 1 ao campeão e 2 ao vice, depois uma posição a cada leva de eliminados; os outros em faixas de 6 (a expectativa da diretoria e da imprensa [inferência])',
          "rule: the 18 entrants' predicted positions, by reputation: the knockout (stage 5) gives 1 to the winner and 2 to the runner-up, then one place to each round's losers; the rest in bands of 6 (the board and press expectation [inference])"),
         ('wel_lge_cup_start_new_season', 'WelLgeCup::start_new_season', '0x9268a0', 'wel_lge_cup.cpp', 'thiscall, ret 0',
          'regra: na virada, a Copa da Liga galesa é refeita: a tabela, as rodadas, os inscritos e as fases da edição passada liberados, as estatísticas arquivadas, o ano sobe; depois os 18 inscritos, as posições previstas, as regras, a tabela do primeiro grupo, os seus jogos e ordem e as fases dos outros 5 grupos',
          "rule: at the turnover the Welsh League Cup is rebuilt: last edition's table, rounds, entrants and stages freed, the statistics archived, the year moves on; then the 18 entrants, the predicted positions, the rules, the first group's table, its fixtures and order and the stages of the other 5 groups")])
CAMPOS = []
INTERESSA = []
