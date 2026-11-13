# The constructors of the Norwegian Premier Division and of the Portuguese Premier and Second Divisions (reader mwe,
# whole-world round, 12/11/2026). Each runs once, when the new game is created with Norway and Portugal loaded
# (docs/cobertura_mundo.txt), and again never (the saved state is not loaded in the test career):
# - 0x791500 NorPrm::NorPrm (528 bytes);
# - 0x7ce720 PorPrm::PorPrm (528 bytes);
# - 0x7d0950 PorSecond::PorSecond (528 bytes).
# The three are the body of construct_division_league (the French and Spanish constructors, cqm_5), checked
# instruction by instruction: only the vtable, the discipline byte, the two direct calls (set_up_rules,
# create_table), the source file and the box lines change. Equal in cm0102-1080.exe (whole).
# Conventions checked: the callers (the competition setup) push the record and the year and set ecx -> thiscall,
# ret 8 (the `ret 8` of the three checked); inside, the same calls as construct_division_league (League::League,
# load_from_temp_file push 1, create_fixtures, sort_table push 0, operator new + Comp_Stats push league,
# set_predicted_positions).
# Comparer: compare_division_league_ctor (only the original runs: it allocates; the vtable, year, record, discipline
# and, for a new league, no stages and the stats).
# Determinism: no stack, heap or address read as a value.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_nor_prm', 0x791500, [0x6a, 0xff, 0x68, 0x19, 0xe6, 0x95, 0x00]),
           ('construct_por_prm', 0x7ce720, [0x6a, 0xff, 0x68, 0x29, 0xfa, 0x95, 0x00]),
           ('construct_por_second', 0x7d0950, [0x6a, 0xff, 0x68, 0x79, 0xfa, 0x95, 0x00])]
DOCS = ('os construtores da primeira divisão norueguesa e da primeira e da segunda divisões portuguesas',
        'the constructors of the Norwegian Premier Division and of the Portuguese Premier and Second Divisions',
        [('construct_nor_prm', 'NorPrm::NorPrm', '0x791500', 'nor_prm.cpp', 'thiscall, ret 8',
          'a primeira divisão norueguesa: regra de disciplina 21 (a da liga norueguesa), só a liga (sem fase de play-off); sem estado salvo, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas',
          'the Norwegian Premier Division: discipline rule 21 (the Norwegian league one), the league only (no play-off stage); with no saved state, the rules, the table, the fixtures, the order, the stats and the predicted positions'),
         ('construct_por_prm', 'PorPrm::PorPrm', '0x7ce720', 'por_prm.cpp', 'thiscall, ret 8',
          'a primeira divisão portuguesa: regra de disciplina 23 (a portuguesa), só a liga; sem estado salvo, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas',
          'the Portuguese Premier Division: discipline rule 23 (the Portuguese one), the league only; with no saved state, the rules, the table, the fixtures, the order, the stats and the predicted positions'),
         ('construct_por_second', 'PorSecond::PorSecond', '0x7d0950', 'por_second.cpp', 'thiscall, ret 8',
          'a segunda divisão portuguesa (a Liga de Honra): regra de disciplina 23, só a liga; o mesmo corpo',
          'the Portuguese Second Division (the Liga de Honra): discipline rule 23, the league only; the same body')])
CAMPOS = [('liga +0x50 regra de disciplina; valor; grava', 'league +0x50 discipline rule; value; writes')]
INTERESSA = []
