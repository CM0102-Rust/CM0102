# The board request screen ("Meet with Board"): belgium_rules.cpp (reader tel30, the screens round, 21/11/2026; a new
# group from the disassembly sweep: the build and input functions are passed by address by the opener).
# Conventions checked in the disassembly (every push against esp):
# - 0x425390 board_request_screen_open cdecl(int* club id) ret 0 (called by 0x6b2370); 0x7fe600
#   ScreenManager::open_screen thiscall(0xb74450, draw 0x4253f0, input 0x4257f0, 1, 0x5a8f60, 0) ret 0x14; 0x7ff270
#   set_screen_param thiscall(index, value, 0) ret 0xc.
# - 0x4253f0 board_request_screen_draw cdecl() ret 0, SEH (a local Text, 0x4ae590 / 0x4ae630); 0x41c8e0
#   background_set_section_picture cdecl(7, 0); 0x53b640 database_get_club_kit_colours cdecl(club, 1, short* fore,
#   short* back); 0x53dba0 database_format_staff_name cdecl(chairman, buffer, 500, 1); 0x803f30 add_screen_title
#   thiscall(back, fore, name) ret 0xc (the colours pushed as dwords with the short in the low half); 0x803f80
#   add_screen_subtitle(text, 1) ret 8; 0x5e5cd0 font_get_height cdecl(3); 0x55fba0 Display::add_area thiscall on
#   [0xb74c4c + slot * 0x300] (11 dwords) ret 0x2c; 0x4ae660 Text::set_string(text) ret 4; 0x4aeaf0 Text::draw_wrapped
#   thiscall(area, 0x6e, 0x30c, &row, 3, colour, 0, 0) ret 0x20; 0x6a8770 get_board_request_option_text (ecx = the
#   manager manager, request, 0xde1f64, 2000) ret 0xc; 0x6ae180 is_board_request_available (ecx = the manager
#   manager, club, request) ret 8 -> al; 0x8040a0 add_area_button (11 dwords) ret 0x2c; 0x804100 add_area_text (9
#   dwords) ret 0x24; 0x75db00 menubar_build cdecl(4, 0).
# - 0x4257f0 board_request_screen_handle_event cdecl(short item) ret 0 -> eax; 0x6a8810
#   MANAGER_MANAGER::handle_board_request thiscall(0xb67a44 value, club, request) ret 8; 0x7615d0 / 0x7643d0
#   menubar_handle_command / menubar_handle_object_link cdecl(item).
# Identical in cm0102-1080.exe (the whole routines).
# Depends on: tel26_1 (tel26_add_area, tel26_area_button, tel26_area_text, tel26_item_id, tel26_item_value, JOB_TEXT),
#   tel29_1 (current_screen_node, screen_node_fields), tel30_2 (background_set_section_picture).
# Kept from the original (simplification noted): the colour words are passed zero-extended.
# Comparers: the build through the display list (compare_by_display_list_with, with the screen's picture path); the
#   input side by side over the parameters and the answer for a request button and Cancel (compare_resposta), the
#   original only for Confirm (the board's answer), the menu bar and a network game; the opener the original only, with
#   the plan (this screen current, the club in parameter 0, parameters 1 and 2 zero).
# Defects of the original: none found (request 6's text is fetched and not shown).
MOTOR = True
TELAS = True
ROTINAS = [('board_request_screen_open', 0x425390, [0x6a, 0x00, 0x68, 0x60, 0x8f, 0x5a, 0x00]),
           ('board_request_screen_draw', 0x4253f0, [0x6a, 0xff, 0x68, 0x0b, 0x43, 0x95, 0x00]),
           ('board_request_screen_handle_event', 0x4257f0, [0x51, 0x53, 0x56, 0x57, 0x6a, 0x00])]
_A = 'belgium_rules.cpp'
DOCS = ('a tela dos pedidos à diretoria ("Meet with Board")',
        'the board request screen ("Meet with Board")',
        [('board_request_screen_open', 'board_request_screen_open', '0x425390', _A, 'cdecl',
          'abre a tela dos pedidos à diretoria do clube (instância única); o parâmetro 0 recebe o clube e os parâmetros 1 (o pedido escolhido) e 2 (o seu botão) voltam a 0',
          "opens the club's board request screen (single instance); parameter 0 takes the club and parameters 1 (the chosen request) and 2 (its button) go back to 0"),
         ('board_request_screen_draw', 'board_request_screen_draw', '0x4253f0', _A, 'cdecl',
          'monta a tela: a foto da sala da diretoria, o nome do presidente nas cores do clube, a introdução, um botão por pedido 1 a 5 (cinza quando indisponível, amarelo quando escolhido), o menu e os botões Cancelar e Confirmar (só texto sem pedido escolhido); sem presidente fica vazia',
          'builds the screen: the boardroom picture, the chairman\'s name in the club colours, the introduction, one button per request 1 to 5 (grey when unavailable, yellow when chosen), the menu bar and the Cancel and Confirm buttons (plain text with no request chosen); without a chairman it stays empty'),
         ('board_request_screen_handle_event', 'board_request_screen_handle_event', '0x4257f0', _A, 'cdecl',
          'responde ao clique: um pedido vira o escolhido (redesenha), Confirmar entrega o pedido escolhido à diretoria e volta, Cancelar volta; o resto vai para o menu; em jogo em rede só volta',
          'answers the click: a request becomes the chosen one (redraw), Confirm hands the chosen request to the board and goes back, Cancel goes back; the rest goes to the menu bar; in a network game it just goes back')])
