# As áreas da interface (2) (Area.cpp; leitor tel29, rodada das telas, 21/11/2026): liberar, acrescentar um item, a
# linha de rolagem, a barra de rolagem, a posição de uma área suspensa e mover.
# Convenções conferidas na desmontagem (0x4031a0..0x40333f, 0x403600..0x403edf): todas thiscall, ecx = a área (0xbf1
# bytes); release, get_shown_row e draw_scrollbar ret 0; add_widget ret 4 (índice short); set_scroll_row ret 4;
# setup_scrollbar ret 8 (margens); get_popup_position ret 8 (out_x, out_y); move_to ret 8 (x, y). O exe de 1920x1080
# reescreve os imediatos 600 de get_popup_position (0x40384d, 0x403924, 0x4039b7) e 0x31f/0x257 de move_to
# (0x403b02/0x403b65, 0x403bde/0x403c41): lidos do próprio exe; o 400 (0x403969 e 0x403a11) não é reescrito.
# Comparadores: add_widget, set_scroll_row, setup_scrollbar, get_popup_position, move_to e get_shown_row lado a lado
# (a área, as saídas, a barra do teclado); release e draw_scrollbar só na original (liberam ou guardam imagens e
# desenham), release conferindo o plano.
TELAS = True
ROTINAS = [('area_release', 0x4031a0, [0x56, 0x8b, 0xf1, 0x8b, 0x86, 0xa6, 0x0b, 0x00, 0x00]),
           ('area_add_widget', 0x403200, [0x51, 0x53, 0x55, 0x56, 0x66, 0x8b, 0x74, 0x24, 0x14]),
           ('area_get_shown_row', 0x4032e0, [0x8b, 0x81, 0xed, 0x0b, 0x00, 0x00]),
           ('area_set_scroll_row', 0x403320, [0x8b, 0x51, 0x14, 0x33, 0xc0]),
           ('area_setup_scrollbar', 0x403600, [0x8b, 0x41, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('area_get_popup_position', 0x403760, [0x66, 0x8b, 0x81, 0x0e, 0x02, 0x00, 0x00]),
           ('area_move_to', 0x403a70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('area_draw_scrollbar', 0x403c80, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as áreas da interface (2): liberar, acrescentar um item, a linha de rolagem, a barra de rolagem, a posição de uma área suspensa e mover',
        'the interface areas (2): releasing, adding a widget, the scroll row, the scroll bar, the popup position and moving',
        [('area_release', 'Area::release', '0x4031a0', 'Area.cpp', 'thiscall',
          'libera as imagens guardadas atrás da barra de rolagem e atrás da área e solta a barra do teclado se for desta área',
          "frees the images saved behind the scroll bar and behind the area and drops the keyboard scroll bar when it is this area's"),
         ('area_add_widget', 'Area::add_widget', '0x403200', 'Area.cpp', 'thiscall, ret 4',
          'acrescenta um item à lista da área, ordenada pela linha do item (até 1200; iguais mantêm a ordem), e alarga a primeira e a última linha',
          "adds a widget to the area's list, sorted by the widget's row (up to 1200; equal rows keep their order), and widens the first and last rows"),
         ('area_get_shown_row', 'Area::get_shown_row', '0x4032e0', 'Area.cpp', 'thiscall',
          'a linha do primeiro item à mostra quando a área tem barra de rolagem, senão 0',
          'the row of the first visible widget when the area has a scroll bar, otherwise 0'),
         ('area_set_scroll_row', 'Area::set_scroll_row', '0x403320', 'Area.cpp', 'thiscall, ret 4',
          'guarda a linha de rolagem pedida, entre 0 e a última linha menos as linhas à mostra mais uma',
          'stores the requested scroll row, between 0 and the last row less the visible rows plus one'),
         ('area_setup_scrollbar', 'Area::setup_scrollbar', '0x403600', 'Area.cpp', 'thiscall, ret 8',
          'monta a barra de rolagem dentro das margens: a barra, o trilho entre as setas, o cursor (altura proporcional às linhas à mostra, ao menos 10), a linha pendente aplicada, e a barra passa a ser a do teclado',
          'sets up the scroll bar inside the margins: the bar, the track between the arrows, the thumb (height proportional to the visible rows, at least 10), the pending row applied, and the bar becomes the keyboard\'s'),
         ('area_get_popup_position', 'Area::get_popup_position', '0x403760', 'Area.cpp', 'thiscall, ret 8',
          'onde uma área suspensa abre, ao lado ou embaixo do item dela, dentro da tela (600 de altura lido do exe, meio da tela em 400)',
          'where a popup area opens, beside or under its widget, inside the screen (the 600 height read from the exe, the middle of the screen at 400)'),
         ('area_move_to', 'Area::move_to', '0x403a70', 'Area.cpp', 'thiscall, ret 8',
          'move a área para o canto pedido; fora da tela, as caixas de erro e o movimento limitado à tela (pela original)',
          'moves the area to the given corner; off the screen, the error boxes and the move clamped to the screen (through the original)'),
         ('area_draw_scrollbar', 'Area::draw_scrollbar', '0x403c80', 'Area.cpp', 'thiscall',
          'desenha a barra de rolagem: o botão de cima com a seta, o trilho escuro, o cursor e o botão de baixo com a seta (afundados quando apertados), guardando antes a imagem atrás da barra',
          'draws the scroll bar: the up button with its arrow, the dark track, the thumb and the down button with its arrow (sunken when pressed), saving the image behind the bar first')])
