# The search screens (4): the search edit session's destructor, closing the Player and Staff Search screen, opening
# the Filters screen and the Filters input (search_screens.cpp; reader tel21, the screens round, 21/11/2026).
# Uses tel26_1 (tel26_error_box, tel26_item_id, tel26_item_value): goes after it. The Filters builder 0x8133a0 is
# tel21_5 (it uses this item's search_edit_session, search_filters_fields and search_session_current).
# Conventions checked in the disassembly: 0x813140 thiscall(session) ret 0 (SEH frame; LLIST::~LLIST 0x68d040
# thiscall, the CRT vector destructor iterator 0x9462bb stdcall(array, 0x2b0, 10, PLAYER_SEARCH::~PLAYER_SEARCH
# 0x7ae4f0)); 0x8131c0 cdecl() ret 0 (the leave function of the Player and Staff Search screen, a pointer in 0x808540;
# SearchEditSession::save_all_slots 0x804450 thiscall, operator delete, SCOUT_MANAGER::set_edit_session 0x7f6af0
# thiscall(manager, 0, 0) ret 8, already Rust); 0x813210 cdecl(session) ret 0 (called from the search input 0x80eb00;
# ScreenManager::open_screen(0x8133a0, 0x818100, 1, 0, 0), operator new(0x99), SearchFilters::set_defaults 0x806450
# thiscall, a 0x99-byte copy of the current search's block at +0x1fa); 0x818100 cdecl(short item) ret 0 -> eax (the
# Filters screen's handler, a pointer in 0x813210; the SearchFilters methods 0x8064d0..0x808500 thiscall as in the
# code's constants, search_screens_filter_index_to_bit 0x81b4e0 cdecl -> edx:eax, ScreenManager::get_widget_action_id
# 0x468b40 thiscall ret 4, menubar_handle_object_link 0x7643d0 cdecl).
# Comparers: the destructor and the close only run the original (they free); the open runs the original and checks the
# plan (param 1 a copy of the current search's filter block, param 2 its page); the input runs side by side over the
# filters (0x99 bytes) and the screen params for the answers that only change them, the original only for Cancel, Ok,
# the option buttons (id 0xa) and the menu bar.
# DEFECT of the original kept: 0x813210 copies the filters into address 0 when the 0x99-byte allocation fails.
TELAS = True
ROTINAS = [('search_edit_session_destructor', 0x813140, [0x6a, 0xff, 0x68, 0x46, 0x12, 0x96, 0x00]),
           ('search_screens_close_player_search_screen', 0x8131c0, [0x56, 0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('search_screens_open_filters_screen', 0x813210, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('search_screens_handle_filters_input', 0x818100, [0x51, 0x53, 0x55, 0x56, 0x57])]
DOCS = ('as telas de busca (4): a sessão de edição da busca, abrir e fechar as telas e a entrada da tela de filtros',
        'the search screens (4): the search edit session, opening and closing the screens and the Filters input',
        [('search_edit_session_destructor', 'SearchEditSession::~SearchEditSession', '0x813140', 'search_screens.cpp', 'thiscall',
          'destrói a sessão de edição da busca: a lista (+0x361e) e os dois vetores de dez buscas (as salvas e as editadas)',
          "destroys the search edit session: its list (+0x361e) and both ten-search arrays (the saved and the edited ones)"),
         ('search_screens_close_player_search_screen', 'search_screens_close_player_search_screen', '0x8131c0', 'search_screens.cpp', 'cdecl',
          'fecha a tela de busca de jogadores e comissão: com sessão, grava todas as buscas de volta, destrói e libera a sessão, zera o parâmetro 0 e tira a sessão do gerente de olheiros',
          'closes the Player and Staff Search screen: with a session, saves every search back, destroys and frees it, clears param 0 and removes the session from the scout manager'),
         ('search_screens_open_filters_screen', 'search_screens_open_filters_screen', '0x813210', 'search_screens.cpp', 'cdecl',
          'abre a tela de filtros da busca atual: sem sessão, a caixa de erro (linha 0x100c); aberta a tela, os parâmetros são a sessão, uma cópia dos filtros da busca atual (0x99 bytes) e a última página de filtros dela',
          "opens the Filters screen of the current search: without a session, the error box (line 0x100c); once open, the params are the session, a copy of the current search's filters (0x99 bytes) and its last Filters page"),
         ('search_screens_handle_filters_input', 'search_screens_handle_filters_input', '0x818100', 'search_screens.cpp', 'cdecl',
          'trata a entrada da tela de filtros: troca de página, limpar (geral ou atributos), ligar e desligar um atributo (1 <-> 10), o nível do atributo (10, 15, 20, 1), o mínimo um abaixo ou acima, quantos atributos têm de bater, Cancelar (libera os filtros), Ok (os filtros vão para a busca, que guarda a página e fica marcada como mudada), as opções, a nação e a região da nacionalidade e da base, o idioma, o cargo e a faixa de valor; o resto vai para a barra de menus',
          "handles the Filters screen's input: the page toggle, Clear (general or attributes), an attribute on/off (1 <-> 10), the attribute level (10, 15, 20, 1), the minimum one down or up, how many attributes must match, Cancel (frees the filters), Ok (the filters go to the search, which keeps the page and is marked changed), the options, the nationality and base nation and region, the language, the job and the value band; the rest goes to the menu bar")])
