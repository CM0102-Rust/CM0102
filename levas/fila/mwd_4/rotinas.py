# The Croatian leagues: the schedules (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp). Reader mwd (12/11/2026), round 2 (A1 3
# calls, A2a 2, A2b 3: career creation, the tie-break round in May 2002 when there is a tie, the turnover).
# - 0x5140e0 CroA1::create_schedule, 0x5155e0 CroA2a::create_schedule, 0x516700 CroA2b::create_schedule: the same
#   1744-byte template (the 30 rounds generated from the disassembly and checked equal in the three); only the source
#   file changes (box lines 0xaa and 0x118 in all three).
# Conventions: thiscall, ret 0x10; 0x68a160 (9 pushes; cdecl) and 0x68a1c0 (7 pushes; cdecl); 0x521e60 (7 pushes;
# cdecl) and 0x521eb0 (9 pushes; cdecl), with add esp after each pair.
# Comparer: compare_league_round_table (side by side; the 0x41-byte rounds or the 0x68-byte cup round compared).
# Equal in cm0102-1080.exe.
# Depends on: SpaSecondScheduleRound / spa_second_set_league_rounds (spa_second.rs), FraSuperCupRound / cql12r /
# fra_super_write_cup_round (fra_super.rs), TIEBREAK_PAIRS (mwd_1).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('cro_a1_create_schedule', 0x5140e0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cro_a2a_create_schedule', 0x5155e0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cro_a2b_create_schedule', 0x516700, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
_R = '30 rodadas de 24 de julho a 6 de maio, quase todas no sábado, quatro no meio da semana (7/9, 12/10, 29/2 do ano seguinte, mesmo quando não é bissexto, como o original), pausa de inverno de meados de dezembro a meados de fevereiro, sem faixas de TV; a fase 0 é o jogo-desempate de uma rodada (sorteio 7/5, jogo 13/5, código 0xa0), com 2 times por par de desempate'
_E = '30 rounds from 24 July to 6 May, nearly all on Saturday, four midweek (7/9, 12/10, 29/2 of the next year, even when it is not a leap year, as the original), a winter break from mid-December to mid-February, no TV slots; stage 0 is the one-round tie-break play-off (drawn 7/5, played 13/5, code 0xa0), 2 teams per tie-break pair'
DOCS = ('as ligas croatas: os calendários', 'the Croatian leagues: the schedules',
        [('cro_a1_create_schedule', 'CroA1::create_schedule', '0x5140e0', 'cro_a1.cpp', 'thiscall, ret 0x10', 'regra: o calendário da Primeira Divisão croata: ' + _R, 'rule: the Croatian First Division schedule: ' + _E),
         ('cro_a2a_create_schedule', 'CroA2a::create_schedule', '0x5155e0', 'cro_a2a.cpp', 'thiscall, ret 0x10', 'regra: o calendário da Segunda Divisão croata, grupo norte: o mesmo da Primeira', 'rule: the Croatian Second Division north group schedule: the same as the First Division'),
         ('cro_a2b_create_schedule', 'CroA2b::create_schedule', '0x516700', 'cro_a2b.cpp', 'thiscall, ret 0x10', 'regra: o calendário da Segunda Divisão croata, grupo sul: o mesmo da Primeira', 'rule: the Croatian Second Division south group schedule: the same as the First Division')])
CAMPOS = [('liga +0x40 ano, +0xee pares de desempate; a tabela de rodadas (0x41) e a rodada de copa (0x68)', 'league +0x40 year, +0xee tie-break pairs; the round table (0x41) and the cup round (0x68)')]
