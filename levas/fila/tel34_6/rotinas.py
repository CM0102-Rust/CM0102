# The Search For screen and the friendly teams' other clubs input (find_screens.cpp; reader tel34, the screens
# round, 21/11/2026: the menu bar opens open_find_screen 0x5a2c30, the rest is passed by pointer). Uses the helpers of
# tel26_1 and tel34_1 (tel34_lang_text, tel34_game_title, Tel34FormatFn): goes after tel34_1.
# Conventions checked in the disassembly: every routine cdecl; open_find_screen(char tab); draw_find_screen(void) with
# an SEH frame, _chkstk 0x14bc and 25 MenuItems built and destroyed by the EH vector iterators (ctor 0x4568f0, dtor the
# empty 0x5a53c0); the four result tabs (int top, int bottom) with an SEH frame (operator new of the SearchEngine);
# the inputs (short item) -> eax (the tab inputs return 0 when they do not handle the item); release_find_screen(void).
# SearchEngine::SearchEngine 0x804f00 thiscall -> this; find_clubs 0x804fe0 / find_staff 0x805310 thiscall(engine,
# text, mode) ret 8; ~SearchEngine 0x804fd0; gui_add_page_buttons 0x5ede20 cdecl (8); gui_add_tab_bar 0x5ed780;
# ScreenManager::add_text_item 0x803fd0 (11); human_manager_is_human_club 0x600a50 cdecl(club, 0, 1, 0, 0);
# database_get_national_team 0x53b3d0; background_set_nation/club/staff_picture 0x41c100/0x41c250/0x41c790 cdecl(0, 0,
# 0); Display::get_widget_id 0x468b10, ScreenManager::get_widget_action_id 0x468b40 and get_widget_action_data
# 0x468b90 read the same item fields as tel26_item_id / tel26_item_value (the id 0 or item -1 giving the command id).
# The four result routines share one body (tel34_find_results; the same instructions with each tab's parameters, edit
# box kind/id, search and strings), and the four tab inputs one (tel34_find_tab_input): the nations' and clubs' are
# inline in find_screen_input, the non-players' and players' are their own routines.
# Left out: shared_noop_ret0_5a53c0 (1 byte, a bare ret: no room for the 5-byte detour).
# The 1080 exe (resolucao.py) rewrites the paging of the four result tabs and the three inputs for 27 rows per page
# (0x1b and 0x10e in place of 0x10 and 0xa0, code caves at 0x601f28/0x601f38): those seven entries are for the original
# exe only (EXE); draw_find_screen calls the result tabs by address, so in the 1080 exe its own versions run.
TELAS = True
EXE = {0x5a30f0: 'original', 0x5a35b0: 'original', 0x5a3af0: 'original', 0x5a4060: 'original', 0x5a45c0: 'original',
       0x5a4950: 'original', 0x5a4c00: 'original'}
