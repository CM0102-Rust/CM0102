# The menu bar (Menubar.cpp; reader tel26, the screens round, 21/11/2026).
# - 0x75db00 menubar_build (14176 bytes): cdecl(char mode, int context), ret 0, no SEH; the mode is read as a byte and
#   rewritten in the argument slot (4 = redraw becomes the record's mode or 2). Conventions checked at the call sites:
#   Display::add_item 0x55f990 thiscall(display, 18) ret 0x48 and add_area 0x55fba0 thiscall(display, 11) ret 0x2c on
#   [0xb74c4c + 0x300 x slot]; ScreenManager (ecx 0xb74450): add_area_text 0x804100 (9) ret 0x24, add_area_button
#   0x8040a0 (11) ret 0x2c, add_menu_item 0x415640 (4) ret 0x10, add_menu_area 0x415600 (1), count_flagged_screens
#   0x803d50 (AX), get_current_screen_flag 0x7fed30, current_screen_has_exit_fn 0x7fedd0, is_manager_done 0x7ffe40 (1),
#   send_clear_click 0x800000, count_active_records 0x7feb70 (AX, signed), set_pending_click_widget 0x800160 (1),
#   is_back_unavailable 0x7fec60 (1), is_forward_unavailable 0x7febb0; menubar_add_separator 0x761260 (the map says
#   stdcall(area) ret 4; ecx is loaded with the screen manager, called as thiscall); NATION_MENU_SPLIT (a 14-byte
#   local): ctor 0x66a280 (3) ret 0xc, part_count 0x66a350 (AL), in_second_part 0x66a430 (1, AL), format_header
#   0x66a360 (3) ret 0xc; HUMAN_MANAGER [0xb63c98]: is_on_holiday 0x5fd210 (1), controls_all_national_teams 0x5fd6b0
#   (2) ret 8, controls_reserve_team 0x5fd480 (1), set_holiday_stamp 0x5fdd30 (2) ret 8; News [0xb7431c]
#   count_unread 0x786fe0 (2); AWARD_MANAGER [0xad9c60] find_nation_awards 0x414340 (1); Date 0xae2c90
#   get_weekday_numeric_date_string 0x54a950 (3) ret 0xc; cdecl: 0x838de0, 0x838c60, 0x838d10 (1), 0x66a0c0 (1),
#   0x90bf90 (1), 0x765010/0x7651e0/0x7612c0 (3), langlib_sprintf, sprintf, time (0x946ebb), utils_get_time_ms,
#   utils_build_file_path, database_*, club_*, get_continent_adjective, _splitpath (0x944cff), gui_show_message_box.
#   The other tel26 routines (0x761260, 0x7612c0, 0x765010, 0x7651e0) are called by address.
# - The colours the original pushes from registers with only AX/CX/DX loaded (dirty high halves, or the high half of a
#   stack slot that held a time or an area id) are passed zero-extended (colour_word), as in the mould.
# - 1080: in cm0102-1080.exe the background's height (push imm32 at 0x75dc10, 0x257 -> 0x437) and the team menu's
#   header font (push imm8 at 0x75e16b, 1 -> 2) differ; the copy reads both from the exe.
# - Split in named pieces (game, team, manager, competitions, nations & find, game options, match and status menus),
#   all inside the one hooked routine.
# Defects of the original: none that changes the bar. Notes: the competitions loop never destroys its
# NATION_MENU_SPLIT (a plain local, nothing leaks); "Send Abuse To" compares the human manager table entries with the
# slot's manager; a competition row with neither nation nor continent shows the error box on every build.
# Comparer: side by side over the display list plus the menu bar record (without the refresh time); in a network
# game only the original runs.
MOTOR = True
TELAS = True
ROTINAS = [('menubar_build', 0x75db00, [0x81, 0xec, 0x64, 0x03, 0x00, 0x00])]
DOCS = ('a barra de menus',
        'the menu bar',
        [('menubar_build', 'menubar_build', '0x75db00', 'Menubar.cpp', 'cdecl',
          'Monta a barra de menus do técnico humano atual: guarda a hora da montagem e o tratador do menu; sem o modo interativo ou sem técnico entre os 16 últimos registros de pessoa não há registro e sai o menu do jogo; o modo 4 (redesenhar) vira o modo do registro, ou 2 quando não há modo ou o técnico não pode voltar nem avançar. O fundo (0, 0, 0x59, 599; 1079 na cópia 1080) com a figura game.mbr e a área da barra. Modo 1, o menu do jogo: a versão, as setas, "Add Manager", "Chat" (jogo em rede), "Restart Game" e "Exit Game". Modo 2, o menu do time: a data (ou a partida, ou a porcentagem do dia de jogos), as setas, "Continue Game" (ou "Latest Scores", "Waiting For Players", a contagem regressiva no jogo em rede), o menu do técnico (seleções, clube, notícias, buscas, transferências, histórico, férias, "Send Abuse To", aposentar), Competitions (por nação, continente e mundiais, com prêmios, ranking da FIFA e coeficientes da UEFA), Nations & Clubs, Find, trocar de jogador e Game Options. Modo 3, o menu da partida pausada: o minuto, as setas, "Paused", as táticas dos dois times e a velocidade da narração. Por fim o aviso de baixo (notícias, mensagem de chat, jogadores esperando ou vazio), o carimbo das férias e a hora de atualização do registro',
          "Builds the menu bar of the current human manager: stores the build time and the menu handler; with no interactive mode or no manager among the last 16 staff records there is no record and the game menu is built; mode 4 (redraw) becomes the record's mode, or 2 when there is none or the manager can go neither back nor forward. The background (0, 0, 0x59, 599; 1079 in the 1080 copy) with the game.mbr picture and the bar's area. Mode 1, the game menu: the version, the arrows, \"Add Manager\", \"Chat\" (network game), \"Restart Game\" and \"Exit Game\". Mode 2, the team menu: the date (or the match, or the match day percentage), the arrows, \"Continue Game\" (or \"Latest Scores\", \"Waiting For Players\", the countdown in a network game), the manager's menu (national teams, club, news, searches, transfers, history, holiday, \"Send Abuse To\", retire), Competitions (by nation, continent and the world ones, with awards, FIFA rankings and UEFA coefficients), Nations & Clubs, Find, change player and Game Options. Mode 3, the paused match menu: the minute, the arrows, \"Paused\", both teams' tactics and the commentary speed. Finally the bottom status item (news, chat message, players waiting or empty), the holiday stamp and the record's refresh time")])
