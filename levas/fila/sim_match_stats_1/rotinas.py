ROTINAS = [('estatistica_da_liga_do_jogador', 0x7c47b0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('entrada_do_jogo_bate', 0x729500, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('balde_do_registro', 0x729bd0, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c])]
DOCS = ('as estatísticas da partida: a estatística de liga do jogador, a entrada do jogo e o balde do registro',
        'the match statistics: the player league stat, the fixture entry and the record bucket',
        [('estatistica_da_liga_do_jogador', 'PlayerStats::get_club_league_stat', '0x7c47b0', 'player_stats.cpp', 'thiscall',
          'as partidas (campo 1) ou os gols (campo 2) de liga do jogador no clube, como número de ponto flutuante',
          "the player's league appearances (field 1) or league goals (field 2) at the club, as a float"),
         ('entrada_do_jogo_bate', 'Match_Stats::fixture_entry_matches', '0x729500', 'match_screens.cpp', 'thiscall',
          'diz se uma entrada das estatísticas tem os dois clubes, em qualquer ordem, e as datas dadas',
          'reports whether a match statistics entry holds the two given clubs in either order and the given date'),
         ('balde_do_registro', 'match_stats_get_record_bucket', '0x729bd0', 'match_screens.cpp', 'stdcall',
          'o balde (0 a 3) em que o registro de estatística do jogo é guardado: tipo da competição, seleção e marcas do jogo',
          'the bucket (0 to 3) where the match statistics record is stored: competition type, national team and fixture flags')])
