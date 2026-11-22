# The Discipline Player screen (Fine.cpp; reader tel35, the screens round, 21/11/2026). Uses the helpers of tel26_1
# (tel26_add_area, tel26_area_button, tel26_area_text, tel26_item_id, tel26_item_value, tel26_copy_text, JOB_TEXT)
# and tel26_4 (manager_menu_bar).
# Conventions checked in the disassembly: the open cdecl(Staff*) (open_screen thiscall(SM, builder, handler, 1,
# 0x5a8f60, 0) ret 0x14; set_screen_param thiscall(SM, k, v, 0) ret 0xc); the builder cdecl(void) with an SEH frame
# (a local Text built by Text::Text 0x4ae590 and destroyed by 0x4ae630); the handler cdecl(short) -> eax.
# background_set_staff_picture 0x41c790 cdecl(person, 0, 0); database_get_club_kit_colours 0x53b640 cdecl(club, 1,
# &fore, &back) (shorts); database_format_staff_name 0x53dba0 cdecl(person, out, 500, 1); add_screen_title 0x803f30
# thiscall(SM, back, fore, text) ret 0xc; add_screen_subtitle 0x803f80 thiscall(SM, text, 1) ret 8; font_get_height
# 0x5e5cd0 cdecl(3); Display::add_area 0x55fba0 thiscall(slot display, 11) ret 0x2c; Text::set_string 0x4ae660
# thiscall ret 4; Text::add_link 0x4ae8a0 thiscall(text, phrase, 0x7d7, person) ret 0xc; Text::draw_wrapped 0x4aeaf0
# thiscall(text, area, 0x6e, 0x30c, &line, 3, colour, 0, 0) ret 0x20; add_area_button 0x8040a0 (11) ret 0x2c and
# add_area_text 0x804100 (9) ret 0x24 thiscall on SM; Fine::reason_text 0x5a73a0 (k, out, 2000, 0) ret 0x10 (ecx =
# [0xae2a3c]); langlib_get_language 0x66f4d0; menubar_build cdecl(4, 0); set_widget_flags 0x800580,
# set_widget_text_colour 0x801960, set_widget_data 0x8022d0, set_widget_text 0x801c70, set_widget_id 0x801fc0 thiscall
# (SM, widget, v) ret 8; redraw_screen 0x802d00 thiscall(SM, 0, 0) ret 8; Fine::fine_player 0x5a5980 thiscall(fine,
# person, period, reason) ret 0xc; menubar_handle_command 0x7615d0 / menubar_handle_object_link 0x7643d0 cdecl(raw).
# Equal in cm0102-1080.exe.
# Defects of the original: the builder takes the highlighted reason from parameter 2 (the "-" widget it writes there)
# instead of parameter 3 (the reason the handler keeps): on a redraw the yellow reason is the "-" widget's low byte;
# kept.
# Comparers: the builder side by side over the display list; the open and the handler (they open the screen, fine the
# person, redraw or run the menu bar) only in the original.
MOTOR = True
TELAS = True
ROTINAS = [('open_discipline_player_screen', 0x5a8380, [0x6a, 0x00, 0x68, 0x60, 0x8f, 0x5a, 0x00]),
           ('draw_discipline_player_screen', 0x5a83e0, [0x6a, 0xff, 0x68, 0x2b, 0x94, 0x95, 0x00]),
           ('discipline_player_screen_input', 0x5a8990, [0x83, 0xec, 0x08, 0x53, 0x55])]
DOCS = ('a tela de disciplinar um jogador',
        'the Discipline Player screen',
        [('open_discipline_player_screen', 'open_discipline_player_screen', '0x5a8380', 'Fine.cpp', 'cdecl',
          'abre a tela de disciplinar a pessoa com o período em 1 (advertência) e o motivo em 0',
          'opens the screen to discipline the person with the period at 1 (a warning) and the reason at 0'),
         ('draw_discipline_player_screen', 'draw_discipline_player_screen', '0x5a83e0', 'Fine.cpp', 'cdecl',
          'monta a tela: o nome da pessoa nas cores do clube, a frase com o link para ela, o seletor do período ("-", advertência ou multa de n semanas, "+"; "-" apagado na advertência e "+" nas duas semanas), os cinco motivos (o escolhido em amarelo, lido do parâmetro 2: defeito do original) e Cancelar / Confirmar; sem clube não monta nada',
          'builds the screen: the person\'s name in his club\'s colours, the sentence with his link, the period chooser ("-", warning or an n-week fine, "+"; "-" greyed at the warning and "+" at two weeks), the five reasons (the chosen one yellow, read from parameter 2: a defect of the original) and Cancel / Confirm; with no club nothing is built'),
         ('discipline_player_screen_input', 'discipline_player_screen_input', '0x5a8990', 'Fine.cpp', 'cdecl',
          '"-" e "+" mudam o período entre a advertência e duas semanas (apagando o botão no limite), um motivo fica amarelo e é guardado, Confirmar multa a pessoa e fecha, Cancelar fecha; no jogo em rede sempre fecha; o resto vai para a barra de menus',
          '"-" and "+" move the period between the warning and two weeks (greying the button at the end), a reason turns yellow and is kept, Confirm fines the person and closes, Cancel closes; in a network game it always closes; the rest goes to the menu bar')])
