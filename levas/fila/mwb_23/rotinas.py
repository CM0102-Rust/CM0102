# As duas copas galesas em grupos (wel_lge_cup.cpp, wel_prm_cup.cpp, em cups\): os calendários e a virada da Copa
# Premier. Leitor r4_mwab (20/11/2026), lista do mundo (Escócia e País de Gales), continuação do mwb.
# - 0x9261e0 (WelLgeCup::create_schedule): create_schedule (cqo_10) com a fase 5, rótulo 0x3e9, 6 rodadas de grupo e 3
#   de mata-mata (rounds.py: as tabelas conferem).
# - 0x927b30 (WelPrmCup::create_schedule): o mesmo quadro (create_schedule_with_years: a última rodada de grupo cai no
#   ano seguinte, com o deslocamento de ano que o create_schedule não tem), fase 1, rótulo 0x420, 6 rodadas de grupo e
#   3 de mata-mata com prêmios.
# - 0x9281f0 (WelPrmCup::start_new_season): start_irish_league_cup_season (mwa_18) com as rotinas da Copa Premier e um
#   grupo (same.py contra 0x637760: registros trocados; o laço dos grupos é uma chamada só, cujo retorno passa adiante).
# Convenções conferidas: os calendários thiscall, ret 0x10 (0x68a160 e 0x68a1c0, 0x521e60 e 0x521eb0 cdecl); a virada
# thiscall, ret 0.
# Comparadores: os calendários lado a lado (ita_cup_compare_schedule); a virada só a original (o ano sobe um).
# A virada fica sem mutação própria: a edição nova só joga depois de 2002-08-01 (o molde já está provado no mwa_18).
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('wel_lge_cup_create_schedule', 0x9261e0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wel_prm_cup_create_schedule', 0x927b30, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wel_prm_cup_start_new_season', 0x9281f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as copas galesas em grupos: os calendários e a virada da Copa Premier',
        "the Welsh group cups: the schedules and the Premier Cup's season turnover",
        [('wel_lge_cup_create_schedule', 'WelLgeCup::create_schedule', '0x9261e0', 'wel_lge_cup.cpp', 'thiscall, ret 0x10',
          'regra: os grupos da Copa da Liga galesa (fases até 4 e a tabela da copa) jogam 6 rodadas semanais de 23 de agosto a 27 de setembro, cada grupo uma fase de liga (rótulo 0x3e9 + fase); o mata-mata (fase 5) de 8 clubes: quartas em 11 de outubro (jogo único, categoria 4), semifinais em ida e volta com 14 dias a partir de 25 de outubro e a final em 10 de janeiro do ano seguinte (categoria 6)',
          "rule: the Welsh League Cup groups (stages up to 4 and the cup's table) play 6 weekly matchdays from 23 August to 27 September, each group a league stage (label 0x3e9 + stage); the 8-club knockout (stage 5): quarter-finals on 11 October (single matches, category 4), two-legged semi-finals 14 days apart from 25 October and the final on 10 January of the next year (category 6)"),
         ('wel_prm_cup_create_schedule', 'WelPrmCup::create_schedule', '0x927b30', 'wel_prm_cup.cpp', 'thiscall, ret 0x10',
          'regra: os dois grupos de 4 da Copa Premier galesa jogam 6 rodadas de 5 de setembro a 9 de janeiro (5/9, 3/10, 10/10, 14/11, 12/12 e 9/1), cada grupo uma fase de liga (rótulo 0x420 + fase); o mata-mata (fase 1) de 8 clubes no ano seguinte: quartas em 13 de fevereiro (15.000 a cada clube [inferência]), semifinais em ida e volta a partir de 10 de abril (10.000) e a final em 13 de maio (100.000 ao campeão e 50.000 ao vice)',
          "rule: the Welsh Premier Cup's two groups of 4 play 6 matchdays from 5 September to 9 January (5/9, 3/10, 10/10, 14/11, 12/12 and 9/1), each group a league stage (label 0x420 + stage); the 8-club knockout (stage 1) in the next year: quarter-finals on 13 February (15,000 to each club [inference]), two-legged semi-finals from 10 April (10,000) and the final on 13 May (100,000 to the winner and 50,000 to the loser)"),
         ('wel_prm_cup_start_new_season', 'WelPrmCup::start_new_season', '0x9281f0', 'wel_prm_cup.cpp', 'thiscall, ret 0',
          'regra: na virada, a Copa Premier galesa é refeita: a tabela, as rodadas, os inscritos e as fases da edição passada liberados, as estatísticas arquivadas, o ano sobe; depois os 12 inscritos, as posições previstas, as regras, a tabela do primeiro grupo, os seus jogos e ordem e a fase do segundo grupo',
          "rule: at the turnover the Welsh Premier Cup is rebuilt: last edition's table, rounds, entrants and stages freed, the statistics archived, the year moves on; then the 12 entrants, the predicted positions, the rules, the first group's table, its fixtures and order and the second group's stage")])
CAMPOS = []
INTERESSA = []
