# O calendario da Segunda Divisao belga (bel_second.cpp; leitor cx08, rodada 5, 24/11/2026). Simulacao com a Belgica (as
# ligas do cx02_21: 'ENG BEL HOL GER DEN', ate 2002-08-01): a fase -1 na criacao do jogo (set_up_rules), o grupo da
# repescagem (fase 3) em maio de 2002 (create_playoff_group, cx08_16) e a copa da repescagem (fase 4) se for criada.
# - 0x41f340 BelSecond::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10 (virtual 0x3c): fase -1, 34
#   rodadas de liga de 23/ago a 6/mai, extra 0 (caixa 0xb1); fase 3, 6 rodadas de liga de 13 a 31/mai, extra 0xa0 (caixa
#   0x12b); fase 4, 3 rodadas de copa em jogo unico em maio (0x521e60, 0x521eb0 com 0x28 no ultimo argumento; extra 0x452;
#   caixa 0x151); as outras 0 sem escrever. Prologo mov al, [esp+4]; sub esp, 0x200 (10 bytes). As tabelas de liga
#   extraidas por script; as rodadas de copa conferidas campo a campo.
# Comparador: compare_staged_schedule do cx08_2 (lado a lado; rodadas de copa na fase 4).
# Depende do cx08_2 (compare_staged_schedule, ScheduleTable): integrar depois.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_second_create_schedule', 0x41f340, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da Segunda Divisão belga', 'the Belgian Second Division schedule',
        [('bel_second_create_schedule', 'BelSecond::create_schedule', '0x41f340', 'bel_second.cpp', 'thiscall, ret 0x10',
          'monta o calendário da Segunda Divisão belga: a temporada em 34 rodadas de 23 de agosto a 6 de maio do ano seguinte, sem faixas de TV; o grupo da repescagem de acesso em 6 rodadas de 13 a 31 de maio; a copa da repescagem em três rodadas de jogo único em maio',
          'builds the Belgian Second Division schedule: the season in 34 rounds from 23 August to 6 May of the next year, with no TV slots; the promotion play-off group in 6 rounds from 13 to 31 May; the play-off cup in three single-leg rounds in May')])
CAMPOS = []
INTERESSA = []
