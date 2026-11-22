# The interface building blocks: a text area, a text area from a Text, a block of menu buttons and a tab bar
# (gui_utils.cpp; reader tel37, the screens round, 21/11/2026).
# Conventions checked in the disassembly (every push against esp, every caller's add esp):
# - gui_add_text_area 0x5ed070 cdecl, variadic (x1, y1, x2, y2, marks, colour, rows, font, text colour, text, then
#   link triples (text, value, value) ending in a null text; the 66 callers pass 11, 14 or 17 dwords): the copy takes
#   the arguments as one repr(C) record by value (the caller's argument area), at most 8 links.
# - gui_add_text_area_from_text 0x5ed200 cdecl (the same ten, the last a Text*); gui_add_menu_buttons 0x5ed300 cdecl
#   (x1, y1, x2, y2, columns, rows, font, n, buttons, selected bits, sort); gui_add_tab_bar 0x5ed780 cdecl (tabs, n,
#   selected id, *top y, *bottom y, odd first). All return 1 (0 after an error box).
# - Called: Display::add_area 0x55fba0 (tel26_add_area) and Display::add_item 0x55f990 (tel26_add_item) on the
#   current Display (ScreenManager +0x7fc of the current channel); Text::Text / set_string / add_link / ~Text;
#   Text 0x4aeaf0 thiscall(area, x1, x2, *row, font, colour, centred, lines) ret 0x20; font_text_width; strchr
#   0x946540; operator new / delete, vector_constructor_iterator / vector_destructor_iterator (stdcall) with the
#   MenuButton constructor 0x4568f0 and destructor 0x5a53c0; utils_swap_bytes (cdecl).
# - The 1080 exe changes the tab colour (the push imm8 at 0x5ed7ab and 0x5ed7ad): the copy reads the three immediates
#   from the running exe. Nothing else of this item differs in the 1080 exe.
# - The original opens an exception frame in gui_add_text_area, gui_add_menu_buttons and gui_add_tab_bar (for the
#   Text or the button array); the copy does not (no exception is thrown there).
# Where they run: text areas on most screens (66 callers); text areas from a Text (8 callers), menu buttons (3
# callers, 0x81bcc5..0x81ef12) and tab bars (15 callers) on the screens that call them [which screens: not checked].
# Depends on tel26_1 (tel26_add_area, tel26_add_item, tel26_error_box, display_list, display_item_at,
# display_area_at, JOB_COLOUR_BDE8, first_differences), tel29_1 (screen_record) and tel37_1 (network_buffer,
# DISPLAY_PACKET_BUFFER).
# Defects of the original: gui_add_tab_bar keeps the raw allocation pointer as the selected group when no tab has
# the selected id (then every tab with a non-zero group is drawn as "another group"); the menu
# buttons' colour dword is an uninitialised local when the sort did not run (only its low word is meaningful).
TELAS = True
ROTINAS = [('gui_add_text_area', 0x5ed070, [0x6a, 0xff, 0x68, 0x98, 0xab, 0x95, 0x00]),
           ('gui_add_text_area_from_text', 0x5ed200, [0x8b, 0x44, 0x24, 0x18, 0x8b, 0x4c, 0x24, 0x14]),
           ('gui_add_menu_buttons', 0x5ed300, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('gui_add_tab_bar', 0x5ed780, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os blocos da interface: uma área de texto, uma área de texto de um Text, um bloco de botões de menu e uma barra de abas',
        'the interface building blocks: a text area, a text area from a Text, a block of menu buttons and a tab bar',
        [('gui_add_text_area', 'gui_add_text_area', '0x5ed070', 'gui_utils.cpp', 'cdecl, variádica',
          'acrescenta uma área de uma coluna e N linhas e, com texto, o texto com os seus vínculos: numa linha só (cabe na largura menos 9 e sem "\\n") centrado na linha do meio, senão das linhas do alto; entre x1 + 5 e x2 - 5',
          'adds an area of one column and N rows and, with a text, the text with its links: on one line (it fits the width minus 9 and has no "\\n") centred on the middle row, else from the top rows; between x1 + 5 and x2 - 5'),
         ('gui_add_text_area_from_text', 'gui_add_text_area_from_text', '0x5ed200', 'gui_utils.cpp', 'cdecl',
          'o mesmo que gui_add_text_area com um Text já montado no lugar do texto e dos vínculos',
          'the same as gui_add_text_area with a ready Text instead of the text and links'),
         ('gui_add_menu_buttons', 'gui_add_menu_buttons', '0x5ed300', 'gui_utils.cpp', 'cdecl',
          'acrescenta um bloco de botões de menu: copia os N registros (ordenados pelo texto quando pedido, os bits de seleção acompanhando), uma área de colunas × linhas e um botão por registro (o selecionado em amarelo, os outros em cinza claro), a última linha centrada; sem botões a caixa de gui_utils.cpp 193',
          'adds a block of menu buttons: copies the N records (sorted by text when asked, the selection bits following), an area of columns × rows and one button per record (the selected one yellow, the others pale grey), the last row centred; without buttons the gui_utils.cpp 193 box'),
         ('gui_add_tab_bar', 'gui_add_tab_bar', '0x5ed780', 'gui_utils.cpp', 'cdecl',
          'acrescenta uma barra de até 12 abas em uma ou duas fileiras (com mais de 5, a de cima leva metade), cada fileira com um painel e a área das abas, 0x23 de altura, descendo o y de cima e subindo o de baixo 0x2d; a aba selecionada, as do mesmo grupo e as outras com cores próprias; as caixas de gui_utils.cpp 350, 358 e 366',
          'adds a bar of up to 12 tabs on one or two rows (with more than 5, the top row takes half), each row with a panel and the tab area, 0x23 high, moving the top y down and the bottom y up by 0x2d; the selected tab, those of its group and the others with their own colours; the gui_utils.cpp 350, 358 and 366 boxes')])
