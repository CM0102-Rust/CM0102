ROTINAS = [('ultima_nota_do_jogador', 0x7c1a60, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('vaga_da_busca', 0x7ae640, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tipo_da_lista_da_busca', 0x7ae6e0, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a última nota do jogador e a vaga e o tipo da busca de jogadores',
        'the player\'s last rating and the player search slot and list type',
        [('ultima_nota_do_jogador', 'PlayerStats::get_last_match_rating', '0x7c1a60', 'player_stats.cpp', 'thiscall',
          'a nota da última partida do jogador (6,5 sem nota)',
          'the player\'s last match rating (6.5 with none)'),
         ('vaga_da_busca', 'PLAYER_SEARCH::set_slot_index', '0x7ae640', 'player_search.cpp', 'thiscall',
          'grava a vaga (0 a 9) da busca de jogadores',
          'stores the player search slot (0 to 9)'),
         ('tipo_da_lista_da_busca', 'PLAYER_SEARCH::set_list_type', '0x7ae6e0', 'player_search.cpp', 'thiscall',
          'grava o tipo de lista (1 a 4) da busca de jogadores',
          'stores the player search list type (1 to 4)')])
