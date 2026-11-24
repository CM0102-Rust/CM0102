# As regras da minliga e o construtor de uma vaga de torneio dos amistosos (mini_league.cpp, friendly.cpp; leitor cx05,
# rodada 5, 24/11/2026). Simulacao, carreira ENG (sem LIGAS).
# - 0x766db0 MiniLeague::set_up_rules thiscall() ret 0 (virtual +0x8c, so pela vtable 0x96d94c): o corpo das regras do
#   construtor (mini_league_rules do cx05_13). Duvida para a prova: nenhum chamador direto; a virtual +0x8c e chamada
#   pela virada das ligas, que a minliga nao faz; pode nao rodar na carreira (entao o item fica so com a vaga).
#   Prologo push esi; mov esi, ecx; xor eax, eax (5 bytes).
# - 0x5c22c0 friendly_tournament_slot_construct thiscall() ret 0 -> this: Date::Date 0x54a110 em +4 (lea ecx ->
#   thiscall). Chamada pelo vector constructor iterator do CRT nas 127 vagas, no construtor dos amistosos (0x5c1f80,
#   na criacao do jogo). Prologo push esi; mov esi, ecx; lea ecx, [esi+4] (6 bytes).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Depende do cx05_13 (mini_league_rules).
# Comparadores: as regras so a original (o calendario aloca), com o plano; a vaga lado a lado nos 16 primeiros bytes.
SAVE = False
ROTINAS = [('mini_league_set_up_rules', 0x766db0, [0x56, 0x8b, 0xf1, 0x33, 0xc0]),
           ('friendly_tournament_slot_construct', 0x5c22c0, [0x56, 0x8b, 0xf1, 0x8d, 0x4e, 0x04])]
DOCS = ('as regras da minliga e a vaga de torneio dos amistosos', 'the mini league rules and the friendly tournament slot',
        [('mini_league_set_up_rules', 'MiniLeague::set_up_rules', '0x766db0', 'mini_league.cpp', 'thiscall, ret 0',
          'regra: a minliga joga em turno único, com 9 reservas e 9 substituições, sem acesso; monta de novo o calendário',
          'rule: the mini league is a single round robin with 9 substitutes named and 9 allowed, no promotion; it builds the round table again'),
         ('friendly_tournament_slot_construct', 'friendly_tournament_slot_construct', '0x5c22c0', 'friendly.cpp', 'thiscall, ret 0',
          'monta uma vaga de torneio dos amistosos: só a data de início',
          'constructs a friendly tournament slot: only the start date')])
CAMPOS = []
INTERESSA = []
