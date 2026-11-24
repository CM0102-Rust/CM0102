# As rodadas das supercopas e das copas de clubes da Asia, e o dia da Supercopa da Asia (spa_super.cpp,
# asia_cup_winner.cpp, asia_super_cup.cpp; leitor cx03, rodada 5, 24/11/2026). Carreira de 8 ligas (o Japao jogavel e as
# supercopas da Espanha, da Italia, da Franca e do Japao), ate 2002-08-01: a virada da Supercopa da Asia e em 25 de
# junho, fora da janela padrao de 2002-06-01.
# - 0x858e70 SpaSuper::set_up_rounds thiscall() ret 0 -> 1: a virtual 0x8c de 23 vtables (as supercopas e outras): so
#   a tabela pela virtual 0x3c em +0xa3 (build_cup_round_table_by_vtable), o premio fica. Prologo push esi; mov esi,
#   ecx; push 0.
# - 0x40d800 AsiaCupWinner::set_up_rounds: a virtual 0x8c da Recopa e da Liga dos Campeoes da Asia (vtables 0x967574,
#   0x9674d4): +0x28 = 4.000.000 e a tabela (set_up_prize_rounds, cx02_4).
# - 0x410e20 AsiaSuperCup::update_for_date thiscall(hoje, fase) ret 8: o molde update_cup_with_turnover_on do cx03_7 com
#   Date(0x19, 5, ano de hoje, -1) (25 de junho; conferido instrucao a instrucao contra o RusCup).
# Usa set_up_prize_rounds e compare_set_up_prize_rounds do cx02_4 e update_cup_with_turnover_on do cx03_7 (integrar
# depois deles).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: as rodadas lado a lado; o dia so a original, com o plano (o ano sobe 1 quando a virada era devida).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('spa_super_set_up_rounds', 0x858e70, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('asia_cup_winner_set_up_rounds', 0x40d800, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('asia_super_cup_update_for_date', 0x410e20, [0x83, 0xec, 0x10, 0x53, 0x8b, 0x5c, 0x24, 0x1c])]
DOCS = ('as rodadas das supercopas e das copas de clubes da Ásia, e o dia da Supercopa da Ásia',
        "the round set-up of the super cups and of the Asian club cups, and the Asian Super Cup's day",
        [('spa_super_set_up_rounds', 'SpaSuper::set_up_rounds', '0x858e70', 'spa_super.cpp', 'thiscall, ret 0',
          'regra: a Supercopa da Espanha e outras 22 copas que dividem a virtual (quase todas supercopas) só montam a tabela de rodadas, sem mexer no prêmio',
          'rule: the Spanish Super Cup and 22 other cups that share the virtual (mostly super cups) only build the round table, leaving the prize fund alone'),
         ('asia_cup_winner_set_up_rounds', 'AsiaCupWinner::set_up_rounds', '0x40d800', 'asia_cup_winner.cpp', 'thiscall, ret 0',
          'regra: a Recopa da Ásia e a Liga dos Campeões da Ásia têm um prêmio total de 4.000.000; monta a tabela de rodadas',
          "rule: the Asian Cup Winners' Cup and the Asian Club Championship have a 4,000,000 prize fund; builds the round table"),
         ('asia_super_cup_update_for_date', 'AsiaSuperCup::update_for_date', '0x410e20', 'asia_super_cup.cpp', 'thiscall, ret 8',
          'regra: a Supercopa da Ásia começa a edição seguinte em 25 de junho, quando a atual terminou; depois, todo dia, o dia da copa',
          "rule: the Asian Super Cup starts its next edition on 25 June once the current one is over; then, every day, the cup's day")])
CAMPOS = []
INTERESSA = []
