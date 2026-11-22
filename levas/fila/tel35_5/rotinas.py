# The menu bar's records, its menu lists and the manager chat screen (Menubar.cpp; reader tel35, the screens round,
# 21/11/2026). Uses the helpers of tel26_1 (tel26_add_item, tel26_add_area, tel26_area_text, tel26_item_id,
# tel26_slot_manager, tel26_copy_text, tel26_back_next, display_item_at, display_item::TEXT, JOB_TEXT), tel26_6
# (MENUBAR_RECORDS, MENUBAR_RECORD_SIZE), tel35_1 (tel35_colour) and tel35_2 (tel35_text_item, tel35_cdecl1,
# tel35_c_string_len).
# Conventions checked in the disassembly: menubar_init_human_manager_records cdecl (the vector constructor 0x9463af
# (0xb73390, 0xf6, 16, 0x75da80, 0x5a53c0) and atexit 0x94627a (0x75dae0)); HUMAN_MANAGER_RECORD::HUMAN_MANAGER_RECORD
# thiscall -> this; menubar_destroy_human_manager_records cdecl (the vector destructor 0x9462bb); menubar_add_separator
# thiscall(SM, area) ret 4 (the display of [SM + 0x7fc + 0x300 x [SM + 0x382a]]); menubar_add_change_player_menu,
# menubar_list_continents and menubar_list_league_nations cdecl(3) (the change player menu returns 1);
# get_channel_manager_keys 0x7ffd80 thiscall(SM, keys, &count, slot) ret 0xc; is_current_manager_being_created
# 0x838de0 cdecl; qsort 0x945333 (keys, n, 4, database_compare_staff_by_full_name 0x540270) and (order, n, 1,
# 0x765b90); club_get_national_team_name 0x542100 cdecl(team, 0, out, 100, 1, 1); is_manager_done 0x7ffe40
# thiscall(SM, person) ret 4; key_nation_sorted_order 0x66a0c0 cdecl(0); NATION_MENU_SPLIT ctor 0x66a280 (0, order, 0)
# ret 0xc, part_count 0x66a350 (AL), in_second_part 0x66a430 (league) ret 4 (AL, asked twice as the original),
# format_header 0x66a360 (out, 2000, part) ret 0xc; the manager chat builder and handler cdecl (get_game_title
# 0x839570, get_heading_colour 0x839590, add_text_item 0x803fd0, add_area_text 0x804100, set_widget_text 0x801c70,
# set_widget_text_colour 0x801960, redraw_screen 0x802d00); utils_string_compare 0x90c1d0 cdecl(a, b, 1, 0).
# Equal in cm0102-1080.exe.
# Defects of the original: menubar_list_league_nations puts the nations in the order array's address as their area
# when the split has no part (part_count 0); manager_chat_screen_input compares each line with a null text when a line
# widget index is -1 (a crash); both kept.
# Comparers: the record constructor side by side over the record; the lists, the separator and the change player menu
# side by side over the display list (the change player menu also over the row); the chat builder over the display
# list and the changed flags (the first build sets the chat up: only the original); the start-up/exit routines and
# the chat handler only in the original.
MOTOR = True
TELAS = True
ROTINAS = [('menubar_init_human_manager_records', 0x75da50, [0x68, 0xc0, 0x53, 0x5a, 0x00]),
           ('human_manager_record_human_manager_record', 0x75da80, [0x8b, 0xc1, 0x83, 0xca, 0xff]),
           ('menubar_destroy_human_manager_records', 0x75dae0, [0x68, 0xc0, 0x53, 0x5a, 0x00]),
           ('menubar_add_separator', 0x761260, [0x8b, 0x44, 0x24, 0x04, 0x66, 0x8b, 0x15, 0xea, 0xbd, 0xae, 0x00]),
           ('menubar_add_change_player_menu', 0x7612c0, [0x66, 0xa1, 0x7a, 0x7c, 0xb7, 0x00]),
           ('menubar_list_continents', 0x765010, [0x8b, 0x15, 0x4c, 0x23, 0xae, 0x00]),
           ('menubar_list_league_nations', 0x7651e0, [0x81, 0xec, 0xac, 0x01, 0x00, 0x00]),
           ('manager_chat_screen_build', 0x7654d0, [0xa1, 0x14, 0x43, 0xb7, 0x00]),
           ('manager_chat_screen_input', 0x7657c0, [0x83, 0xec, 0x1c, 0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('compare_continents_by_name', 0x765b90, [0x8b, 0x44, 0x24, 0x04, 0x57])]
DOCS = ('a barra de menus (2): os registros dos técnicos humanos, as listas dos menus e o bate-papo dos técnicos',
        'the menu bar (2): the human manager records, the menu lists and the manager chat',
        [('menubar_init_human_manager_records', 'menubar_init_human_manager_records', '0x75da50', 'Menubar.cpp', 'cdecl',
          'na partida do programa constrói os 16 registros de barra de menus dos técnicos humanos e registra a destruição na saída',
          'at program start builds the 16 human manager menu bar records and registers their destruction at exit'),
         ('human_manager_record_human_manager_record', 'HUMAN_MANAGER_RECORD::HUMAN_MANAGER_RECORD', '0x75da80', 'Menubar.cpp', 'thiscall',
          'um registro novo: sem técnico, sem modo, sem carimbos (os campos com 0, -1 e 1 da original)',
          'a fresh record: no manager, no mode, no stamps (the original\'s 0, -1 and 1 fields)'),
         ('menubar_destroy_human_manager_records', 'menubar_destroy_human_manager_records', '0x75dae0', 'Menubar.cpp', 'cdecl',
          'na saída destrói os 16 registros',
          'at exit destroys the 16 records'),
         ('menubar_add_separator', 'menubar_add_separator', '0x761260', 'Menubar.cpp', 'thiscall',
          'um separador de menu: uma linha vazia (marcas 0x1000010) na área do menu',
          'a menu separator: an empty line (flags 0x1000010) in the menu area'),
         ('menubar_add_change_player_menu', 'menubar_add_change_player_menu', '0x7612c0', 'Menubar.cpp', 'cdecl',
          'com mais de um humano, "Change Player" com os técnicos em ordem de nome ("nome (clube)"), o atual em cinza claro e os que já terminaram, fora da espera, sem link',
          'with more than one human, "Change Player" with the managers in name order ("name (club)"), the current one pale grey and those done, outside the waiting menu, without link'),
         ('menubar_list_continents', 'menubar_list_continents', '0x765010', 'Menubar.cpp', 'cdecl',
          'os continentes em ordem de nome, o escolhido marcado',
          'the continents in name order, the chosen one ticked'),
         ('menubar_list_league_nations', 'menubar_list_league_nations', '0x7651e0', 'Menubar.cpp', 'cdecl',
          'as 35 nações com liga carregada na ordem das chaves, em partes com cabeçalho quando a lista é longa, a escolhida marcada',
          'the 35 nations with a loaded league in the key order, in parts with a header when the list is long, the chosen one ticked'),
         ('manager_chat_screen_build', 'manager_chat_screen_build', '0x7654d0', 'Menubar.cpp', 'cdecl',
          'a tela do bate-papo: na primeira vez prepara as 16 páginas ("Welcome to manager chat"); mostra a página do humano (as linhas dele em amarelo, as dos outros em cinza claro), o "Say:" e a caixa de texto',
          'the chat screen: the first time sets the 16 pages up ("Welcome to manager chat"); shows the human\'s page (his lines yellow, the others\' pale grey), "Say:" and the text box'),
         ('manager_chat_screen_input', 'manager_chat_screen_input', '0x7657c0', 'Menubar.cpp', 'cdecl',
          'mostra de novo a página que mudou; "Send" escreve " nome sobrenome: texto" na página de todos os humanos (rolando a página cheia) e as marca mudadas',
          'shows a changed page again; "Send" writes " first second: text" on every human\'s page (scrolling a full page) and marks them changed'),
         ('compare_continents_by_name', 'compare_continents_by_name', '0x765b90', 'Menubar.cpp', 'cdecl',
          'a ordem dos continentes pelo nome',
          'the order of the continents by name')])
