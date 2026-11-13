# The schedules of the Liga de Honra, the Superettan and the Ykkonen (reader mwe, whole-world round, 12/11/2026).
# The slot +0x3c of the vtable (create_schedule); each runs from its league's set_up_rules (creation and turnover) and,
# for the play-off stages, from the play-off routines (the Superettan play-off in November 2001; the Ykkonen play-offs
# in September/October 2001):
# - 0x7d0c40 PorSecond::create_schedule (2272 bytes): 34 rounds, linear;
# - 0x88e8e0 SweFirst::create_schedule (1712 bytes): 30 league rounds (stage -1) and the one-round play-off (stage 0);
# - 0x5931c0 FinFirst::create_schedule (2224 bytes): 27 group rounds (stages -1 and 0, codes 0x41d/0x41e) and the
#   play-off stages 1 (two rounds), 2 and 3 (one round each).
# The league tables were extracted from the disassembly (capstone, the pushes of each league_round_set_date and
# league_round_set_match_slot followed) and checked; the cup rounds were read by hand: the draw and match dates
# (0x521e60, 0x521eb0) and every round field the original writes (+7 code, +9 index, +0xb, +0xd, +0xf rules, +0x17
# category, +0x18/+0x1a/+0x1c/+0x1e counts, +0x20/+0x21/+0x22 legs, +0x5c/+0x60/+0x64 prizes) - the same set as
# eng_cc_cup_write_cup_round (cql_10). Equal in cm0102-1080.exe (whole).
# Conventions checked: the callers (the virtual +0x3c: push 0 or &league; push &extra; push &rounds; push stage; mov
# ecx) -> thiscall, ret 0x10; malloc, 0x68a160, 0x68a1c0, 0x521e60 (7 pushes) and 0x521eb0 (9 pushes): cdecl.
# The league rounds get the year as a register with the word of +0x40 (garbage above; Date::Date reads the word);
# the cup rounds get it sign-extended (movsx), as the copy passes it.
# Comparers: compare_league_round_table (side by side) for the first two; FinFirst its own (the league bytes at both
# group stages, the cup round bytes at stages 1-3).
# Determinism: only exe constants and the league's year.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('por_second_create_schedule', 0x7d0c40, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('swe_first_create_schedule', 0x88e8e0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('fin_first_create_schedule', 0x5931c0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os calendários da Liga de Honra, da Superettan e da Ykkönen',
        'the schedules of the Liga de Honra, the Superettan and the Ykkönen',
        [('por_second_create_schedule', 'PorSecond::create_schedule', '0x7d0c40', 'por_second.cpp', 'thiscall, ret 0x10',
          'o calendário da Liga de Honra: 34 rodadas (18 clubes), as datas da primeira divisão (22 de agosto a 14 de maio), com um jogo da maior parte das rodadas na faixa de domingo [inferência: o televisionado]; só a fase principal',
          'the Liga de Honra schedule: 34 rounds (18 clubs), the Premier Division dates (22 August to 14 May), with one match of most rounds in the Sunday slot [inference: the televised one]; main stage only'),
         ('swe_first_create_schedule', 'SweFirst::create_schedule', '0x88e8e0', 'swe_first.cpp', 'thiscall, ret 0x10',
          'o calendário da Superettan: 30 rodadas (16 clubes), de 6 de abril a 8 de novembro, as datas da Allsvenskan e quatro rodadas numa quarta; o play-off de acesso (fase 0) é um confronto de ida e volta a partir de 15 de novembro (sorteio em 9 de novembro), com 7 dias entre os jogos',
          'the Superettan schedule: 30 rounds (16 clubs), from 6 April to 8 November, the Allsvenskan dates and four Wednesday rounds; the promotion play-off (stage 0) is a two-legged tie from 15 November (draw on 9 November), 7 days apart'),
         ('fin_first_create_schedule', 'FinFirst::create_schedule', '0x5931c0', 'fin_first.cpp', 'thiscall, ret 0x10',
          'o calendário da Ykkönen: os dois grupos (Norte e Sul) têm as mesmas 27 rodadas (três turnos de 10 clubes), de 29 de abril a 6 de outubro; o play-off de acesso tem semifinais de ida e volta a partir de 5 de outubro e a final a partir de 12 de outubro; o de rebaixamento, dois confrontos a partir de 5 de outubro; o play-off contra a Veikkausliiga, um confronto a partir de 19 de outubro',
          'the Ykkönen schedule: the two groups (North and South) have the same 27 rounds (three legs of 10 clubs), from 29 April to 6 October; the promotion play-off has two-legged semi-finals from 5 October and the final from 12 October; the relegation play-off, two ties from 5 October; the play-off against the Veikkausliiga, one tie from 19 October')])
CAMPOS = []
INTERESSA = []
