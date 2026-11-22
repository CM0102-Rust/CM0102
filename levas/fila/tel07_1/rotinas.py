# Opening the staff profile and the compare players screens (staff_screens.cpp, part 1; reader tel07, the screens round,
# 21/11/2026).
# - 0x8702a0 staff_screens_open_staff_profile (944 bytes): cdecl(staff, tab, slot2, keep_view, attribute_mode), ret 0;
#   the callers push the chars as dwords (the routine reads the bytes). Calls: 0x7ff010 get_screen_param thiscall(k)
#   ret 4, 0x7fe600 open_screen thiscall(builder, handler, 0, close, 0) ret 0x14, 0x7ff270 set_screen_param
#   thiscall(k, v, 0) ret 0xc; the error box (0x944cff, 0x90d130, 0x944c9f, 0x5e8290 cdecl) as show_error_at.
# - 0x870650 staff_screens_open_compare_players_screen (176 bytes; the map's 40640 bytes also hold the two builders
#   0x870700 and 0x876520, items tel07_3 and tel07_2): cdecl(manager, ...) ret 0, only the first argument read;
#   0x7f7920 SCOUT_MANAGER::get_compare_player thiscall(which) ret 4 on [0xb74448].
# Comparers: opening does not run twice; the profile one checks the 22 params of the new screen against a plan
# written apart (staff_profile_planned_params); the compare one runs the original only.
# How to reach them: any person's name (link) opens the profile (e.g. Squad > a player); the compare screen: the
# profile's Action menu > "Compare with..." (after choosing two players to compare in the scouting).
TELAS = True
ROTINAS = [('staff_screens_open_staff_profile', 0x8702a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00, 0x53, 0x56]),
           ('staff_screens_open_compare_players_screen', 0x870650, [0x8b, 0x0d, 0x48, 0x44, 0xb7, 0x00, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('abrir o perfil da pessoa e a comparação de jogadores',
        'opening the staff profile and the compare players screens',
        [('staff_screens_open_staff_profile', 'staff_screens_open_staff_profile', '0x8702a0', 'staff_screens.cpp', 'cdecl',
          'Abre o perfil da pessoa na aba pedida (aba 5 para os 16 últimos registros de pessoa): sem pessoa, a caixa de erro; só abre se a pessoa pode ser vista (quem não é técnico e não está entre os 16 últimos registros; quem tem ficha de jogador e id abaixo da contagem; ou, com id abaixo da contagem, quem não tem cargo 1 nem 2). Grava os 22 parâmetros da tela: a pessoa, a aba, o segundo valor, 4 e 5 = 1 e 1 para quem tem ficha de não jogador (sem ficha de jogador, cargo fora de 0xb/0xc/0xd/0xf, ou modo de atributos 1), senão o modo e 2; as cores azul e cinza em 0x11 e 0x12; com "manter a vista", os parâmetros 1, 2, 5 e 0x13 voltam da tela anterior e o 4 volta a 2',
          'Opens the person\'s profile on the chosen tab (tab 5 for the 16 last staff records): with no person, the error box; it only opens when the person is viewable (not a manager and not among the 16 last records; a player sheet with an id below the count; or, with an id below the count, a job other than 1 and 2). Writes the screen\'s 22 params: the person, the tab, the second value, 4 and 5 = 1 and 1 for a non-player sheet (no player sheet, a job outside 0xb/0xc/0xd/0xf, or attribute mode 1), otherwise the mode and 2; the blue and grey colours in 0x11 and 0x12; with "keep the view", params 1, 2, 5 and 0x13 come back from the previous screen and 4 back to 2'),
         ('staff_screens_open_compare_players_screen', 'staff_screens_open_compare_players_screen', '0x870650', 'staff_screens.cpp', 'cdecl',
          'Abre a tela de comparação de jogadores (construtor 0x876520, resposta 0x889c30); sem o primeiro jogador da comparação e sem o técnico passado, a caixa de erro',
          'Opens the compare players screen (builder 0x876520, handler 0x889c30); with no first compare player and no manager given, the error box')])
