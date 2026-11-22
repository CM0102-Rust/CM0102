# The coach report request screen ("Request Coach Report"): coach.cpp (reader tel30, the screens round, 21/11/2026; a
# new group from the disassembly sweep: the build and input functions are passed by address by the opener).
# Conventions checked in the disassembly (every push against esp):
# - 0x48bc00 coach_open_report_request_screen cdecl(player, coach) ret 0 (called by the staff profile action 0x888260
#   and the compare players action 0x889c30, command 0x2a); 0x7fe600 open_screen(0x48bd20, 0x48c290, 1, 0x5a8f60, 0)
#   ret 0x14; 0x540a50 club_get_linked_b_team cdecl(club, &senior, 1); 0x7ff270 set_screen_param(index, value, 0).
# - 0x48bd20 coach_report_request_draw cdecl() ret 0, SEH (a local Text): 0x41c8e0 background_set_section_picture
#   cdecl(6, 0); 0x53b640 database_get_club_kit_colours cdecl(club, 1, &fore, &back); 0x53dba0
#   database_format_staff_name cdecl(.., 500 / 0x96, 1); 0x803f30 add_screen_title(back, fore, name) ret 0xc; 0x803f80
#   add_screen_subtitle(text, 1) ret 8; 0x804030 add_button_item (12 dwords) ret 0x30; 0x5e5cd0 font_get_height
#   cdecl(3); 0x5ed070 gui_add_text_area cdecl (11 dwords); 0x55fba0 Display::add_area on [0xb74c4c + slot * 0x300]
#   ret 0x2c; 0x4ae660 Text::set_string ret 4; 0x4ae8a0 Text::add_link thiscall(text, phrase, 0x7d7, player) ret 0xc;
#   0x4aeaf0 Text::draw_wrapped ret 0x20; 0x8040a0 add_area_button ret 0x2c; 0x53ad00 database_get_job_name cdecl(job,
#   0, 1, 0) (two pushes of the next button, the bit and 1, go before it and stay on the stack); 0x804100 add_area_text
#   ret 0x24; 0x75db00 menubar_build cdecl(4, 0).
# - 0x48c290 coach_report_request_handle_action cdecl(short item) ret 0 -> eax; 0x487c70 coach_post_report_news (ecx =
#   0xadadf4, coach, player) ret 8; 0x7615d0 / 0x7643d0 menubar_handle_command / menubar_handle_object_link cdecl(item).
# Identical in cm0102-1080.exe (the whole routines).
# Depends on: tel26_1 (tel26_add_area, tel26_add_button, tel26_area_button, tel26_area_text, tel26_text_area,
#   tel26_item_id, tel26_item_value, JOB_TEXT), tel29_1 (current_screen_node, screen_node_fields), tel30_2
#   (background_set_section_picture).
# Kept from the original (simplification noted): the colour words are passed zero-extended.
# Comparers: the build through the display list (compare_by_display_list_with, with the screen's picture path); the
#   input side by side over the parameters and the answer for a coach, Select All and Cancel (compare_resposta), the
#   original only for Confirm (the reports go out), the menu bar and a network game; the opener the original only with
#   the plan (this screen current, the player in parameter 0, the coach's bit in parameter 1).
# Defects of the original: none found (the list writes each coach's name over the player's name buffer).
MOTOR = True
TELAS = True
ROTINAS = [('coach_open_report_request_screen', 0x48bc00, [0x83, 0xec, 0x1c, 0x53, 0x55]),
           ('coach_report_request_draw', 0x48bd20, [0x6a, 0xff, 0x68, 0xfb, 0x5f, 0x95, 0x00]),
           ('coach_report_request_handle_action', 0x48c290, [0x83, 0xec, 0x24, 0x53, 0x55])]
_A = 'coach.cpp'
DOCS = ('a tela do pedido de relatório aos treinadores ("Request Coach Report")',
        'the coach report request screen ("Request Coach Report")',
        [('coach_open_report_request_screen', 'coach_open_report_request_screen', '0x48bc00', _A, 'cdecl',
          'abre a tela do pedido de relatório sobre um jogador a partir do perfil de um treinador: os candidatos são o técnico e os cinco auxiliares do clube dele (com ficha de não jogador e que não sejam o próprio jogador); o parâmetro 1 recebe o bit do treinador entre eles e o 2 o clube (o principal, para um time B)',
          "opens the report request screen on a player from a coach's profile: the candidates are the manager and the five coaches of his club (with a non-player record and not the player himself); parameter 1 takes the coach's bit among them and 2 the club (the senior side, for a B team)"),
         ('coach_report_request_draw', 'coach_report_request_draw', '0x48bd20', _A, 'cdecl',
          'monta a tela: a foto do treino, o nome do jogador nas cores do clube, "Select All"; com um só candidato a pergunta "Pedir a ... um relatório sobre ...?", senão a pergunta com link no jogador e uma linha por candidato (nome e cargo, amarelo quando escolhido); o menu, Cancelar e Confirmar (só texto sem escolhido)',
          "builds the screen: the training picture, the player's name in the club colours, \"Select All\"; with a single candidate the question \"Ask ... to file a report on ...?\", otherwise the question with a link on the player and one row per candidate (name and job, yellow when chosen); the menu bar, Cancel and Confirm (plain text with none chosen)"),
         ('coach_report_request_handle_action', 'coach_report_request_handle_action', '0x48c290', _A, 'cdecl',
          'responde ao clique: um treinador troca o seu bit, "Select All" marca todos (ambos redesenham), Confirmar pede o relatório a cada escolhido e volta, Cancelar volta; o resto vai para o menu; em jogo em rede só volta',
          "answers the click: a coach flips its bit, \"Select All\" marks them all (both redraw), Confirm asks each chosen one for the report and goes back, Cancel goes back; the rest goes to the menu bar; in a network game it just goes back")])
