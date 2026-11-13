# The Croatian leagues: the season turnover (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp). Reader mwd (12/11/2026), round 2
# (1 call each, the turnover of summer 2002: the First Division's calls the two second division groups').
# - 0x5149a0 CroA1::start_new_season: the chain (virtual +0xb0, push 1), 0x514ed0 (the third tier swap, mwd_6), 0x515190
#   (the north/south regrouping by latitude, cdecl, not ported), the rebuild, the tier cap of both groups (0x68a980,
#   push 1; push -3; push 3; push 1 -> thiscall, ret 0x10), both groups' turnovers (virtual +8), the predicted positions
#   (virtual +0x5c), 0x68aa80 (thiscall, ret 0) and 0x79cee0 (push +4; mov ecx, [0xb74340] -> thiscall, ret 4).
# - 0x515ea0 CroA2a::start_new_season, 0x516fc0 CroA2b::start_new_season: the same 448-byte template (only the file).
# Comparer: compare_state_league_new_season (only the original; the year +1). Equal in cm0102-1080.exe.
# Depends on: bra_nat_first_call_virtual_no_args, comp_by_global, set_expectations_and_renew_officials
# (bra_nat_first.rs), compare_state_league_new_season (bra_reg_bahia.rs).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('cro_a1_start_new_season', 0x5149a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cro_a2a_start_new_season', 0x515ea0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cro_a2b_start_new_season', 0x516fc0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
_R = 'a temporada reaberta; a tabela e as datas liberadas, as estatísticas arquivadas, o vetor de fases liberado; o ano seguinte; as regras, a tabela, os jogos e a ordem refeitos; as posições previstas, as expectativas da diretoria e a renovação dos árbitros'
_E = 'the season reopened; the table and the dates freed, the statistics archived, the stage vector freed; the next year; the rules, the table, the fixtures and the order rebuilt; the predicted positions, the board expectations and the officials renewed'
DOCS = ('as ligas croatas: a virada da temporada', 'the Croatian leagues: the season turnover',
        [('cro_a1_start_new_season', 'CroA1::start_new_season', '0x5149a0', 'cro_a1.cpp', 'thiscall, ret 0',
          'regra: a virada croata, puxada pela Primeira Divisão: o acesso e o rebaixamento pela cadeia Primeira -> grupo norte -> grupo sul; os quatro melhores da terceira divisão sobem à segunda e os rebaixados dela descem; os 32 clubes da segunda reagrupados em norte e sul pela latitude da cidade; ' + _R + '; o nível de divisão dos clubes dos dois grupos limitado e a virada dos dois grupos',
          "rule: the Croatian turnover, led by the First Division: promotion and relegation along the chain First Division -> north group -> south group; the four best third-tier clubs up into the second division and its relegated clubs down; the 32 second division clubs regrouped north and south by the latitude of their city; " + _E + "; the division tier of both groups' clubs capped and both groups turned over"),
         ('cro_a2a_start_new_season', 'CroA2a::start_new_season', '0x515ea0', 'cro_a2a.cpp', 'thiscall, ret 0',
          'regra: a virada do grupo norte da Segunda Divisão croata (chamada pela da Primeira): ' + _R,
          "rule: the Croatian Second Division north group turnover (called by the First Division's): " + _E),
         ('cro_a2b_start_new_season', 'CroA2b::start_new_season', '0x516fc0', 'cro_a2b.cpp', 'thiscall, ret 0',
          'regra: a virada do grupo sul da Segunda Divisão croata, como a do grupo norte',
          "rule: the Croatian Second Division south group turnover, as the north group's")])
CAMPOS = [('liga +0x4c, +0xba, +8, +0xc, +0x2c, +0x30, +0x40', 'league +0x4c, +0xba, +8, +0xc, +0x2c, +0x30, +0x40')]
