MOTOR = True
ROTINAS = [('atualizar_barra_da_posse', 0x71d9e0, [0x6a, 0xff, 0x68, 0x3b, 0xd5, 0x95, 0x00]),
           ('painel_da_visao_geral_da_partida', 0x71c270, [0x6a, 0xff, 0x68, 0x1b, 0xd5, 0x95, 0x00]),
           ('tabela_da_liga_ao_vivo', 0x7280e0, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as telas da partida (13): a barra da posse, o painel da visão geral da partida e a tabela da liga ao vivo',
        'the match screens (13): the possession bar, the match overview panel and the live league table',
        [('atualizar_barra_da_posse', 'refresh_possession_bar', '0x71d9e0', 'match_screens.cpp', 'cdecl',
          'redesenha a barra da posse da visão geral quando a posse muda (percentuais e a cor do clube com mais posse) e guarda o estado da partida na cópia do parâmetro 0x1d',
          'redraws the overview possession bar when the share changes (percentages and the colour of the club with more of the ball) and stores the match state in the param 0x1d copy'),
         ('painel_da_visao_geral_da_partida', 'draw_match_overview_panel', '0x71c270', 'match_screens.cpp', 'cdecl',
          'desenha a visão geral da partida: período, barra da posse e comentário, data, placar com intervalo/prorrogação/pênaltis, competição, autores dos gols e cartões, disputa de pênaltis, árbitro, público, tempo e partida abandonada',
          'draws the match overview: period, possession bar and commentary, date, score with half time/extra time/penalties, competition, scorers and cards, shoot-out, referee, attendance, weather and abandoned match'),
         ('tabela_da_liga_ao_vivo', 'match_screens_draw_live_league_table', '0x7280e0', 'match_screens.cpp', 'cdecl',
          'desenha a tabela da liga ao vivo (liga, topo, base) numa cópia da liga com os placares do dia aplicados (ou o aviso da data do sorteio): título, cabeçalhos, separadores, posição, seta de subida/descida, clube e colunas',
          'draws the live league table (league, top, bottom) on a copy of the league with the day scores applied (or the draw date notice): title, headers, separators, position, up/down arrow, club and columns')])
