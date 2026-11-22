# The player and staff search screen (search_screens.cpp, part 1; reader tel19, the screens round, 21/11/2026):
# opening it, its builder and three small getters.
# Conventions checked in the disassembly: 0x808540 cdecl(char list), SEH frame (the session allocation); 0x808690 cdecl,
# no arguments, ret 0, SEH frame and 0x1c4c bytes of locals (25 tabs of 0xd4 bytes, the filters copy, an LLIST, a Text);
# 0x80ea90 thiscall, the 64-bit mask in edx:eax; 0x80eaa0 / 0x80ead0 thiscall(char index) ret 4.
# Calls checked at the call sites: ScreenManager::open_screen ret 0x14; SearchEditSession::SearchEditSession(list)
# 0x8041c0 thiscall ret 4; SCOUT_MANAGER::set_edit_session(session, 0); get_current_results(&list, &count, comparer)
# 0x804a40 thiscall ret 0xc (runs the search once, cached); add_text_item 0x803fd0 ret 0x2c (11 dwords);
# add_button_item 0x804030 ret 0x30 (12); add_area_button 0x8040a0 ret 0x2c (11); add_area_text 0x804100 ret 0x24
# (9); ScreenManager::add_area 0x4567f0 ret 0x2c; ScreenManager::add_item 0x45cb60 ret 0x48; gui_add_tab_bar cdecl;
# gui_add_page_buttons 0x5ede20 cdecl (8); Text::draw 0x4aeaf0 thiscall ret 0x20; HUMAN_MANAGER::get_recent_file
# 0x5fc760 ret 0x18; get_password 0x5fde40 ret 4; player_search_read_file_header 0x7bcee0 cdecl;
# SearchFilters::get_attribute_minimum 0x808190 ret 8; STAFF_CONTRACT 0x85e750 ret 0xc, 0x85e8f0 ret 4, 0x868020
# ret 8; PlayerStats 0x7c25b0 ret 0x20, 0x7c1660 ret 0xc, player_stats_get_field_label 0x7c1b20 stdcall ret 0xc;
# Notes::find_note 0x795750 ret 8; notes_get_reason_label 0x795b90 stdcall ret 0x10.
# The 1080 exe patches the full-screen pick button's size in the builder (0x808e77, 0x808e7c): read from the
# running exe. The other routines are equal in both exes.
# The column comparers (0x8189f0..0x81aca0) and the SearchEditSession / SearchFilters methods are other groups'
# (tel21, tel20, tel36): called by address.
# Kept from the original: the international-duty status cell passes the previous row's club (the first row: the page
# buttons' edge) as the fourth argument; that cell, without a club, and the duty cell with a club show the buffer's
# old text; a staff Attributes view with neither Non Playing nor Filtered sorts and reads the "total found" as the
# column list (one variable in the original); the last match "-" cells get no text; `qsort` sorts the fixed attribute
# group lists in place.
TELAS = True
EXE = {}
ROTINAS = [('search_screens_open_player_search_screen', 0x808540, [0x6a, 0xff, 0x68, 0xab, 0x11, 0x96, 0x00]),
           ('search_filters_get_flag_mask', 0x80ea90, [0x8b, 0x01, 0x8b, 0x51, 0x04]),
           ('search_edit_session_get_list_type', 0x80eaa0, [0x8a, 0x44, 0x24, 0x04, 0x3c, 0xff]),
           ('search_edit_session_get_assigned_club', 0x80ead0, [0x8a, 0x44, 0x24, 0x04, 0x3c, 0xff]),
           ('search_screens_build_player_search_screen', 0x808690, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8])]
DOCS = ('a tela de busca de jogadores e funcionários (1): abrir a tela, montá-la e três leituras da sessão',
        'the player and staff search screen (1): opening it, building it and three session getters',
        [('search_screens_open_player_search_screen', 'search_screens_open_player_search_screen', '0x808540', 'search_screens.cpp', 'cdecl',
          'abre a tela de busca para a lista do olheiro: uma sessão de edição nova (entregue ao gerente dos olheiros) no parâmetro 0, os parâmetros 1..6 zerados, 7 = 1 (páginas), 8..10 = -1 e 11 zerado',
          "opens the search screen for the scout's list: a new edit session (handed to the scout manager) in parameter 0, parameters 1..6 cleared, 7 = 1 (pages), 8..10 = -1 and 11 cleared"),
         ('search_filters_get_flag_mask', 'SearchFilters::get_flag_mask', '0x80ea90', 'search_screens.cpp', 'thiscall',
          'a máscara de 64 bits dos filtros (+0 e +4)',
          "the filters' 64-bit mask (+0 and +4)"),
         ('search_edit_session_get_list_type', 'SearchEditSession::get_list_type', '0x80eaa0', 'search_screens.cpp', 'thiscall, ret 4',
          'o tipo de uma lista da sessão (1 lista curta, 2 busca rápida, 3 funcionários, 4 olheiro); -1 = a lista atual',
          "the type of a list of the session (1 short list, 2 quick search, 3 staff, 4 scout); -1 = the current list"),
         ('search_edit_session_get_assigned_club', 'SearchEditSession::get_assigned_club', '0x80ead0', 'search_screens.cpp', 'thiscall, ret 4',
          'o clube que uma lista da sessão acompanha (-1 nenhum); -1 = a lista atual',
          "the club a list of the session follows (-1 none); -1 = the current list"),
         ('search_screens_build_player_search_screen', 'search_screens_build_player_search_screen', '0x808690', 'search_screens.cpp', 'cdecl',
          'monta a tela de busca: as cores do título (o uniforme do clube do humano), as abas das dez listas, o menu Busca (olheiro, carregar, salvar, salvar para outro humano, apagar, travar, reiniciar, limpar, arquivos recentes), os menus Ver, a barra de filtros e as páginas, a linha sobre a lista (a tarefa do olheiro), o nome da lista e quantos foram achados (e filtrados), e na página atual uma linha por pessoa: o primeiro campo (seleção, jogos vistos ou estrelas), a situação, o nome (na cor do vínculo), as colunas da vista (geral, contrato, estatísticas, atributos, relatório do olheiro, notas, outros) e o valor ou a reputação; por fim Cancelar e Ok',
          "builds the search screen: the title colours (the human's club kit), the tabs of the ten lists, the Search menu (scout, load, save, save for another human, delete, lock, reset, clear, recent files), the View menus, the filters bar and the pages, the line above the list (the scout's assignment), the list's name and how many were found (and filtered), and on the current page one row per person: the first cell (national squad, matches watched or stars), the status, the name (in the colour of his ties), the view's columns (general, contract, stats, attributes, scout report, notes, other) and the value or reputation; last Cancel and Ok")])
