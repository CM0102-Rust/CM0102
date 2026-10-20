MOTOR = True
ROTINAS = [('abrir_tela_da_partida', 0x718aa0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('responder_ultimos_placares_da_partida', 0x725860, [0x51, 0x53, 0x56, 0x57, 0x6a, 0x11]),
           ('responder_estatisticas_do_time', 0x722dc0, [0x53, 0x56, 0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('as telas da partida (4): abrir a tela da partida e as respostas dos painéis dos últimos placares e das estatísticas do time',
        'the match screens (4): opening the match screen and the responses of the latest scores and team stats panels',
        [('abrir_tela_da_partida', 'open_match_screen', '0x718aa0', 'match_screens.cpp', 'cdecl',
          'abre a tela da partida ao vivo, grava os parâmetros dela e marca o humano como espectador',
          'opens the live match screen, writes its parameters and marks the human as a spectator'),
         ('responder_ultimos_placares_da_partida', 'match_screens_handle_latest_scores_cycle', '0x725860', 'match_screens.cpp', 'cdecl',
          'o painel dos últimos placares da tela da partida: a troca de página a cada 5 s e os cliques (competição, páginas, parar)',
          'the latest scores panel of the match screen: the page switch every 5 s and the clicks (competition, pages, pause)'),
         ('responder_estatisticas_do_time', 'handle_team_stats_input', '0x722dc0', 'match_screens.cpp', 'cdecl',
          'o painel das estatísticas do time: o redesenho com dados novos e os cliques que sobem e descem a linha',
          'the team stats panel: the redraw with new data and the clicks that move the row up and down')])
