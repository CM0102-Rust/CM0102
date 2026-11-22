# The Football Association meeting screen (fifa_confed.cpp; reader tel34, the screens round, 21/11/2026: the board
# confidence screen's "FA Request" button opens it, tel26_4 calls 0x58acd0 by address). Uses the helpers of tel26_1
# and tel34_1 (tel34_lang_text): goes after tel34_1.
# Conventions checked in the disassembly: the open cdecl(club*); the builder cdecl(void) with an SEH frame and a local
# Text (Text::Text 0x4ae590 thiscall(t, 0), Text::~Text 0x4ae630 on every exit); the handler cdecl(short item) -> eax.
# Text::set_string 0x4ae660; Text::draw_wrapped 0x4aeaf0 thiscall(t, area, 0x6e, 0x30c, &line, 3, colour, 0, 0) ret
# 0x20; get_fa_request_option_text 0x6a9e20 called with ecx = [0xb67a44] and (request, text, 0x7d0) ret 0xc;
# ScreenManager::add_area_button 0x8040a0 (11); ScreenManager::set_widget_text_colour 0x801960 thiscall(SM, widget,
# colour) ret 8; ScreenManager::redraw_screen 0x802d00 (SM, 0, 0); MANAGER_MANAGER::handle_fa_request 0x6a9e60
# thiscall([0xb67a44], club, request) ret 8. The rows byte goes to add_area in a dword whose high bytes are the frame's
# (zero here; the area keeps a byte).
TELAS = True
ROTINAS = [('fa_meeting_open_screen', 0x58acd0, [0x6a, 0x00, 0x68, 0x60, 0x8f, 0x5a, 0x00]),
           ('fa_meeting_screen_draw', 0x58ad30, [0x6a, 0xff, 0x68, 0x3b, 0x8d, 0x95, 0x00]),
           ('fa_meeting_screen_handle_event', 0x58b0f0, [0x51, 0x53, 0x56, 0x57, 0x6a, 0x00])]
DOCS = ('a reunião com a federação: abrir, a tela e as escolhas',
        'the Football Association meeting: opening it, the screen and the choices',
        [('fa_meeting_open_screen', 'fa_meeting_open_screen', '0x58acd0', 'fifa_confed.cpp', 'cdecl',
          'abre a tela da reunião com a federação para o clube (parâmetro 0 = o índice do clube; 1 e 2 zerados)',
          'opens the FA meeting screen for the club (parameter 0 = the club index; 1 and 2 cleared)'),
         ('fa_meeting_screen_draw', 'fa_meeting_screen_draw', '0x58ad30', 'fifa_confed.cpp', 'cdecl',
          'monta a tela: o técnico do clube no título, a explicação quebrada em linhas, os dois pedidos (reclamar do árbitro, adiar um jogo) como botões, o escolhido destacado, e Cancelar e Confirmar; sem técnico, nada',
          "builds the screen: the club's manager in the title, the wrapped explanation, the two requests (referee complaint, postponing a match) as buttons, the chosen one highlighted, and Cancel and Confirm; with no manager, nothing"),
         ('fa_meeting_screen_handle_event', 'fa_meeting_screen_handle_event', '0x58b0f0', 'fifa_confed.cpp', 'cdecl',
          'um pedido escolhido troca as cores dos dois botões, guarda a escolha e redesenha (0); Confirmar manda o pedido escolhido à federação e fecha; Cancelar fecha; com a marca do diálogo (0xb77c84) zera o argumento pendente e fecha',
          "picking a request swaps the two buttons' colours, keeps the choice and redraws (0); Confirm sends the chosen request to the FA and closes; Cancel closes; with the dialog flag (0xb77c84) set it clears the pending argument and closes")])
