# The constructors of the K League and of the Korean League Cup (kor_league.cpp, kor_league_cup.cpp). Reader mwf
# (19/11/2026), whole-world round, continuation: each runs once, at career creation (docs/cobertura_mundo.txt).
# - 0x66ae70 KorLeague::KorLeague: construct_nation_division (batch 299, GerFirst), checked instruction by instruction:
#   only the discipline rule 42, the vtable 0x96cdd0, KorLeague::set_up_rules 0x66bcc0 and ::create_table 0x66bba0 and
#   the file change (the same five box lines).
# - 0x66bf00 KorLeagueCup::KorLeagueCup: the ItaCCup/IreLgeCup body (construct_irish_league_cup, mwa_16), checked
#   instruction by instruction; it differs in the flags (+0xd9 = 8, not 0x28), the box lines (0x53, 0x5d, 0x64, 0x6a,
#   0x71) and the end (one call to create_group_b_stage(0) after a good sort instead of the loop over the groups):
#   written as its own body, with the fixed-fields comparer of the Italian cups (ita_cups_check_constructor_plan).
# Conventions: thiscall, ret 8 (0x682200 League::League thiscall ret 0; 0x687b10 push 1 -> thiscall ret 4; the routines
# called thiscall ret 0; 0x66cb60 push 0 -> thiscall ret 4; 0x6835c0 thiscall ret 0; 0x6827d0 push 0 -> thiscall ret 4).
# Comparers: only the original runs (constructors: they allocate, draw and read the save file), the plan checked.
# No mutation: the comparers run only the original, the K League body is the shared template (already mutated in batch
# 299) and the League Cup's own writes (flags, stage count) have no effect the career sees for sure without risking a
# crash (a missing stage).
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_kor_league', 0x66ae70, [0x6a, 0xff, 0x68, 0xd9, 0xc6, 0x95, 0x00]),
           ('construct_kor_league_cup', 0x66bf00, [0x6a, 0xff, 0x68, 0x29, 0xc7, 0x95, 0x00])]
DOCS = ('os construtores da K League e da Copa da Liga coreana', 'the constructors of the K League and the Korean League Cup',
        [('construct_kor_league', 'KorLeague::KorLeague', '0x66ae70', 'kor_league.cpp', 'thiscall, ret 8',
          'regra: a K League ao criar a carreira: a regra de disciplina 42, só a liga (sem fases); sem estado gravado, as regras, a tabela com os dez clubes, os jogos, a ordem, as estatísticas e as posições previstas',
          'rule: the K League at career creation: discipline rule 42, only the league (no stages); with no saved state, the rules, the table with its ten clubs, the fixtures, the order, the statistics and the predicted positions'),
         ('construct_kor_league_cup', 'KorLeagueCup::KorLeagueCup', '0x66bf00', 'kor_league_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Liga coreana ao criar a carreira: 5 reservas e 3 substituições, a regra de disciplina 42, importância 12; sem estado gravado, duas fases (o grupo B e o mata-mata), os dez clubes, as estatísticas, as posições previstas, as regras, a tabela do grupo A (a própria copa), os jogos e a ordem dele e a fase do grupo B',
          'rule: the Korean League Cup at career creation: 5 substitutes and 3 substitutions, discipline rule 42, importance 12; with no saved state, two stages (group B and the knockout), the ten clubs, the statistics, the predicted positions, the rules, group A table (the cup itself), its fixtures and order and group B stage')])
CAMPOS = [('liga +0xd9 (flags), +0x2c e +0xc (fases), +0x49, +0x4a (banco), +0x50 (disciplina), +0x51 (importância)',
           'league +0xd9 (flags), +0x2c and +0xc (stages), +0x49, +0x4a (bench), +0x50 (discipline), +0x51 (importance)')]
