# A tabela, as regras e a virada da Segunda Divisao argentina (Nacional B) (arg_second.cpp; leitor cx08, rodada 5,
# 24/11/2026). Simulacao com a Argentina (as ligas do cx01_8, ate 2002-08-01): a tabela e as regras rodam na criacao do
# jogo (o construtor do mwf_23) e de novo na virada de 2002, chamada pela da Primeira (cx08_4).
# - 0x407cd0 ArgSecond::fill_league_table thiscall() ret 0: 25 clubes, malloc 0x5c3 (caixa 0x247); os clubes com
#   +0x57 e +0x60 nao nulos e divisao [0x9cf658] (init_table_row 0x687430); com 24, o clube fixo ([0x9d020c], Huracan de
#   Tres Arroyos) que nao estiver na divisao ganha +0x57 = +4 e a linha 24; cheia: +0x28 = 0, +0xe1 = 2 e 1. Prologo sub
#   esp, 0x200.
# - 0x407e50 ArgSecond::set_up_rules thiscall() ret 0 -> 1: as gravacoes na ordem da original (1 turno, 1 sobe direto,
#   acima a Primeira, abaixo -1, bits 3) e a virtual 0x3c em +0xba. Prologo push ebx; push esi; mov esi, ecx; mov ebx, 1.
# - 0x407f00 ArgSecond::start_new_season thiscall() ret 0: o molde do bel_second_start_new_season (rebuild_season com
#   0x407cd0 e as caixas 0x2ba, 0x2c1, 0x2c7, 0x2ce; depois set_expectations_and_renew_officials), conferido instrucao a
#   instrucao. Prologo sub esp, 0x200.
# Comparadores: a tabela so a original contra o plano; as regras a seco (compare_nation_league_rules); a virada pelo
# ano + 1.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('arg_second_fill_league_table', 0x407cd0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('arg_second_set_up_rules', 0x407e50, [0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00]),
           ('arg_second_start_new_season', 0x407f00, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela, as regras e a virada da Segunda Divisão argentina', 'the Argentine Second Division table, rules and turnover',
        [('arg_second_fill_league_table', 'ArgSecond::fill_league_table', '0x407cd0', 'arg_second.cpp', 'thiscall, ret 0',
          'a Segunda Divisão argentina (Nacional B) tem 25 clubes; achando só 24, o Huracán de Tres Arroyos entra na divisão como o 25º; sem fundo de prêmios',
          'the Argentine Second Division (Nacional B) has 25 clubs; when only 24 are found, Huracán de Tres Arroyos is put into the division as the 25th; no prize fund'),
         ('arg_second_set_up_rules', 'ArgSecond::set_up_rules', '0x407e50', 'arg_second.cpp', 'thiscall, ret 0',
          'as regras da Segunda Divisão argentina: turno único, 3 pontos por vitória e 1 por empate, um sobe direto, sem repescagem nem rebaixamento direto (vai pelos promédios), a Primeira acima, desempates 1, 2 e 3, 5 reservas e 3 substituições; depois o calendário',
          'the Argentine Second Division rules: a single round, 3 points for a win and 1 for a draw, one promoted directly, no play-off nor direct relegation (it goes by the averages), the Premier Division above, tie-breakers 1, 2 and 3, 5 substitutes and 3 substitutions; then the schedule'),
         ('arg_second_start_new_season', 'ArgSecond::start_new_season', '0x407f00', 'arg_second.cpp', 'thiscall, ret 0',
          'vira a temporada da Segunda Divisão argentina (chamada pela virada da Primeira): a temporada reaberta e refeita (regras, tabela, jogos, ordem, previsões), as expectativas da diretoria e os árbitros',
          "turns over the Argentine Second Division season (called by the Premier Division's): the season reopened and rebuilt (rules, table, fixtures, order, predictions), the board expectations and the officials")])
CAMPOS = []
INTERESSA = []
