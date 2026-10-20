MOTOR = True
ROTINAS = [('painel_das_zonas_de_acao', 0x71f500, [0xa0, 0xb4, 0xb7, 0xb6, 0x00]),
           ('painel_do_relatorio_da_partida', 0x721000, [0x6a, 0xff, 0x68, 0x9e, 0xd5, 0x95, 0x00]),
           ('montar_jogos_do_dia', 0x717db0, [0x81, 0xec, 0x9c, 0x00, 0x00, 0x00])]
DOCS = ('as telas da partida (10): os painéis das zonas de ação e do relatório da partida e a tela dos jogos do dia',
        'the match screens (10): the action zones and match report panels and the day fixtures screen',
        [('painel_das_zonas_de_acao', 'draw_action_zones_panel', '0x71f500', 'match_screens.cpp', 'cdecl',
          'desenha as zonas de ação: o seletor de período, o campo dividido em três zonas com as barras das cores dos dois clubes, ou a mensagem de que não há zonas',
          'draws the action zones: the period selector, the pitch split into three zones with both clubs\' colour bars, or the no-zones message'),
         ('painel_do_relatorio_da_partida', 'draw_match_report_panel', '0x721000', 'match_screens.cpp', 'cdecl',
          'desenha o relatório da partida: os minutos e os textos dos lances (criando o relatório fora da partida), ou a mensagem de que não há relatório',
          'draws the match report: the minutes and the event texts (building the report outside the match), or the no-report message'),
         ('montar_jogos_do_dia', 'build_day_fixtures_screen', '0x717db0', 'match_screens.cpp', 'cdecl',
          'monta a tela dos jogos do dia: título, árvore das competições do dia, a tabela das rodadas e partidas e a rolagem até a partida do humano',
          'builds the day fixtures screen: title, the day\'s competition tree, the rounds and matches table and the scroll to the human\'s match')])
