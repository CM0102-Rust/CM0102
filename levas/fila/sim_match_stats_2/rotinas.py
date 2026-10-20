ROTINAS = [('preencher_registro_da_partida', 0x729ff0, [0xa0, 0xc4, 0xb7, 0xb6, 0x00]),
           ('ordem_dos_eventos', 0x72b160, [0x8b, 0x54, 0x24, 0x08, 0x56]),
           ('tirar_idioma_do_jogador', 0x7c6bf0, [0x83, 0xec, 0x08, 0x56, 0x8b, 0xf1])]
DOCS = ('o registro da partida, a ordem dos eventos e o idioma que o jogador perde',
        'the match record, the order of the events and the language a player loses',
        [('preencher_registro_da_partida', 'Match_Stats::fill_match_record', '0x729ff0', 'match_screens.cpp', 'thiscall',
          'preenche o registro de estatística da partida: clubes, datas, público, renda, clima, eventos ordenados, formação, capitão e os 20 jogadores de cada time',
          'fills the match statistics record: clubs, dates, attendance, gate receipts, weather, sorted events, formation, captain and the 20 players of each side'),
         ('ordem_dos_eventos', 'match_stats_compare_events', '0x72b160', 'match_screens.cpp', 'cdecl',
          'ordena dois eventos da partida pelo grupo do tipo, depois pelo minuto e pela ordem de chegada (comparador do qsort)',
          'orders two match events by type group, then by minute and arrival order (qsort comparator)'),
         ('tirar_idioma_do_jogador', 'PlayerStats::remove_language', '0x7c6bf0', 'player_stats.cpp', 'thiscall',
          'tira um idioma do jogador da tabela de idiomas (busca binária) e desce as entradas seguintes',
          "removes a language from the player's language table (binary search) and shifts the later entries down")])
