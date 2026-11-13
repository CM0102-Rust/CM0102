# The season turnovers of the Finnish, Norwegian and Portuguese cups (reader mwe, whole-world round, 12/11/2026). Each
# runs once in the test career, when its cup's new edition starts (docs/cobertura_mundo.txt):
# - 0x592d00 FinCup::start_new_season (432 bytes);
# - 0x78fb90 NorCup::start_new_season (432 bytes);
# - 0x7ce540 PorCup::start_new_season (432 bytes).
# The body of start_new_cup_season (the knockout cups' turnover, bra_cup.rs; the same as the Japanese and American
# cups'), checked instruction by instruction: only the entrants routine, the source and the box lines change. Equal
# in cm0102-1080.exe (whole).
# Conventions checked: through the vtable (+8, mov ecx) -> thiscall, ret 0 (plain `ret`); inside, free (cdecl),
# archive_season_and_reset (push 1), collect_entrants (direct, mov ecx -> thiscall, ret 0; the jge on its eax), the
# virtuals +0x8c, +0x94 and +0x5c (no push -> thiscall, ret 0).
# Comparer: compare_cup_new_season (only the original: it frees and allocates; the year + 1).
# Determinism: no stack, heap or address read as a value.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('fin_cup_start_new_season', 0x592d00, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nor_cup_start_new_season', 0x78fb90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('por_cup_start_new_season', 0x7ce540, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as viradas de temporada das copas da Finlândia, da Noruega e de Portugal',
        'the season turnovers of the Finnish, Norwegian and Portuguese cups',
        [('fin_cup_start_new_season', 'FinCup::start_new_season', '0x592d00', 'fin_cup.cpp', 'thiscall, ret 0',
          'a virada da Copa da Finlândia: soltas as tabelas de inscritos, rodadas e confrontos da edição anterior, a temporada arquivada nas estatísticas e o ano + 1; depois os inscritos, as rodadas, a árvore dos confrontos e as posições previstas',
          'the Finnish Cup turnover: last edition\'s entrant, round and tie tables freed, the season archived in the statistics and the year + 1; then the entrants, the rounds, the tie tree and the predicted positions'),
         ('nor_cup_start_new_season', 'NorCup::start_new_season', '0x78fb90', 'nor_cup.cpp', 'thiscall, ret 0',
          'a mesma virada na Copa da Noruega', 'the same turnover in the Norwegian Cup'),
         ('por_cup_start_new_season', 'PorCup::start_new_season', '0x7ce540', 'por_cup.cpp', 'thiscall, ret 0',
          'a mesma virada na Taça de Portugal', 'the same turnover in the Portuguese Cup')])
CAMPOS = []
INTERESSA = []
