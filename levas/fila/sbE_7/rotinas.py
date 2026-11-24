# Os calendarios da Primeira e da Segunda Divisao polonesas (pol_first.cpp, pol_second.cpp; leitor sbE, rodada 5,
# 24/11/2026). Simulacao com a Polonia (as ligas e a data dos itens mwd): os calendarios na criacao do jogo (fase -1
# pelo set_up_rules e fase 0 pelo create_second_group_stage), a fase final em novembro de 2001, a repescagem em junho
# de 2002 e a virada.
# - 0x7c8df0 PolFirst::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10 (virtual 0x3c): fase (byte com
#   sinal) < 1, 14 rodadas de 22/jul a 4/nov, extra 0x420 + fase (caixa 0xb2); 1, 14 rodadas de 17/nov a 18/mai do ano
#   seguinte, extra 0 (caixa 0xf1); > 1, 0 sem escrever. Datas 0x68a160 e faixas 0x68a1c0 (cdecl, intercaladas), malloc
#   0x944e46. Prologo mov al, [esp+4]; sub esp, 0x200 (10 bytes).
# - 0x7cb4a0 PolSecond::create_schedule: -1, 38 rodadas em 2001 (+0x40 = 0x7d1) e 34 depois, de 29/jul a 27/mai (extra 0,
#   caixa 0xa8; a caixa 0x153 de contagem errada nunca e alcancada); 0, a repescagem (e_liga 0, 1 rodada de copa de 0x68
#   bytes, caixa 0x165): sorteio 0x521e60 (28/mai, +1 ano, dia 6), jogo 0x521eb0 (3/jun, +1 ano, dia 5, codigo 1),
#   rodada 0xa0, desempate 0x200 e 3, categoria 5, 4 times, 2 confrontos, 4 novos, ida e volta (2), 4 dias, premios 0;
#   outra fase 0. Prologo mov al, [esp+4]; sub esp, 0x200 (10 bytes).
# Tabelas tiradas executando a original no unicorn (emula.py do cx14, com o ano 2002 para as 34 rodadas).
# Depende do cx08_2 (compare_staged_schedule, ScheduleTable): integrar depois dele.
# Iguais no cm0102-1080.exe (inteiras). Comparadores: lado a lado (compare_staged_schedule; a repescagem como copa).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('pol_first_create_schedule', 0x7c8df0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('pol_second_create_schedule', 0x7cb4a0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os calendários da Primeira e da Segunda Divisão polonesas', 'the Polish First and Second Division schedules',
        [('pol_first_create_schedule', 'PolFirst::create_schedule', '0x7c8df0', 'pol_first.cpp', 'thiscall, ret 0x10',
          'cada fase de grupos (os dois grupos de oito) em 14 rodadas semanais de 22 de julho a 4 de novembro; a fase final (grupos do título e do rebaixamento) em 14 rodadas de 17 de novembro a 18 de maio do ano seguinte, uma no meio da semana; sem faixas de TV',
          'each group stage (the two groups of eight) in 14 weekly rounds from 22 July to 4 November; the final stage (the championship and relegation groups) in 14 rounds from 17 November to 18 May of the next year, one midweek; no TV slots'),
         ('pol_second_create_schedule', 'PolSecond::create_schedule', '0x7cb4a0', 'pol_second.cpp', 'thiscall, ret 0x10',
          'a temporada em 34 rodadas de 29 de julho a 27 de maio do ano seguinte (38 em 2001, com quatro rodadas a mais no meio da semana); a repescagem de acesso (fase 0) em uma rodada de ida e volta no começo de junho',
          'the season in 34 rounds from 29 July to 27 May of the next year (38 in 2001, with four more midweek rounds); the promotion play-off (stage 0) in one two-legged round at the start of June')])
CAMPOS = []
INTERESSA = []
