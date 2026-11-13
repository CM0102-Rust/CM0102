# The schedules of the Greek First and Second Divisions and the Turkish Second Division (gre_prm.cpp, gre_second.cpp,
# tur_second.cpp). Reader mwd (12/11/2026), round 2 (2 calls each: career creation and the turnover; stage 0 of the
# Greek leagues never runs, they have no extra stage).
# - 0x5ea6d0 GrePrm::create_schedule (26 rounds; stage 0: 3 league rounds, extra 0x44c, league output 1; lines 0xa4,
#   0x13f), 0x5eb570 GreSecond::create_schedule (30 rounds; the same stage 0; lines 0xa8, 0x143), 0x8fe9b0
#   TurSecond::create_schedule (38 rounds, no stage 0; line 0xa2): fixed tables generated from the disassembly.
# Note: the out-of-memory box of the Greek stage 0 passes the extra pointer to the path splitter (it would write the
# drive into it); show_error_at does not reproduce that (only on an allocation failure).
# Conventions: thiscall, ret 0x10; 0x68a160 and 0x68a1c0 (cdecl).
# Comparers: side by side; the Greek ones compare every stage as league rounds (compare_league_stage_rounds), the
# Turkish one compare_league_round_table. Equal in cm0102-1080.exe.
# Depends on: SpaSecondScheduleRound / spa_second_set_league_rounds (spa_second.rs), written_league_round_bytes
# (fra_third.rs). fixed_league_schedule, LeagueStageRounds and compare_league_stage_rounds are reused by other mwd items.
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('gre_prm_create_schedule', 0x5ea6d0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('gre_second_create_schedule', 0x5eb570, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tur_second_create_schedule', 0x8fe9b0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os calendários das divisões gregas e da Segunda Divisão turca', 'the schedules of the Greek divisions and the Turkish Second Division',
        [('gre_prm_create_schedule', 'GrePrm::create_schedule', '0x5ea6d0', 'gre_prm.cpp', 'thiscall, ret 0x10',
          'regra: o calendário da Primeira Divisão grega: 26 rodadas aos domingos de 17 de setembro a 6 de maio (pausa de 17 de dezembro a 28 de janeiro); a fase 0, uma fase de liga de três rodadas em 13, 20 e 27 de maio (código 0x44c), que a liga nunca alcança',
          'rule: the Greek First Division schedule: 26 rounds on Sundays from 17 September to 6 May (break from 17 December to 28 January); stage 0, a three-round league stage on 13, 20 and 27 May (code 0x44c), which the league never reaches'),
         ('gre_second_create_schedule', 'GreSecond::create_schedule', '0x5eb570', 'gre_second.cpp', 'thiscall, ret 0x10',
          'regra: o calendário da Segunda Divisão grega: 30 rodadas aos sábados de 16 de setembro a 28 de abril (pausa de 16 de dezembro a 6 de janeiro); a fase 0 como a da Primeira',
          "rule: the Greek Second Division schedule: 30 rounds on Saturdays from 16 September to 28 April (break from 16 December to 6 January); stage 0 as the First Division's"),
         ('tur_second_create_schedule', 'TurSecond::create_schedule', '0x8fe9b0', 'tur_second.cpp', 'thiscall, ret 0x10',
          'regra: o calendário da Segunda Divisão turca: 38 rodadas aos domingos de 13 de agosto a 27 de maio (pausa de 17 de dezembro a 21 de janeiro); sem outra fase',
          'rule: the Turkish Second Division schedule: 38 rounds on Sundays from 13 August to 27 May (break from 17 December to 21 January); no other stage')])
CAMPOS = [('liga +0x40 ano; a tabela de rodadas (0x41 bytes)', 'league +0x40 year; the round table (0x41 bytes)')]
