# The Greek season turnover (gre_prm.cpp, gre_second.cpp). Reader mwd (12/11/2026), round 2 (1 call each, the turnover
# of summer 2002).
# - 0x5eaf50 GrePrm::start_new_season: the chain with 0 (push 0), 0x5eb120 (the swap), the rebuild (the Croatian one
#   without +0x4c = 1: rebuild_croatian_league(.., false)), the Second Division's turnover (virtual +8), the predicted
#   positions, 0x68aa80 and 0x79cee0. No tier cap.
# - 0x5ebe90 GreSecond::start_new_season: the rebuild without reopening and the end.
# - 0x5eb120 gre_prm_swap_second_and_third_division_clubs: like the Croatian swap (mwd_6) with no nation check, 2 up
#   (picks 3) and one group; the map says cdecl(void) but ecx (the First Division) is passed as this to 0x6830b0 and
#   0x6831a0: thiscall, ret 0.
# Conventions: thiscall, ret 0; callees as in mwd_5 and mwd_6.
# Comparers: the turnovers only the original (the year +1); the swap only the original, the relegated clubs checked in
# the Third Division (when there are at least 2 candidates). Equal in cm0102-1080.exe.
# Depends on: rebuild_croatian_league / finish_croatian_turnover (mwd_5, with the reopen flag).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('gre_prm_start_new_season', 0x5eaf50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('gre_second_start_new_season', 0x5ebe90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('gre_prm_swap_second_and_third_division_clubs', 0x5eb120, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a virada da temporada grega', 'the Greek season turnover',
        [('gre_prm_start_new_season', 'GrePrm::start_new_season', '0x5eaf50', 'gre_prm.cpp', 'thiscall, ret 0',
          'regra: a virada grega, puxada pela Primeira Divisão: o acesso e o rebaixamento pela cadeia; os dois melhores da Terceira Divisão sobem à Segunda e os rebaixados dela descem; a Primeira refeita para o ano seguinte (sem reabrir a temporada: +0x4c fica 0); a virada da Segunda; as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'rule: the Greek turnover, led by the First Division: promotion and relegation along the chain; the two best Third Division clubs up into the Second Division and its relegated clubs down; the First Division rebuilt for the next year (without reopening the season: +0x4c stays 0); the Second Division turned over; the predicted positions, the board expectations and the officials renewed'),
         ('gre_second_start_new_season', 'GreSecond::start_new_season', '0x5ebe90', 'gre_second.cpp', 'thiscall, ret 0',
          'regra: a virada da Segunda Divisão grega (chamada pela da Primeira): a tabela, as datas e o vetor de fases liberados, as estatísticas arquivadas, o ano seguinte, as regras, a tabela, os jogos e a ordem refeitos; as posições previstas, as expectativas da diretoria e os árbitros',
          "rule: the Greek Second Division turnover (called by the First Division's): the table, the dates and the stage vector freed, the statistics archived, the next year, the rules, the table, the fixtures and the order rebuilt; the predicted positions, the board expectations and the officials"),
         ('gre_prm_swap_second_and_third_division_clubs', 'gre_prm_swap_second_and_third_division_clubs', '0x5eb120', 'gre_prm.cpp', 'thiscall (ecx = a Primeira Divisão), ret 0',
          'regra: na virada grega, os dois melhores clubes da Terceira Divisão (ordenados por força, os nove melhores embaralhados) sobem à Segunda, e os rebaixados (situação 3) da Segunda descem à Terceira; com menos de 2 candidatos ninguém sobe nem desce',
          'rule: at the Greek turnover, the two best Third Division clubs (sorted by strength, the best nine shuffled) go up to the Second Division, and its relegated clubs (status 3) go down to the Third; with fewer than 2 candidates nobody moves')])
CAMPOS = [('liga +0x4c, +0xba, +8, +0xc, +0x2c, +0x30, +0x40; clube +0x57 divisão; linha da tabela +0x37 situação', 'league +0x4c, +0xba, +8, +0xc, +0x2c, +0x30, +0x40; club +0x57 division; table row +0x37 status')]
