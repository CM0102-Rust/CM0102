MOTOR = True
ROTINAS = [('valores_das_estatisticas_da_partida', 0x71e750, [0x6a, 0xff, 0x68, 0x5b, 0xd5, 0x95, 0x00]),
           ('valores_das_zonas_de_acao', 0x720320, [0x6a, 0xff, 0x68, 0x7b, 0xd5, 0x95, 0x00]),
           ('valores_das_notas_dos_jogadores', 0x723350, [0x6a, 0xff, 0x68, 0xbb, 0xd5, 0x95, 0x00])]
DOCS = ('as telas da partida (11): os valores das estatísticas da partida, das zonas de ação e das notas dos jogadores',
        'the match screens (11): the match stats, action zones and player ratings values',
        [('valores_das_estatisticas_da_partida', 'refresh_match_stats_values', '0x71e750', 'match_screens.cpp', 'cdecl',
          'passa às células das estatísticas da partida os valores que mudaram desde a cópia do estado (parâmetro 0x1d) e atualiza a cópia',
          'sends the match stats cells the values that changed since the match state copy (param 0x1d) and updates the copy'),
         ('valores_das_zonas_de_acao', 'refresh_action_zones_values', '0x720320', 'match_screens.cpp', 'cdecl',
          'atualiza as zonas de ação: marcadores, posse, tempo de bola em jogo, barras das três zonas e da posse com as cores dos clubes; atualiza a cópia do estado',
          'updates the action zones: markers, possession, ball in play time, the three zone bars and the possession bar in the club colours; updates the state copy'),
         ('valores_das_notas_dos_jogadores', 'refresh_player_ratings_rows', '0x723350', 'match_screens.cpp', 'cdecl',
          'atualiza as linhas das notas dos jogadores: cartões, minuto de lesão/expulsão/substituição, nota, gols e cor do nome; atualiza a cópia do estado',
          'updates the player ratings rows: cards, injury/dismissal/substitution minute, rating, goals and name colour; updates the state copy')])
