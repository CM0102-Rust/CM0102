MOTOR = True
ROTINAS = [('painel_dos_ultimos_placares', 0x723d20, [0x83, 0xec, 0x24, 0x53, 0x55]),
           ('sem_tela_seguinte', 0x7febb0, [0x0f, 0xbf, 0x81, 0x2a, 0x38, 0x00, 0x00]),
           ('cor_do_titulo', 0x839590, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0])]
DOCS = ('as telas da partida (5): o painel dos últimos placares, a tela seguinte e a cor do título',
        'the match screens (5): the latest scores panel, the next screen and the heading colour',
        [('painel_dos_ultimos_placares', 'draw_latest_scores_panel', '0x723d20', 'match_screens.cpp', 'cdecl',
          'desenha o painel dos últimos placares: título, selo, grupos e partidas do dia, tabela e botões',
          'draws the latest scores panel: title, badge, the day\'s groups and matches, table and buttons'),
         ('sem_tela_seguinte', 'ScreenManager::is_forward_unavailable', '0x7febb0', 'scrman.cpp', 'thiscall', 'o gerente de telas não tem tela seguinte para avançar', 'the screen manager has no next screen to go forward to'),
         ('cor_do_titulo', 'get_heading_colour', '0x839590', 'Setup.cpp', 'cdecl',
          'a cor do texto do título (duas cores pela marca)',
          'the heading text colour (two colours by the flag)')])
