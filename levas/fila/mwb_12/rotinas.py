# A Liga de Gales (wel_first.cpp): o calendário. Leitor mwb (12/11/2026), lista do mundo (Escócia e País de Gales).
# - 0x9250f0 (WelFirst::create_schedule): 34 rodadas de 19/8 a 28/4; o bloco do fim do ano (2 rodadas) pelo dia da 16a
#   rodada, no molde do calendário inglês (EngFirstChristmasSchedule e eng_first_set_christmas_rounds, já em Rust:
#   7 casos e o padrão depois da caixa da linha 0x12e).
# Convenções conferidas: thiscall, ret 0x10 (0x68a160/0x68a1c0 cdecl; 0x549f70 push -1; push ano; push 0xb; push 1; lea
# ecx -> thiscall, ret 0x10).
# Comparador: lado a lado (compare_league_round_table).
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('wel_first_create_schedule', 0x9250f0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da Liga de Gales',
        'the League of Wales schedule',
        [('wel_first_create_schedule', 'WelFirst::create_schedule', '0x9250f0', 'wel_first.cpp', 'thiscall, ret 0x10',
          'regra: 34 rodadas (18 clubes, 2 turnos) de 19/8 a 28/4 do ano seguinte, quase todas aos sábados (4/10, 21/2 e 17/4 no meio da semana); no fim do ano duas rodadas escolhidas pelo dia em que cai a 16a (de 26 a 31/12 e 2/1); só a fase principal',
          'rule: 34 rounds (18 clubs, 2 rounds) from 19/8 to 28/4 of the next year, mostly on Saturdays (4/10, 21/2 and 17/4 midweek); at the year end two rounds chosen by the day the 16th falls on (26 to 31/12 and 2/1); only the main stage')])