ROTINAS = [('choose_friendly_teams_other_clubs_input', 0x5a2b30, [0x56, 0x66, 0x8b, 0x74, 0x24, 0x08]),
           ('open_find_screen', 0x5a2c30, [0x83, 0xec, 0x1c, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('draw_find_screen', 0x5a2e60, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('draw_find_nations_results', 0x5a30f0, [0x6a, 0xff, 0x68, 0x2b, 0x92, 0x95, 0x00]),
           ('draw_find_clubs_results', 0x5a35b0, [0x6a, 0xff, 0x68, 0x4b, 0x92, 0x95, 0x00]),
           ('draw_find_non_players_results', 0x5a3af0, [0x6a, 0xff, 0x68, 0x6e, 0x92, 0x95, 0x00]),
           ('draw_find_players_results', 0x5a4060, [0x6a, 0xff, 0x68, 0x8e, 0x92, 0x95, 0x00]),
           ('find_screen_input', 0x5a45c0, [0x53, 0x56, 0x57, 0x6a, 0x00]),
           ('find_screen_non_players_input', 0x5a4950, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('find_screen_players_input', 0x5a4c00, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('release_find_screen', 0x5a4eb0, [0x56, 0x6a, 0x02, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('a tela de busca: abrir, montar as quatro abas (seleções, clubes, não jogadores, jogadores), os cliques e liberar as buscas; e o clique na aba Outros clubes dos amistosos',
        'the Search For screen: opening it, building its four tabs (nations, clubs, non-players, players), the clicks and freeing the searches; and the click in the friendlies\' Other clubs tab',
        [('choose_friendly_teams_other_clubs_input', 'choose_friendly_teams_other_clubs_input', '0x5a2b30', 'find_screens.cpp', 'cdecl',
          'guarda a rolagem da lista e, num clube (id 7) diferente do guardado no parâmetro 0x10, guarda-o e pede remontagem (-11); senão 0',
          'keeps the list scroll and, on a club (id 7) other than the one in parameter 0x10, keeps it and asks for a rebuild (-11); otherwise 0'),
         ('open_find_screen', 'open_find_screen', '0x5a2c30', 'find_screens.cpp', 'cdecl',
          'abre a busca numa aba e zera as quatro abas: texto vazio, sem busca feita, "buscar de novo" ligado, primeira linha, rolagem e área zeradas',
          'opens the search on a tab and resets the four tabs: empty text, no search made, "search again" on, first row, scroll and area cleared'),
         ('draw_find_screen', 'draw_find_screen', '0x5a2e60', 'find_screens.cpp', 'cdecl',
          'monta a busca: título, menu, as abas Nation, Club, Non-Player e Player, os resultados da aba aberta (ou uma área vazia) e Voltar/Avançar',
          'builds the search: title, menu bar, the Nation, Club, Non-Player and Player tabs, the open tab\'s results (or an empty area) and Back/Next'),
         ('draw_find_nations_results', 'draw_find_nations_results', '0x5a30f0', 'find_screens.cpp', 'cdecl',
          'a aba das seleções: a caixa de busca, a busca quando pedida, os botões de página (mais de 100 achados: 16 por página), o título e cada seleção achada com o continente e a federação',
          'the nations tab: the search box, the search when asked, the page buttons (over 100 found: 16 per page), the heading and each national team found with its continent and federation'),
         ('draw_find_clubs_results', 'draw_find_clubs_results', '0x5a35b0', 'find_screens.cpp', 'cdecl',
          'a aba dos clubes: como a das seleções; cada clube com a nação (ligada à seleção) e a divisão (ligada quando carregada; "uma divisão inferior" na liga escolhida e não carregada)',
          'the clubs tab: like the nations tab; each club with its nation (linked to the national team) and division (linked when loaded; "a lower division" in a selected league not loaded)'),
         ('draw_find_non_players_results', 'draw_find_non_players_results', '0x5a3af0', 'find_screens.cpp', 'cdecl',
          'a aba dos não jogadores: cada pessoa achada com o clube (o emprego na seleção quando é o principal) e a divisão; sem clube, "No Club" e "No Division"',
          'the non-players tab: each person found with his club (his national job when it is his main one) and division; with no club, "No Club" and "No Division"'),
         ('draw_find_players_results', 'draw_find_players_results', '0x5a4060', 'find_screens.cpp', 'cdecl',
          'a aba dos jogadores: cada jogador achado com o clube e a divisão',
          'the players tab: each player found with his club and division'),
         ('find_screen_input', 'find_screen_input', '0x5a45c0', 'find_screens.cpp', 'cdecl',
          'o clique na busca: primeiro a aba aberta (buscar, páginas de 16 e de 160), depois um botão de aba diferente abre a aba (-11); senão a barra de menu',
          'a click on the search: the open tab first (search, pages of 16 and of 160), then a different tab button opens that tab (-11); otherwise the menu bar'),
         ('find_screen_non_players_input', 'find_screen_non_players_input', '0x5a4950', 'find_screens.cpp', 'cdecl',
          'o clique na aba dos não jogadores: guarda a rolagem; buscar de novo ou mudar de página (-11); senão 0',
          'a click in the non-players tab: keeps the scroll; search again or change page (-11); otherwise 0'),
         ('find_screen_players_input', 'find_screen_players_input', '0x5a4c00', 'find_screens.cpp', 'cdecl',
          'o clique na aba dos jogadores: guarda a rolagem; buscar de novo ou mudar de página (-11); senão 0',
          'a click in the players tab: keeps the scroll; search again or change page (-11); otherwise 0'),
         ('release_find_screen', 'release_find_screen', '0x5a4eb0', 'find_screens.cpp', 'cdecl',
          'ao sair da busca, libera as buscas das quatro abas (parâmetros 2, 8, 0xe e 0x14) e zera os parâmetros',
          'leaving the search frees the four tabs\' searches (parameters 2, 8, 0xe and 0x14) and clears the parameters')])
