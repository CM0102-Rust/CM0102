# As rodadas da Copa da Polonia e da Copa da Liga polonesa e a tabela de rodadas da Copa da Liga (pol_cup.cpp,
# pol_lge_cup.cpp; leitor cx03, rodada 5, 24/11/2026). Competicoes da Polonia: LIGAS do grupo da Polonia, ate
# 2002-08-01 (a tabela da Copa da Liga roda 2 vezes no mundo ate essa data; as rodadas na criacao do jogo).
# - 0x7c8a50 PolCup::set_up_rounds thiscall() ret 0 -> 1: a virtual 0x8c da Copa da Polonia e da Copa da Turquia
#   (vtables 0x96e2c0, 0x97048c). O premio (+0x28 = 600.000) antes da virtual 0x3c (push 0; push &+0x3a; push &+0x3c;
#   push -1; mov ecx -> thiscall, ret 0x10), o retorno em +0xa3: o molde set_up_prize_rounds do cx02_4.
# - 0x7cb190 PolLgeCup::set_up_rounds: o mesmo com 400.000 e, depois da tabela, +0x49 = 7 e +0x4a = 3.
#   Prologo das duas: push esi; mov esi, ecx; push 0 (5 bytes).
# - 0x7ca7a0 PolLgeCup::create_round_table thiscall(fase, &rodadas, &extra, &saida) ret 0x10: 6 rodadas, 7 em 2001
#   (cmp word [+0x40], 0x7d1; sete), malloc 0x68 por rodada (a caixa da linha 0x9c e 0), as datas (0x521e60 e 0x521eb0,
#   cdecl, 7 e 9 pushes) e os campos de cada rodada; o dia de jogo 0 da rodada de 26/7 (0x521f10, cdecl, 7 pushes). O
#   original reusa a palavra local da contagem como indice da rodada (o numero +9 de cada rodada e o proprio indice).
#   Molde fra_super_create_single_stage_round_table (as mesmas escritas, em outra ordem: os mesmos bytes no fim).
# Usa set_up_prize_rounds e compare_set_up_prize_rounds do cx02_4 (integrar depois dele).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: as rodadas lado a lado (compare_set_up_prize_rounds: premio, rotulo, numero de rodadas, banco, retorno e
# a tabela); a tabela lado a lado (compare_cup_round_table).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('pol_cup_set_up_rounds', 0x7c8a50, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('pol_lge_cup_set_up_rounds', 0x7cb190, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('pol_lge_cup_create_round_table', 0x7ca7a0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('as rodadas da Copa da Polônia e da Copa da Liga polonesa e a tabela de rodadas da Copa da Liga',
        'the round set-up of the Polish Cup and League Cup, and the Polish League Cup round table',
        [('pol_cup_set_up_rounds', 'PolCup::set_up_rounds', '0x7c8a50', 'pol_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Polônia e a Copa da Turquia têm um prêmio total de 600.000; monta a tabela de rodadas da copa',
          'rule: the Polish Cup and the Turkish Cup have a 600,000 prize fund; builds the cup\'s round table'),
         ('pol_lge_cup_set_up_rounds', 'PolLgeCup::set_up_rounds', '0x7cb190', 'pol_lge_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Liga polonesa tem um prêmio total de 400.000, 7 reservas no banco e 3 substituições; monta a tabela de rodadas',
          'rule: the Polish League Cup has a 400,000 prize fund, 7 substitutes named and 3 substitutions; builds the round table'),
         ('pol_lge_cup_create_round_table', 'PolLgeCup::create_round_table', '0x7ca7a0', 'pol_lge_cup.cpp', 'thiscall, ret 0x10',
          'regra: a Copa da Liga polonesa tem seis rodadas de ida e volta, todas no dia da semana 2: duas de 18 clubes em julho (19/7 com os 18, 26/7 com 9 novos), as oitavas em 30/8 (7 novos), as quartas em 21/2, as semifinais em 21/3 e a final em 9/5 do ano seguinte; em 2001 uma rodada preliminar de 4 clubes em 12/7 vem antes e a primeira rodada recebe 16 novos; prêmios de 4.000/2.000 por confronto na preliminar a 120.000/60.000 na final; categoria 4 até as oitavas e 6 depois',
          'rule: the Polish League Cup has six two-legged rounds, all on weekday 2: two rounds of 18 clubs in July (19/7 with all 18, 26/7 with 9 new ones), the round of 16 on 30/8 (7 new ones), the quarter-finals on 21/2, the semi-finals on 21/3 and the final on 9/5 of the next year; in 2001 an opening round of 4 clubs on 12/7 comes first and the first round takes 16 new clubs; prizes from 4,000/2,000 per tie in the opening round to 120,000/60,000 in the final; category 4 up to the round of 16, 6 after')])
CAMPOS = []
INTERESSA = []
