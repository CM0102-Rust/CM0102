MOTOR = True
ROTINAS = [('ordem_das_partidas_do_dia', 0x6b6510, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('ordem_de_acao', 0x6bd840, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('variante_do_evento', 0x6d5d80, [0x51, 0x53, 0x56, 0x6a, 0x03])]
DOCS = ('o fechamento do motor (4): as ordens das partidas e dos jogadores e a variante do evento',
        'closing the engine (4): the match and player orderings and the event variant',
        [('ordem_das_partidas_do_dia', 'day_match_order', '0x6b6510', 'match_eng.cpp', 'cdecl',
          'a ordem das partidas do dia (qsort): competição, campos do registro e o nome do clube',
          'the day\'s match order (qsort): competition, record fields and the club name'),
         ('ordem_de_acao', 'action_order', '0x6bd840', 'match_eng.cpp', 'cdecl',
          'a ordem dos jogadores na vez do motor (qsort): os marcados primeiro, depois pela força',
          'the players\' order on the engine\'s turn (qsort): the marked ones first, then by strength'),
         ('variante_do_evento', 'event_variant', '0x6d5d80', 'match_events.cpp', 'stdcall',
          'troca o evento da narração por uma variante, por sorteio e pelo tipo do lance',
          'swaps the commentary event for a variant, by a draw and by the play type')])
