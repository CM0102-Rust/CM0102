# The constructors of the Norwegian, Swedish and Finnish First Divisions (reader mwe, whole-world round, 12/11/2026).
# Each runs once, when the new game is created with those nations loaded (docs/cobertura_mundo.txt):
# - 0x78fd40 NorFirst::NorFirst (544 bytes);
# - 0x88e5e0 SweFirst::SweFirst (544 bytes);
# - 0x592eb0 FinFirst::FinFirst (560 bytes).
# The same body (checked instruction by instruction): the English division constructor without the stadium and
# with the league's own discipline byte; the number of extra stages (1, 1, 4: malloc of 4 x that, not checked) and,
# in Finland, the last step (the South group stage 0x594cd0 and FinFirst::set_predicted_positions 0x594ff0, direct
# calls, thiscall, ret 0, instead of League::set_predicted_positions). Equal in cm0102-1080.exe (whole).
# Conventions checked: the callers push the record and the year and set ecx -> thiscall, ret 8 (the `ret 8` of the
# three); inside, League::League, load_from_temp_file (push 1), set_up_rules and create_table (direct, mov ecx),
# create_fixtures, sort_table (push 0), operator new (0x5ce) + Comp_Stats (push league), set_predicted_positions.
# Comparer: compare_league_with_stages_ctor (only the original runs: it allocates; vtable, year, record,
# discipline and, for a new league, the stage count and its vector).
# Determinism: no stack, heap or address read as a value.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_nor_first', 0x78fd40, [0x6a, 0xff, 0x68, 0x69, 0xe5, 0x95, 0x00]),
           ('construct_swe_first', 0x88e5e0, [0x6a, 0xff, 0x68, 0xf9, 0x30, 0x96, 0x00]),
           ('construct_fin_first', 0x592eb0, [0x6a, 0xff, 0x68, 0x09, 0x8f, 0x95, 0x00])]
DOCS = ('os construtores das segundas divisões da Noruega, da Suécia e da Finlândia',
        'the constructors of the Norwegian, Swedish and Finnish First Divisions',
        [('construct_nor_first', 'NorFirst::NorFirst', '0x78fd40', 'nor_first.cpp', 'thiscall, ret 8',
          'a primeira divisão norueguesa (a segunda do país): regra de disciplina 21 e uma fase depois da liga (o play-off de acesso); sem estado salvo, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas',
          'the Norwegian First Division (the second tier): discipline rule 21 and one stage after the league (the promotion play-off); with no saved state, the rules, the table, the fixtures, the order, the stats and the predicted positions'),
         ('construct_swe_first', 'SweFirst::SweFirst', '0x88e5e0', 'swe_first.cpp', 'thiscall, ret 8',
          'a Superettan sueca: regra de disciplina 28 (a da liga sueca) e uma fase depois da liga (o play-off de acesso); o mesmo corpo',
          'the Swedish Superettan: discipline rule 28 (the Swedish league one) and one stage after the league (the promotion play-off); the same body'),
         ('construct_fin_first', 'FinFirst::FinFirst', '0x592eb0', 'fin_first.cpp', 'thiscall, ret 8',
          'a Ykkönen finlandesa: regra de disciplina 10 (a da liga finlandesa) e quatro fases depois do grupo Norte; o mesmo corpo, e no fim a fase do grupo Sul e as posições previstas dos dois grupos',
          'the Finnish Ykkönen: discipline rule 10 (the Finnish league one) and four stages after the North group; the same body, and at the end the South group stage and the predicted positions of both groups')])
CAMPOS = [('liga +0x2c fases a mais, +0xc vetor das fases; valor e ponteiro; grava', 'league +0x2c extra stages, +0xc stage vector; value and pointer; writes')]
INTERESSA = []
