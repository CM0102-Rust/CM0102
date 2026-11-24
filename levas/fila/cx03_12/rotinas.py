# As rodadas da Taca de Portugal, o dia da Copa da Suecia e o progresso na Copa da Suecia (por_cup.cpp, swe_cup.cpp;
# leitor cx03, rodada 5, 24/11/2026). Competicoes de Portugal e da Suecia: LIGAS do grupo ENG SWE FIN NOR POR, ate
# 2002-08-01 (o dia da Copa da Suecia roda 2308 vezes no mundo ate essa data; a tabela de rodadas 17).
# - 0x7ce6f0 PorCup::set_up_rounds thiscall() ret 0 -> 1: a virtual 0x8c da Taca de Portugal e de mais quatro copas
#   (vtables 0x9678a0 BelFaCup, 0x96dd80, 0x96f124 RusCup, 0x970044 SweCup): +0x28 = 800.000 e a tabela pela virtual
#   0x3c em +0xa3 (set_up_prize_rounds, cx02_4). Prologo push esi; mov esi, ecx; push 0.
# - 0x88d8a0 SweCup::update_for_date thiscall(hoje, fase) ret 8: o molde update_cup_with_turnover_on do cx03_7 com a
#   data Date(1, 6, ano de hoje, -1) (1 de julho; conferido instrucao a instrucao contra o RusCup).
# - 0x88e580 SweCup::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: Comp_Stats::find_club_entry
#   (push clube; mov ecx, [this+8] -> thiscall, ret 4); fase < 7: cada codigo n vira 8n - 7 (byte: shl 3; sub 7);
#   +0x73 atual, +0x74 melhor, +0x75 pior.
# Usa set_up_prize_rounds, compare_set_up_prize_rounds e compare_club_progress do cx02_4, update_cup_with_turnover_on do
# cx03_7 e store_club_progress do cx03_9 (integrar depois deles).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: as rodadas e o progresso lado a lado; o dia so a original (a virada libera, aloca e sorteia), com o
# plano: o ano da copa sobe 1 quando a virada era devida.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('por_cup_set_up_rounds', 0x7ce6f0, [0x56, 0x8b, 0xf1, 0x6a, 0x00]),
           ('swe_cup_update_for_date', 0x88d8a0, [0x83, 0xec, 0x10, 0x53, 0x8b, 0x5c, 0x24, 0x1c]),
           ('swe_cup_set_club_progress', 0x88e580, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('as rodadas da Taça de Portugal, o dia da Copa da Suécia e o progresso na Copa da Suécia',
        "the Portuguese Cup round set-up, the Swedish Cup's day and the Swedish Cup progress",
        [('por_cup_set_up_rounds', 'PorCup::set_up_rounds', '0x7ce6f0', 'por_cup.cpp', 'thiscall, ret 0',
          'regra: a Taça de Portugal, as Copas da Bélgica, da Rússia e da Suécia e mais uma copa que divide a virtual têm um prêmio total de 800.000; monta a tabela de rodadas',
          'rule: the Portuguese Cup, the Belgian, Russian and Swedish Cups and one more cup that shares the virtual have an 800,000 prize fund; builds the round table'),
         ('swe_cup_update_for_date', 'SweCup::update_for_date', '0x88d8a0', 'swe_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Suécia começa a edição seguinte em 1º de julho, quando a atual terminou; depois, todo dia, o dia da copa (sorteios e jogos)',
          "rule: the Swedish Cup starts its next edition on 1 July once the current one is over; then, every day, the cup's day (draws and matches)"),
         ('swe_cup_set_club_progress', 'SweCup::set_club_progress', '0x88e580', 'swe_cup.cpp', 'thiscall, ret 0x14',
          'regra: nas sete seções preliminares da Copa da Suécia (fases abaixo de 7) cada código de progresso n vira 8n − 7, o lugar da seção na escala da chave principal de 64 clubes [inferência]; na chave principal os códigos ficam como vêm',
          "rule: in the Swedish Cup's seven preliminary sections (stages below 7) each progress code n becomes 8n − 7, the section's place on the scale of the 64-club main draw [inference]; in the main draw the codes stay as they come")])
CAMPOS = []
INTERESSA = []
