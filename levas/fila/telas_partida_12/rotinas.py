MOTOR = True
ROTINAS = [('preencher_estatisticas_do_time', 0x721cd0, [0x81, 0xec, 0x18, 0x01, 0x00, 0x00]),
           ('atualizar_ultimos_placares', 0x724520, [0x81, 0xec, 0x3c, 0x04, 0x00, 0x00]),
           ('esperar_os_outros_jogadores', 0x727fb0, [0x83, 0xec, 0x44, 0x53, 0x56])]
DOCS = ('as telas da partida (12): as linhas das estatísticas do time, o redesenho dos últimos placares e a espera dos outros jogadores',
        'the match screens (12): the team stats rows, the latest scores refresh and the wait for the other players',
        [('preencher_estatisticas_do_time', 'fill_team_stats_rows', '0x721cd0', 'match_screens.cpp', 'cdecl',
          'escreve as 20 linhas de jogador das estatísticas do time (22 colunas pela tabela do parâmetro 0x1c), montando as células ou atualizando o que mudou',
          'writes the 20 player rows of the team stats table (22 columns from the param 0x1c table), building the cells or updating what changed'),
         ('atualizar_ultimos_placares', 'refresh_latest_scores_rows', '0x724520', 'match_screens.cpp', 'cdecl',
          'reescreve as linhas dos últimos placares em páginas: fase, jogo, placar, período, autores dos eventos, agregado e o rodapé da página',
          'rewrites the latest scores rows in pages: stage, fixture, score, period, event scorers, aggregate and the page footer'),
         ('esperar_os_outros_jogadores', 'match_screens_handle_waiting_screen', '0x727fb0', 'match_screens.cpp', 'cdecl',
          'no jogo em rede, espera os outros humanos da mesma partida ou fecha as telas, avisa e abre a tela da partida',
          'in a network game, waits for the other humans of the same fixture or closes the screens, shows the notice and opens the match screen')])
