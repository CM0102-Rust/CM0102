# The physio report screen (physio.cpp; reader tel36, the screens round, 21/11/2026). Uses tel36_1 (tel36_staff_param,
# tel36_langlib), tel26_1 (tel26_add_area, tel26_area_button, tel26_text_area, tel26_item_id, tel26_item_value,
# job_colour), tel26_4 (manager_menu_bar) and tel35_1 (tel35_text_draw_wrapped, TEL35_NETWORK_FLAG).
# Conventions checked in the disassembly: the open cdecl(Staff* player, Staff* physio); the builder cdecl(void) with an
# SEH frame (a local Text: Text::Text 0x4ae590 / ~Text 0x4ae630); background_set_staff_picture 0x41c790 cdecl(person,
# 0, 0); Text::draw_wrapped 0x4aeaf0 thiscall ret 0x20; the handler cdecl(short) -> eax; physio_file_report_news
# 0x7a1a90 thiscall(0xb74344, physio, player) ret 8.
# With exactly one physio the question names the physio of parameter 1, not the one found at the club (kept).
# Equal in cm0102-1080.exe.
# Comparers: the builder side by side over the display list; the handler's physio choice side by side over the screen
# params; the open, the report and the menu bar only the original.
TELAS = True
ROTINAS = [('physio_report_screen_open', 0x7a2a50, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c]),
           ('physio_report_screen_build', 0x7a2ab0, [0x6a, 0xff, 0x68, 0x9b, 0xf0, 0x95, 0x00]),
           ('physio_report_screen_handle_click', 0x7a2f10, [0x53, 0x55, 0x56, 0x57, 0x6a, 0x00])]
DOCS = ('a tela de pedir o relatório do fisioterapeuta',
        'the request physio report screen',
        [('physio_report_screen_open', 'physio_report_screen_open', '0x7a2a50', 'physio.cpp', 'cdecl',
          'abre a tela para um jogador e um fisioterapeuta (os dois presentes); parâmetros: os dois ids',
          'opens the screen for a player and a physio (both present); params: the two ids'),
         ('physio_report_screen_build', 'physio_report_screen_build', '0x7a2ab0', 'physio.cpp', 'cdecl',
          'monta a tela: o nome do jogador nas cores do clube, "Request Physio Report"; com um fisioterapeuta só, a frase de confirmar o relatório dele (o do parâmetro 1) sobre o jogador; com vários (ou nenhum), a frase com o link do jogador e um botão por fisioterapeuta do clube (até 3, os com ficha), o escolhido destacado; Cancelar / Confirmar; sem clube não monta nada',
          'builds the screen: the player\'s name in the club colours, "Request Physio Report"; with one physio, the sentence confirming his report (the one of parameter 1) on the player; with several (or none), the sentence with the player\'s link and one button per club physio (up to 3, those with a record), the chosen one highlighted; Cancel / Confirm; with no club nothing is built'),
         ('physio_report_screen_handle_click', 'physio_report_screen_handle_click', '0x7a2f10', 'physio.cpp', 'cdecl',
          'um fisioterapeuta clicado vira o escolhido (parâmetro 1, -11 para remontar); Confirmar gera a notícia do relatório do escolhido sobre o jogador e fecha; Cancelar fecha; no jogo em rede sempre fecha; o resto vai para a barra de menus',
          'a clicked physio becomes the chosen one (parameter 1, -11 to rebuild); Confirm files the chosen physio\'s report news on the player and closes; Cancel closes; in a network game it always closes; the rest goes to the menu bar')])
