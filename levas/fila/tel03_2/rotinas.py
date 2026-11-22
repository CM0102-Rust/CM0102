# The club history screen (club_screens.cpp, part 3; reader tel03, the screens round, 21/11/2026): opening, building,
# the input handler and its three View popups, leaving.
# - 0x469e70 open_club_history_screen cdecl(club, restore, players_tab) ret 0 (0xf0 bytes of locals: the previous
#   screen's 60 params when restoring); 0x7fe600 open_screen thiscall(builder 0x46a1e0, handler 0x46dd20, 0, leave
#   0x46e560, 0) ret 0x14, 0x7ff270 set_screen_param thiscall(k, v, 0) ret 0xc.
# - 0x46a1e0 club_history_screen_build cdecl(void) ret 0, SEH, 0x1ec0 bytes of locals (25 tab items built by the CRT
#   vector iterators); the map's 870 bytes go on to 0x46ad68 (the Landmarks and Records tabs are inline, here the named
#   pieces club_history_landmarks_tab and club_history_records_tab) plus the jump table 0x46ad68..0x46ad8c.
#   Calls: 0x41c8e0 background_set_section_picture cdecl(8, 0); 0x53b640 database_get_club_kit_colours; 0x803f30
#   add_screen_title; 0x75db00 menubar_build; 0x482a40 add_club_screen_action_button cdecl(club, c1, c2); 0x5ed780
#   gui_add_tab_bar; the tabs 0x46adc0/0x46c030/0x46c510/0x46c9e0/0x46ceb0/0x46d370/0x46d860 cdecl(top, bottom) (item
#   tel03_3); 0x86df90 Staff_Records::get_club_history thiscall(club, 0) ret 8, 0x86f300 format_club_history_entry
#   thiscall(node, Text*) ret 8, 0x54a650 Date::get_short_date_string thiscall ret 0xc, 0x5ede20 gui_add_page_buttons
#   cdecl(8 args), 0x4567a0 ScreenManager::set_area_scroll_row thiscall ret 8, 0x4567f0 ScreenManager::add_area
#   thiscall ret 0x2c, 0x456850 add_popup_area thiscall ret 4, 0x415640 add_menu_item thiscall ret 0x10,
#   0x448fa0/0x4490b0/0x449590/0x4496d0 Club_Records label, type label, applies, record text (thiscall on [0xad9dac],
#   ret 0xc/0xc/0xc/0x10), 0x46ad90 Display::set_area_scroll_row thiscall ret 8, 0x4aeaf0 Text::draw_wrapped thiscall
#   ret 0x20, 0x481ef0 build_club_squad_navigation_menu cdecl(8 args), 0x7febb0/0x7fec60/0x5edcc0 Back/Next.
# - 0x46dd20 club_history_screen_input cdecl(short widget) ret 0, returns −11, −4 or the menu bar's answer; jump table
#   0x46e230 (9 tabs) inside the region; 0x483920 club_record_type_to_category cdecl(char, int*) (the out param is the
#   argument slot), 0x803320 get_area_scroll_row thiscall ret 4, 0x545880 club_get_english_name, 0x5f11c0
#   history_screen_open cdecl, 0x482d10 handle_club_screen_action, 0x7615d0/0x7643d0 the menu bar.
# - 0x46e260/0x46e360/0x46e460 club_history_{sequences,players,transfers}_tab_input cdecl(short widget) ret 0
#   (one shared helper with each tab's params; the item id read inline from the current human's Display).
# - 0x46e560 club_history_screen_leave cdecl(void) ret 0: 0x68d040 ~LLIST, 0x944c94 delete, 0x9452ca free.
# Kept from the original: in the Records tab a view other than 1 and 2 leaves the heading as stack garbage (empty
#   here); the colour words go in dword slots whose high half is register garbage in the original (0 here).
# Comparers: opening only the original, checked against the plan (club_history_planned_params); the build side by
#   side by the display list (original only while the open tab still builds its cached lists, params 0x19..0x1c or 5,
#   and with no club: the assert box); the input side by side only for the clicks that just write params
#   (club_history_click_stays), else original only; the three View popups side by side; leaving only the original,
#   with the cleared params checked.
# How to reach it: a club's screen > History (menu bar or the club's navigation), or any club history link; the tabs
#   Competitions, Landmarks (a club with over 100 landmarks for the pages), Records (View 1/2, click a record type),
#   Positions, Attendances, Results, Sequences, Players, Transfers (each with its View popup); a national team for the
#   greyed tabs; the Hall of Fame button (action 0xc).
TELAS = True
ROTINAS = [('open_club_history_screen', 0x469e70, [0x81, 0xec, 0xf0, 0x00, 0x00, 0x00]),
           ('club_history_screen_build', 0x46a1e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('club_history_screen_input', 0x46dd20, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('club_history_sequences_tab_input', 0x46e260, [0x56, 0x66, 0x8b, 0x74, 0x24, 0x08]),
           ('club_history_players_tab_input', 0x46e360, [0x56, 0x66, 0x8b, 0x74, 0x24, 0x08]),
           ('club_history_transfers_tab_input', 0x46e460, [0x56, 0x66, 0x8b, 0x74, 0x24, 0x08]),
           ('club_history_screen_leave', 0x46e560, [0x56, 0x6a, 0x19, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('a tela de história do clube: abrir, montar, responder, sair',
        'the club history screen: opening, building, answering, leaving',
        [('open_club_history_screen', 'open_club_history_screen', '0x469e70', 'club_screens.cpp', 'cdecl',
          'Abre a história do clube (montagem 0x46a1e0, resposta 0x46dd20, saída 0x46e560) e grava os parâmetros: 0 o clube, 1 a aba (3 Recordes, ou 8 Jogadores com "aba de jogadores"), 0x19..0x1c as listas (nenhuma), 0x1d o período dos marcos −1, 2 a vista dos recordes 1, 7, 0xa, 0xd, 0x10, 0x13 e 0x16 as vistas das abas de posições, públicos, resultados, sequências, jogadores e transferências (1, 3, 7, 0xc, 0x10 ou 0x20, 0x1c), 0x20 a página 1, 0x23 o total de páginas 1, o resto 0. Com "restaurar", os parâmetros 1, 2, 7, 0xa, 0xd, 0x10, 0x13 e 0x16 voltam da tela anterior.',
          'Opens the club history (builder 0x46a1e0, handler 0x46dd20, leave 0x46e560) and writes the params: 0 the club, 1 the tab (3 Records, or 8 Players with "players tab"), 0x19..0x1c the lists (none), 0x1d the landmarks period −1, 2 the records view 1, 7, 0xa, 0xd, 0x10, 0x13 and 0x16 the views of the positions, attendances, results, sequences, players and transfers tabs (1, 3, 7, 0xc, 0x10 or 0x20, 0x1c), 0x20 the page 1, 0x23 the page count 1, the rest 0. With "restore", params 1, 2, 7, 0xa, 0xd, 0x10, 0x13 and 0x16 come back from the previous screen.'),
         ('club_history_screen_build', 'club_history_screen_build', '0x46a1e0', 'club_screens.cpp', 'cdecl, SEH',
          'Monta a história do clube do parâmetro 0 (sem clube, a caixa de erro): a foto de seção 8, o título "{clube} History" nas cores do uniforme, a barra de menus, o botão de ação do clube, as 9 abas (para uma seleção, Posições e Transferências apagadas, e a aba aberta numa delas volta a Competições), a aba aberta: Marcos (a história do clube da mais nova para a mais velha, data e texto; mais de 100 em páginas com os botões de página), Recordes (o botão View com os dois períodos e uma linha por tipo de recorde que vale para o clube no período, o tipo como botão e o texto do recorde) e as outras pelas suas rotinas; no fim o menu de navegação do elenco e Voltar/Avançar.',
          'Builds the club history of param 0 (with no club, the error box): the section picture 8, the title "{club} History" in the kit colours, the menu bar, the club\'s action button, the 9 tabs (for a national team, Positions and Transfers greyed out, and an open one of them goes back to Competitions), the open tab: Landmarks (the club\'s history from the newest, date and text; over 100 in pages with the page buttons), Records (the View button with the two periods and one row per record type that applies to the club in the period, the type as a button and the record\'s text) and the others by their own routines; at the end the squad navigation menu and Back/Next.'),
         ('club_history_screen_input', 'club_history_screen_input', '0x46dd20', 'club_screens.cpp', 'cdecl',
          'Responde ao clique na história do clube: pela aba aberta (cada uma guarda antes a linha de rolagem da sua área): Competições escolhe a vista (0x1d), Marcos muda a página (0x20, de 1 ao total), Recordes escolhe o período (2) ou, num tipo de recorde, vai à aba da categoria com aquele tipo, Posições/Públicos/Resultados escolhem a vista (7, 0xa, 0xd), Sequências/Jogadores/Transferências pelos seus popups; depois, em qualquer aba: ação 1 abre a história de outro clube (restaurando as vistas), 0xc o Hall da Fama, 2 muda de aba; o resto vai às ações do clube e à barra de menus.',
          'Answers a click on the club history: by the open tab (each saving its area\'s scroll row first): Competitions picks the view (0x1d), Landmarks changes the page (0x20, from 1 to the count), Records picks the period (2) or, on a record type, goes to that category\'s tab with that type, Positions/Attendances/Results pick the view (7, 0xa, 0xd), Sequences/Players/Transfers through their popups; then on any tab: action 1 opens another club\'s history (restoring the views), 0xc the Hall of Fame, 2 changes the tab; the rest goes to the club actions and the menu bar.'),
         ('club_history_sequences_tab_input', 'club_history_sequences_tab_input', '0x46e260', 'club_screens.cpp', 'cdecl',
          'O popup View da aba Sequências: guarda a rolagem (0x11) e, com a ação 9 e um tipo diferente, grava o tipo (0x10) e pede remontagem (−11).',
          'The Sequences tab\'s View popup: saves the scroll row (0x11) and, on action 9 with a different type, writes the type (0x10) and asks for a rebuild (−11).'),
         ('club_history_players_tab_input', 'club_history_players_tab_input', '0x46e360', 'club_screens.cpp', 'cdecl',
          'O popup View da aba Jogadores: rolagem em 0x14, ação 0xa grava o tipo em 0x13 (−11).',
          'The Players tab\'s View popup: scroll row in 0x14, action 0xa writes the type to 0x13 (−11).'),
         ('club_history_transfers_tab_input', 'club_history_transfers_tab_input', '0x46e460', 'club_screens.cpp', 'cdecl',
          'O popup View da aba Transferências: rolagem em 0x17, ação 0xb grava o tipo em 0x16 (−11).',
          'The Transfers tab\'s View popup: scroll row in 0x17, action 0xb writes the type to 0x16 (−11).'),
         ('club_history_screen_leave', 'club_history_screen_leave', '0x46e560', 'club_screens.cpp', 'cdecl',
          'Ao fechar a história do clube, libera as listas dos parâmetros 0x19..0x1c e o vetor de recordes do parâmetro 5 (zerando 5 e 6).',
          'When the club history closes, frees the lists of params 0x19..0x1c and the record array of param 5 (clearing 5 and 6).')])
