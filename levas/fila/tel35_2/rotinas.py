# The history screens (History.cpp; reader tel35, the screens round, 21/11/2026): the .his file screen and the
# written history list. Uses the helpers of tel26_1 (tel26_add_area, tel26_add_item, tel26_add_button,
# tel26_area_button, tel26_item_id, tel26_item_value, tel26_error_box, tel26_copy_text, tel26_back_next,
# exe_immediate, JOB_TEXT), tel26_4 (manager_menu_bar) and tel35_1 (tel35_colour).
# Conventions checked in the disassembly: the opens cdecl(arg) (history_screen_open: the name is copied by
# set_screen_param(0xc, name, 1)); the builders and the leave routines cdecl(void); the handlers cdecl(short) -> eax.
# background_set_picture 0x41c0d0 cdecl(0); history_load_his_file 0x5f1e50 cdecl(path, 0) -> eax; Display::add_area
# 0x55fba0 (11) ret 0x2c and Display::add_item 0x55f990 (18) ret 0x48 thiscall on the slot's display;
# graphics_rgb_to_pixel 0x5e4800 cdecl(0, g, b, 0); add_text_item 0x803fd0 thiscall(SM, 11) ret 0x2c; add_button_item
# 0x804030 thiscall(SM, 12) ret 0x30; utils_build_file_path 0x90bde0 cdecl(folder, name); utils_file_exists
# 0x90c120 cdecl(path); font_text_width 0x5e5ad0 cdecl(2, text); get_area_scroll_row 0x803320 thiscall(SM, area) ret
# 4; set_widget_text 0x801c70 thiscall(SM, widget, text) ret 8; redraw_screen 0x802d00 thiscall(SM, 0, 0);
# crt_free 0x9452ca; operator new 0x944cf1 / delete 0x944c94; the vector constructor 0x9463af (array, 0xd4, 10,
# MenuItem::MenuItem 0x4568f0, 0x5a53c0) and destructor 0x9462bb (array, 0xd4, 10, 0x5a53c0); gui_add_tab_bar 0x5ed780
# cdecl(tabs, 5, tab, &top, &bottom, 1); _makepath 0x945b13; file_find_first 0x4c0aa0 cdecl(pattern, data, 0) /
# file_find_next 0x4c0b50 / file_find_close 0x4c0c10; history_get_his_file_category 0x5f2580 and
# history_get_his_file_title 0x5f2700 cdecl(name); strncpy 0x9466d0; qsort 0x945333 (list, n, 0x230, 0x5f3410);
# Area::set_scroll_row 0x403320 thiscall(area, row) ret 4; gui_add_back_next_buttons 0x5edcc0 cdecl(back, next).
# 1080: cm0102-1080.exe changes the full-screen area of history_screen_draw (push imm32 at 0x5f14c7, 0x257 -> 0x437,
# and 0x5f14cf, 0x31f -> 0x77f); the copy reads both immediates from the running exe. The rest is equal.
# Defects of the original: a 101st written history file shows the error box and leaves the file search open and the
# block of entries unkept (leaked); a first word wider than the text column makes the line search walk back past the
# start of the text; both kept.
# Comparers: the builders side by side over the display list (history_screen_draw also over the file name buffer),
# except the first build, which loads the file / searches the folder (only the original); the opens, handlers and
# leave routines only in the original.
MOTOR = True
TELAS = True
ROTINAS = [('history_screen_open', 0x5f11c0, [0x6a, 0x00, 0x68, 0x60, 0x1d, 0x5f, 0x00]),
           ('history_screen_draw', 0x5f1360, [0x81, 0xec, 0x30, 0x02, 0x00, 0x00]),
           ('history_screen_handle_event', 0x5f1ba0, [0x6a, 0x01, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('history_screen_free', 0x5f1d60, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('history_written_screen_open', 0x5f2b80, [0x6a, 0x00, 0x68, 0xd0, 0x33, 0x5f, 0x00]),
           ('history_written_screen_draw', 0x5f2c00, [0x6a, 0xff, 0x68, 0x69, 0xac, 0x95, 0x00]),
           ('history_written_screen_handle_click', 0x5f31c0, [0x81, 0xec, 0x04, 0x01, 0x00, 0x00]),
           ('history_written_screen_close', 0x5f33d0, [0x6a, 0x01, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('as telas do histórico: o arquivo .his e a lista do histórico escrito',
        'the history screens: the .his file and the written history list',
        [('history_screen_open', 'history_screen_open', '0x5f11c0', 'History.cpp', 'cdecl',
          'abre a tela de um arquivo .his com a moldura padrão (0x91, 0x4b)-(0x2e9, 0x20f), a coluna do texto a 0x115 da esquerda, a foto de 0x101 × 0x14f e as fotos em "cm3_history"',
          'opens the screen of a .his file with the default frame (0x91, 0x4b)-(0x2e9, 0x20f), the text column 0x115 from the left, the 0x101 × 0x14f picture and the pictures under "cm3_history"'),
         ('history_screen_draw', 'history_screen_draw', '0x5f1360', 'History.cpp', 'cdecl',
          'carrega o arquivo na primeira montagem e mostra o título, o seletor de registros (com mais de um), a foto do registro (a padrão quando falta o arquivo) e o texto quebrado em linhas pela largura da coluna (na última palavra que cabe, pulando os espaços), uma linha em branco depois de cada entrada, e o botão Ok; índice de arquivo fora da lista: caixa de erro',
          'loads the file on the first build and shows the title, the record chooser (with more than one), the record\'s picture (the default one when its file is missing) and the text cut into lines by the column\'s width (at the last word that fits, skipping the spaces), a blank line after each entry, and the Ok button; a file index outside the list: error box'),
         ('history_screen_handle_event', 'history_screen_handle_event', '0x5f1ba0', 'History.cpp', 'cdecl',
          'a foto acompanha a primeira linha visível do texto; Ok fecha e um registro do seletor passa a ser o mostrado',
          'the picture follows the first visible line of the text; Ok closes and a record of the chooser becomes the shown one'),
         ('history_screen_free', 'history_screen_free', '0x5f1d60', 'History.cpp', 'cdecl',
          'ao sair libera o arquivo carregado: a tabela de fotos, os textos e as listas de cada registro e a tabela de registros',
          'on leaving frees the loaded file: the picture table, each record\'s texts and lists and the record table'),
         ('history_written_screen_open', 'history_written_screen_open', '0x5f2b80', 'History.cpp', 'cdecl',
          'abre o histórico escrito numa aba, sem lista ainda',
          'opens the written history on a tab, with no list yet'),
         ('history_written_screen_draw', 'history_written_screen_draw', '0x5f2c00', 'History.cpp', 'cdecl',
          'as abas Nações, Competições, Ligas, Clubes e Jogadores e a lista, ordenada pelo título, dos arquivos .his da pasta CM3_HISTORY com a categoria da aba (até 100; o 101º dá caixa de erro)',
          'the Nations, Competitions, Leagues, Clubs and Players tabs and the list, sorted by title, of the .his files of the CM3_HISTORY folder with the tab\'s category (up to 100; the 101st gives an error box)'),
         ('history_written_screen_handle_click', 'history_written_screen_handle_click', '0x5f31c0', 'History.cpp', 'cdecl',
          'outra aba troca a categoria e descarta a lista; um item abre a tela do seu arquivo; o resto vai para a barra de menus',
          'another tab changes the category and drops the list; an entry opens its file\'s screen; the rest goes to the menu bar'),
         ('history_written_screen_close', 'history_written_screen_close', '0x5f33d0', 'History.cpp', 'cdecl',
          'ao sair apaga a lista de arquivos',
          'on leaving deletes the file list')])
