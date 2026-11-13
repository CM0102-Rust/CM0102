# The schedules of the Allsvenskan, the Veikkausliiga and the Portuguese Premier Division (reader mwe, whole-world
# round, 12/11/2026). The same slot (+0x3c of the vtable, create_schedule) in three leagues: the round table of the
# main stage (0x41 bytes per round: the date and the TV slots). Each runs from its league's set_up_rules (at the
# creation of the game and at each turnover: the end of 2001 for Sweden and Finland, the summer of 2002 for
# Portugal):
# - 0x88feb0 SwePrm::create_schedule (1408 bytes, 26 rounds);
# - 0x5954b0 FinPrm::create_schedule (1728 bytes, 33 rounds);
# - 0x7ce9e0 PorPrm::create_schedule (3616 bytes, 34 rounds).
# The body of eng_first_set_league_rounds (cqp_5): the calls are linear (no branch besides the stage and the box);
# the tables were extracted from the disassembly with a capstone reader (each league_round_set_date and
# league_round_set_match_slot call with its pushed constants, the registers followed) and checked: the rounds in
# order, the slots of each round in sequence (j = 0, 1, ...) closed by (-1, -1, -1), code 2 for the midweek rounds.
# Equal in cm0102-1080.exe (whole).
# Conventions checked: the callers (the virtual +0x3c in set_up_rules: push 0; push &+0x3a; push &+0xa9; push -1;
# mov ecx) -> thiscall, ret 0x10 (the `ret 0x10` of the three); malloc (cdecl); 0x68a160 (9 pushes; add esp ->
# cdecl); 0x68a1c0 (7 pushes; add esp -> cdecl); the error box.
# The year pushed is a register with only the low word loaded from +0x40 (garbage above); 0x68a160 passes it to
# Date::Date, which reads the word: the copy passes the year clean.
# Comparer: compare_league_round_table (side by side: the original with the caller's outputs, the copy with its own;
# the outputs and the written bytes of both tables equal; the copy's table freed).
# Determinism: only exe constants and the league's year.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('swe_prm_create_schedule', 0x88feb0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('fin_prm_create_schedule', 0x5954b0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('por_prm_create_schedule', 0x7ce9e0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os calendários da Allsvenskan, da Veikkausliiga e da primeira divisão portuguesa',
        'the schedules of the Allsvenskan, the Veikkausliiga and the Portuguese Premier Division',
        [('swe_prm_create_schedule', 'SwePrm::create_schedule', '0x88feb0', 'swe_prm.cpp', 'thiscall, ret 0x10',
          'o calendário da Allsvenskan: 26 rodadas (14 clubes, turno e returno), de 6 de abril a 8 de novembro, aos domingos, com uma rodada numa terça de junho e a penúltima num sábado; sem faixas de TV; só a fase principal',
          'the Allsvenskan schedule: 26 rounds (14 clubs, two legs), from 6 April to 8 November, on Sundays, with one round on a June Tuesday and the second last on a Saturday; no TV slots; main stage only'),
         ('fin_prm_create_schedule', 'FinPrm::create_schedule', '0x5954b0', 'fin_prm.cpp', 'thiscall, ret 0x10',
          'o calendário da Veikkausliiga: 33 rodadas (12 clubes, três turnos), de 29 de abril a 28 de outubro, aos domingos, quartas e segundas; sem faixas de TV; só a fase principal',
          'the Veikkausliiga schedule: 33 rounds (12 clubs, three legs), from 29 April to 28 October, on Sundays, Wednesdays and Mondays; no TV slots; main stage only'),
         ('por_prm_create_schedule', 'PorPrm::create_schedule', '0x7ce9e0', 'por_prm.cpp', 'thiscall, ret 0x10',
          'o calendário da primeira divisão portuguesa: 34 rodadas (18 clubes), de 22 de agosto a 14 de maio do ano seguinte, aos domingos, com três jogos levados para sexta, sábado e segunda na maior parte das rodadas [inferência: os televisionados]; só a fase principal',
          'the Portuguese Premier Division schedule: 34 rounds (18 clubs), from 22 August to 14 May of the next year, on Sundays, with three matches moved to Friday, Saturday and Monday in most rounds [inference: the televised ones]; main stage only')])
CAMPOS = []
INTERESSA = []
