MOTOR = True
ROTINAS = [('painel_das_estatisticas_do_time', 0x7217a0, [0x83, 0xec, 0x34, 0x53, 0x56]),
           ('painel_das_notas_dos_jogadores', 0x722ed0, [0x81, 0xec, 0x0c, 0x01, 0x00, 0x00]),
           ('painel_das_estatisticas_da_partida', 0x71e040, [0x51, 0x8b, 0x44, 0x24, 0x0c])]
DOCS = ('as telas da partida (8): os painéis das estatísticas do time, das notas dos jogadores e das estatísticas da partida',
        'the match screens (8): the team stats, player ratings and match stats panels',
        [('painel_das_estatisticas_do_time', 'draw_team_stats_panel', '0x7217a0', 'match_screens.cpp', 'cdecl',
          'desenha a tabela das estatísticas de um time (cabeçalho da página, setas, títulos das colunas) ou a mensagem de que não há estatísticas',
          'draws one team\'s stats table (page header, arrows, column titles) or the no-stats message'),
         ('painel_das_notas_dos_jogadores', 'draw_player_ratings_panel', '0x722ed0', 'match_screens.cpp', 'cdecl',
          'desenha as notas dos jogadores dos dois times: camisa, nome (botão, com o capitão) e nota',
          'draws both teams\' player ratings: shirt number, name (button, with the captain) and rating'),
         ('painel_das_estatisticas_da_partida', 'draw_match_stats_panel', '0x71e040', 'match_screens.cpp', 'cdecl',
          'desenha as estatísticas da partida: 13 linhas rotuladas (chutes, escanteios, faltas, passes, desarmes, cabeçadas, cartões)',
          'draws the match stats: 13 labelled rows (shots, corners, fouls, passes, tackles, headers, cards)')])
