# The manager stats and job information screens (manager_screens.cpp; reader tel26, the screens round, 21/11/2026).
# Uses the helpers of tel26_1. The Manager Stats builder (0x6ae2a0) is item tel26_5.
# Conventions checked in the disassembly: the opens and builders cdecl(void); the handlers cdecl(short item) -> eax;
# the closers cdecl(void). ScreenManager::open_screen 0x7fe600 thiscall(SM, builder, handler, 0, closer, 0);
# get/set_screen_param 0x7ff010/0x7ff270; get_area_scroll_row 0x803320 thiscall(SM, area); add_screen_title 0x803f30
# thiscall(SM, colour, text colour, text); add_screen_subtitle 0x803f80 thiscall(SM, text, 0); add_button_item
# 0x804030 (12 fields), add_area_button 0x8040a0 (11), add_area_text 0x804100 (9) thiscall on SM; Display::add_item
# 0x55f990 (18) / add_area 0x55fba0 (11) thiscall on the slot's Display; NATION_MENU_SPLIT 0x66a280 thiscall(obj, 0,
# order, 1), 0x66a350 (obj) -> al, 0x66a430 (obj, nation) -> al, 0x66a360 (obj, text, 0x7d0, part); key_nation_sorted
# _order 0x66a0c0 cdecl(1); gui_add_page_buttons 0x5ede20 cdecl(0x30c, 0x91, page, pages, 0, 2, 3, 0);
# gui_add_text_area 0x5ed070 cdecl (11); Area::set_scroll_row 0x403320 thiscall(area, row).
# The colour words pushed with a garbage upper half are passed zero-extended; the known upper halves (the item index
# under the part area's colour, the manager id under his colour, the nation parameter under the rows byte of the
# list area) are reproduced. The original's part area local starts uninitialised (always set before use).
# Defects of the original kept: the B-team flag of club_get_linked_b_team is never reset between clubs (once a club
# sets it, the next B teams pass); a failed realloc loses the old list and returns without the Back/Next buttons.
MOTOR = True
TELAS = True
ROTINAS = [('open_manager_stats_screen', 0x6ae1a0, [0x6a, 0x00, 0x68, 0xd0, 0x09, 0x6b, 0x00]),
           ('manager_stats_screen_handle_action', 0x6b05e0, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('manager_stats_screen_close', 0x6b09d0, [0x6a, 0x01, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('open_job_information_screen', 0x6b0a10, [0x6a, 0x00, 0x68, 0x80, 0x19, 0x6b, 0x00]),
           ('job_information_screen_build', 0x6b0b10, [0x81, 0xec, 0x08, 0x03, 0x00, 0x00]),
           ('job_information_screen_handle_action', 0x6b1790, [0x51, 0x53, 0x56, 0x57, 0x6a, 0x03]),
           ('job_information_screen_close', 0x6b1980, [0x6a, 0x01, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('as telas do técnico (1): abrir, responder e fechar a de estatísticas dos técnicos; a de informação de vagas inteira',
        'the manager screens (1): opening, answering and closing the manager stats screen; the whole job information screen',
        [('open_manager_stats_screen', 'open_manager_stats_screen', '0x6ae1a0', 'manager_screens.cpp', 'cdecl',
          'abre a tela de estatísticas dos técnicos na nação do técnico humano (a do clube, a da seleção, ou a dele se escolhida, senão todas) e grava os nove parâmetros',
          'opens the manager stats screen on the human manager\'s nation (his club\'s, his national team\'s, or his own when chosen, else all) and sets the nine parameters'),
         ('manager_stats_screen_handle_action', 'manager_stats_screen_handle_action', '0x6b05e0', 'manager_screens.cpp', 'cdecl',
          'responde à tela: guarda a rolagem, troca a nação ou a estatística (liberando a lista e voltando à página 1), anda 1 ou 10 páginas, ou passa à barra de menus',
          'answers the screen: keeps the scroll, changes the nation or the statistic (freeing the list, back to page 1), moves 1 or 10 pages, or goes to the menu bar'),
         ('manager_stats_screen_close', 'manager_stats_screen_close', '0x6b09d0', 'manager_screens.cpp', 'cdecl',
          'ao fechar, libera a lista da tela',
          'on close, frees the screen list'),
         ('open_job_information_screen', 'open_job_information_screen', '0x6b0a10', 'manager_screens.cpp', 'cdecl',
          'abre a tela de informação de vagas na nação do clube do técnico humano, ou na dele, ou na da seleção dele (se escolhidas), senão a internacional',
          'opens the job information screen on the nation of the human manager\'s club, or his own, or his national team\'s (when chosen), else the international one'),
         ('job_information_screen_build', 'job_information_screen_build', '0x6b0b10', 'manager_screens.cpp', 'cdecl',
          'monta a tela de informação de vagas: menu de nações, a lista dos clubes da nação (ou das seleções) com técnico, nação, divisão e segurança no cargo (de Muito Inseguro a Muito Seguro, ou Vaga), páginas acima de 100 clubes e a rolagem até o clube do humano',
          'builds the job information screen: the nation menu, the clubs of the nation (or the national teams) with manager, nation, division and job security (Very Insecure to Very Secure, or Available), pages above 100 clubs and the scroll to the human\'s club'),
         ('job_information_screen_handle_action', 'job_information_screen_handle_action', '0x6b1790', 'manager_screens.cpp', 'cdecl',
          'responde à tela: guarda a rolagem, troca a nação (liberando a lista), anda uma página, ou passa à barra de menus',
          'answers the screen: keeps the scroll, changes the nation (freeing the list), moves a page, or goes to the menu bar'),
         ('job_information_screen_close', 'job_information_screen_close', '0x6b1980', 'manager_screens.cpp', 'cdecl',
          'ao fechar, libera a lista e volta à página 1',
          'on close, frees the list and goes back to page 1')])
