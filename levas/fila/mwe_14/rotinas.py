# The season turnovers of the Veikkausliiga, the Tippeligaen and the Allsvenskan (reader mwe, whole-world round,
# 12/11/2026). Each runs once in the test career, at its nation's turnover at the end of the 2001 calendar season
# (the season loop calls the top division's start_new_season through the vtable, slot +8), and leads its nation:
# - 0x595d40 FinPrm::start_new_season (448 bytes);
# - 0x7927a0 NorPrm::start_new_season (448 bytes);
# - 0x8905f0 SwePrm::start_new_season (512 bytes).
# The same skeleton (checked instruction by instruction; top_division_turnover): +0x4c = 0, the nation's promotion and
# relegation steps, the table and round dates freed, the stats archived, the year + 1, the rules, the table, the
# fixtures and the order (each with its box), the lower divisions' turnovers, the predicted positions, the board
# expectations and the referee renewal. No stage destruction (these leagues have no extra stages).
# The steps the copy calls by address (not ported here): 0x595f00 and 0x596050 (Finland, unnamed in the map), 0x792960
# nor_prm_swap_first_and_second_division_clubs and 0x792b40 (Norway), 0x890b40, 0x890980
# swe_prm_promote_regional_clubs_to_first_division and 0x890c70 swe_prm_assign_second_division_sections (Sweden).
# Equal in cm0102-1080.exe (whole).
# Conventions checked: vtable +8 -> thiscall, ret 0; the chain virtual +0xb0 (push 0/1 -> thiscall, ret 4); the steps
# (mov ecx, esi; call; their `ret` checked for 0x595f00 and 0x596050) -> thiscall, ret 0; limit_club_division_tier
# (push 1; push -3; push 3; push 1 -> thiscall, ret 0x10); free_table (push 0), archive (push 1), the virtuals +0x8c
# and +0x5c, create_fixtures, sort_table (push 0), set_club_season_expectations, renew_officials (push record, ecx =
# [0xb74340]).
# Comparers: only the original (they free, allocate and move clubs): the year + 1 (compare_state_league_new_season).
# Determinism: no stack, heap or address read as a value.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('fin_prm_start_new_season', 0x595d40, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nor_prm_start_new_season', 0x7927a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('swe_prm_start_new_season', 0x8905f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas de temporada da Veikkausliiga, da Tippeligaen e da Allsvenskan',
        'the season turnovers of the Veikkausliiga, the Tippeligaen and the Allsvenskan',
        [('fin_prm_start_new_season', 'FinPrm::start_new_season', '0x595d40', 'fin_prm.cpp', 'thiscall, ret 0',
          'a virada finlandesa, puxada pela Veikkausliiga: o acesso e o rebaixamento, dois passos sem nome [inferência: os grupos da Ykkönen refeitos], a Veikkausliiga refeita para o ano seguinte (regras, tabela de 12, jogos, ordem), depois a virada da Ykkönen, as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'the Finnish turnover, led by the Veikkausliiga: promotion and relegation, two unnamed steps [inference: the Ykkönen groups redone], the Veikkausliiga rebuilt for the next year (rules, 12-club table, fixtures, order), then the Ykkönen turnover, the predicted positions, the board expectations and the referee renewal'),
         ('nor_prm_start_new_season', 'NorPrm::start_new_season', '0x7927a0', 'nor_prm.cpp', 'thiscall, ret 0',
          'a virada norueguesa, puxada pela Tippeligaen: o acesso e o rebaixamento, a troca de clubes entre a primeira e a segunda divisões, um passo sem nome, a Tippeligaen refeita para o ano seguinte, depois a virada da primeira divisão, as posições previstas, as expectativas e os árbitros',
          'the Norwegian turnover, led by the Tippeligaen: promotion and relegation, the club swap between the First and Second Divisions, an unnamed step, the Tippeligaen rebuilt for the next year, then the First Division turnover, the predicted positions, the expectations and the referees'),
         ('swe_prm_start_new_season', 'SwePrm::start_new_season', '0x8905f0', 'swe_prm.cpp', 'thiscall, ret 0',
          'a virada sueca, puxada pela Allsvenskan: o acesso e o rebaixamento; sem a segunda divisão jogável, três clubes regionais sobem à Superettan; o nível de divisão dos clubes limitado e as seções da segunda divisão redistribuídas; a Allsvenskan refeita para o ano seguinte, depois as viradas da Superettan e da segunda divisão (quando existe), as posições previstas, as expectativas e os árbitros',
          'the Swedish turnover, led by the Allsvenskan: promotion and relegation; without a playable Second Division, three regional clubs go up to the Superettan; the clubs\' division tier limited and the Second Division sections dealt again; the Allsvenskan rebuilt for the next year, then the Superettan and Second Division (when it exists) turnovers, the predicted positions, the expectations and the referees')])
CAMPOS = []
INTERESSA = []
