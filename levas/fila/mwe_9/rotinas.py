# The season turnovers of the Swedish, Norwegian and Finnish First Divisions (reader mwe, whole-world round,
# 12/11/2026). Each runs once in the test career, at the turnover of its nation (the end of the 2001 calendar season,
# called through the vtable by the Premier Division's start_new_season):
# - 0x88f150 SweFirst::start_new_season (448 bytes) and 0x790b90 NorFirst::start_new_season (448 bytes): the body of
#   rebuild_season + set_expectations_and_renew_officials (the Brazilian Series B, cqq_8), checked instruction by
#   instruction (only the table routine, the source and the box lines change);
# - 0x593c20 FinFirst::start_new_season (496 bytes): the same steps, but the stage vector is freed and allocated
#   again, and the end is the expectations, the referees, the South group stage (0x594cd0) and the predicted
#   positions (virtual +0x5c, whose eax is returned).
# Equal in cm0102-1080.exe (whole).
# Conventions checked: through the vtable (start_new_season, mov ecx) -> thiscall, ret 0 (plain `ret`); inside,
# free_table (push 0), free (cdecl), archive_season_and_reset (push 1), the stage destructors (virtual 0, push 1),
# vtable +0x8c, create_table (direct), create_fixtures, sort_table (push 0), vtable +0x5c,
# set_club_season_expectations, renew_officials (push record, ecx = [0xb74340]), malloc (cdecl).
# Comparers: only the original (they free and allocate): the year + 1 (compare_state_league_new_season); the Finnish
# one also the stage vector.
# Determinism: no stack, heap or address read as a value.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('swe_first_start_new_season', 0x88f150, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nor_first_start_new_season', 0x790b90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('fin_first_start_new_season', 0x593c20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas de temporada das segundas divisões da Suécia, da Noruega e da Finlândia',
        'the season turnovers of the Swedish, Norwegian and Finnish First Divisions',
        [('swe_first_start_new_season', 'SweFirst::start_new_season', '0x88f150', 'swe_first.cpp', 'thiscall, ret 0',
          'a virada da Superettan: arquiva a temporada nas estatísticas, limpa a tabela, as datas e as fases, passa ao ano seguinte e refaz as regras, a tabela de 16, os jogos e a ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros',
          'the Superettan turnover: archives the season in the statistics, clears the table, the dates and the stages, moves to the next year and rebuilds the rules, the 16-club table, the fixtures and the order; then the predicted positions, the board expectations and the referee renewal'),
         ('nor_first_start_new_season', 'NorFirst::start_new_season', '0x790b90', 'nor_first.cpp', 'thiscall, ret 0',
          'a virada da primeira divisão norueguesa: a mesma da Superettan',
          'the Norwegian First Division turnover: the same as the Superettan one'),
         ('fin_first_start_new_season', 'FinFirst::start_new_season', '0x593c20', 'fin_first.cpp', 'thiscall, ret 0',
          'a virada da Ykkönen: a mesma, mas o vetor das fases é solto e alocado de novo, e no fim, depois das expectativas e dos árbitros, a fase do grupo Sul e as posições previstas dos dois grupos',
          'the Ykkönen turnover: the same, but the stage vector is freed and allocated again, and at the end, after the expectations and the referees, the South group stage and the predicted positions of both groups')])
CAMPOS = []
INTERESSA = []
