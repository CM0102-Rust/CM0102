MOTOR = True
ROTINAS = [('painel_dos_ultimos_placares', 0x723d20, [0x83, 0xec, 0x24, 0x53, 0x55]),
           ('altura_da_fonte', 0x5e5cd0, [0x83, 0x3d, 0xc4, 0xc2, 0x9c, 0x00, 0xff]),
           ('cor_do_titulo', 0x839590, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0])]
DOCS = ('as telas da partida (5): o painel dos últimos placares, a altura da fonte e a cor do título',
        'the match screens (5): the latest scores panel, the font height and the heading colour',
        [('painel_dos_ultimos_placares', 'draw_latest_scores_panel', '0x723d20', 'match_screens.cpp', 'cdecl',
          'desenha o painel dos últimos placares: título, selo, grupos e partidas do dia, tabela e botões',
          'draws the latest scores panel: title, badge, the day\'s groups and matches, table and buttons'),
         ('altura_da_fonte', 'font_get_height', '0x5e5cd0', 'goldcup.cpp', 'cdecl',
          'a altura da linha de uma fonte (tabela fixa com as fontes próprias, senão a das fontes carregadas)',
          'the line height of a font (fixed table with the built-in fonts, otherwise the loaded fonts table)'),
         ('cor_do_titulo', 'get_heading_colour', '0x839590', 'Setup.cpp', 'cdecl',
          'a cor do texto do título (duas cores pela marca)',
          'the heading text colour (two colours by the flag)')])
