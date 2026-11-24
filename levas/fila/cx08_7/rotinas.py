# O calendario da Segunda Divisao argentina (arg_second.cpp; leitor cx08, rodada 5, 24/11/2026). Simulacao com a
# Argentina (as ligas do cx01_8, ate 2002-08-01): a fase -1 na criacao do jogo (set_up_rules), as zonas no fim da
# primeira fase (create_zone_stages, dezembro de 2001), a copa de acesso e as repescagens em abril e maio de 2002.
# - 0x406f00 ArgSecond::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10 (virtual 0x3c): fase -1, 25
#   rodadas de liga de 7/ago a 18/dez (extra nao escrito; caixa 0xb1); fases < 3 (com sinal), 14 rodadas de liga de
#   2/fev a 13/abr, extra 0x3e8 + fase (caixa 0x148); fase 3, a copa de acesso em 3 rodadas de copa (0x521e60 sorteio,
#   0x521eb0 jogo, cdecl; os campos da rodada; extra 0x45a; caixa 0x1a3); fases 4 e 5, uma rodada de repescagem (extra
#   0x45b/0x45c; caixas 0x1f1 e 0x219); as outras 0. Prologo mov al, [esp+4]; sub esp, 0x200 (10 bytes). As tabelas de
#   liga extraidas por script; as rodadas de copa conferidas campo a campo.
# Comparador: compare_staged_schedule do cx08_2 (lado a lado; rodadas de copa nas fases 3 a 5).
# Depende do cx08_2 (compare_staged_schedule, ScheduleTable): integrar depois.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('arg_second_create_schedule', 0x406f00, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário da Segunda Divisão argentina', 'the Argentine Second Division schedule',
        [('arg_second_create_schedule', 'ArgSecond::create_schedule', '0x406f00', 'arg_second.cpp', 'thiscall, ret 0x10',
          'monta o calendário da Segunda Divisão argentina: a primeira fase em 25 rodadas de 7 de agosto a 18 de dezembro; as três zonas em 14 rodadas de 2 de fevereiro a 13 de abril do ano seguinte; a copa de acesso em quartas, semifinais e final de ida e volta em abril e maio; cada repescagem de acesso num confronto de ida e volta a partir de 15 de maio',
          'builds the Argentine Second Division schedule: the first phase in 25 rounds from 7 August to 18 December; the three zones in 14 rounds from 2 February to 13 April of the next year; the promotion cup in two-legged quarter-finals, semi-finals and final in April and May; each promotion play-off in one two-legged tie from 15 May')])
CAMPOS = []
INTERESSA = []
