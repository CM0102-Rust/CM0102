# The national squad screens (national_teams_screens.cpp; reader tel28, the screens round, 21/11/2026): the dialog
# that moves a player into, out of or between national squads, and the squad selection and call-up confirmations
# opened from the inbox. Depends on tel28_1 (tel28_lang, tel28_click_compare) and tel26_1 (tel26_*, JOB_TEXT).
# Conventions checked in the disassembly (every push against esp):
# - the openers cdecl: 0x778f60 (player, kind byte, joining), 0x7798c0 and 0x77a140 (manager record, news id) (the
#   inbox's Respond buttons store them, 0x782b70); 0x7fe600 open_screen thiscall(build, input, 1, 0, 0) ret 0x14.
# - the builds cdecl(void): 0x542100 club_get_national_team_name cdecl(club, kind, buf, size, form, 1); 0x53b640
#   database_get_club_kit_colours cdecl(club, 1, &front, &bottom) (words); 0x803f30 add_screen_title thiscall(bottom,
#   front, text) ret 0xc; 0x803f80 add_screen_subtitle thiscall(text, 1) ret 8; 0x5409b0 club_is_national_team;
#   0x76db00 national_teams_can_change_squad thiscall(nation, kind, player, joining, msg, 0x7d0) ret 0x18; 0x775610
#   format_squad_label thiscall(kind, olympic mark, out, 0x64, lower) ret 0x14; 0x53dba0 database_format_staff_name
#   cdecl(player, buf, 0x12c, 1); 0x7754c0 / 0x775520 get_max/min_squad_size thiscall(club, kind) ret 8 -> al;
#   0x5ed070 gui_add_text_area cdecl(11); 0x55fba0 Display::add_area thiscall(11) ret 0x2c; 0x8040a0 add_area_button
#   thiscall(11) ret 0x2c; 0x804030 add_button_item thiscall(12) ret 0x30.
# - the inputs cdecl(widget) -> eax (-9 when answered): 0x7762f0 remove_player_from_squad thiscall(nation, kind,
#   player) ret 0xc; 0x76e010 thiscall(nation, kind) ret 8; 0x5fd6b0 human_manager_controls_all_national_teams
#   thiscall(manager, 0) ret 8; 0x76d810 national_team_level stdcall(player, record, flag) ret 0xc (ecx loaded but
#   unused); 0x776250 add_player_to_squad thiscall(nation, kind, player) ret 0xc; 0x775f90 call_up_replacement
#   thiscall(nation, kind, record dword, player) ret 0x10; 0x786b70 News::find_item_by_id thiscall(manager, id, out)
#   ret 0xc; 0x785830 NewsItem::get_slot thiscall(k) ret 4; 0x76d780 get_squad_slot stdcall(club, kind) ret 8;
#   0x76c0f0 thiscall(nation, k) ret 8; 0x786d30 News::set_item_slot thiscall(manager, id, slot, value, 0) ret 0x14;
#   0x76e0b0 post_call_up_news thiscall(7) ret 0x1c; 0x549ef0 Date::Date thiscall(day, year) ret 8; 0x54c8e0 sub_days
#   thiscall(out, n) ret 8; 0x5acbe0 find_fixtures thiscall(9) ret 0x24 (the dates as two dword pairs at +0 and +2);
#   0x7771c0 record_club_call_ups thiscall(club, kind, fixture) ret 0xc; 0x7777e0 reschedule_clashing_club_fixtures
#   thiscall(date) ret 4; 0x53c590 database_count_club_squad cdecl(club, flag, 0); 0x4a7f20 Date::operator==
#   thiscall(lo, hi) ret 8; 0x76fd10 post_player_called_up_news thiscall(player, nation, slot) ret 0xc; 0x76d7e0
#   thiscall(nation, slot) ret 8.
# Kept from the original: the squad kinds the original passes as dwords with only the low byte stored go as that byte
#   (the callees read the byte); the move's replacement passes the nation record pointer with its low byte replaced by
#   the player's level, as the original does.
# Defects of the original: none found.
# Comparers: the openers only the original, the params checked against the plan; the builds side by side by the
#   display list, the original only with no human team (the error box); the inputs side by side for No (id 1), the
#   original only for Yes (they change the squads, the news and the fixtures), the menu bar and the links.
# Identical in cm0102-1080.exe (the nine routines whole).
TELAS = True
ROTINAS = [('national_teams_screens_open_squad_move_screen', 0x778f60, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('national_squad_move_screen_build', 0x778fd0, [0x81, 0xec, 0x10, 0x04, 0x00, 0x00]),
           ('national_squad_move_screen_handle_input', 0x779610, [0x81, 0xec, 0x18, 0x02, 0x00, 0x00]),
           ('national_squad_selection_open_from_news', 0x7798c0, [0x6a, 0xff, 0x68, 0xdb, 0xdf, 0x95, 0x00]),
           ('national_squad_selection_screen_build', 0x779a50, [0x81, 0xec, 0xd4, 0x00, 0x00, 0x00]),
           ('national_squad_selection_screen_handle_input', 0x779df0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('national_squad_call_up_open_from_news', 0x77a140, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('national_squad_call_up_screen_build', 0x77a270, [0x81, 0xec, 0xd4, 0x00, 0x00, 0x00]),
           ('national_squad_call_up_screen_handle_input', 0x77a610, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00])]
DOCS = ('as telas das convocações: mover um jogador entre os elencos da seleção, confirmar a seleção do elenco e a convocação',
        'the national squad screens: moving a player between the national squads, confirming the squad selection and the call-up',
        [('national_teams_screens_open_squad_move_screen', 'national_teams_screens_open_squad_move_screen', '0x778f60', 'national_teams_screens.cpp', 'cdecl',
          'abre o diálogo de mover o jogador: parâmetros 0 o jogador, 1 o elenco (o byte), 2 entrar (1) ou sair (0), 3 = −1',
          'opens the move dialog: parameters 0 the player, 1 the squad (the byte), 2 joining (1) or leaving (0), 3 = −1'),
         ('national_squad_move_screen_build', 'national_squad_move_screen_build', '0x778fd0', 'national_teams_screens.cpp', 'cdecl',
          'monta o diálogo: o nome da seleção nas cores do clube, o elenco atual do jogador (−1 sem convocação, 2 quando é seleção, senão 1 ou 0 pela marca de convocado); sem poder sair ou entrar, o motivo, "Não foi possível..." e Ok; senão a pergunta (mover, acrescentar ou retirar) com Não/Sim',
          'builds the dialog: the national team name in the club colours, the player\'s current squad (−1 with no call-up, 2 when it is a national team, else 1 or 0 by the called-up flag); unable to leave or join, the reason, "Could Not..." and Ok; otherwise the question (move, add or remove) with No/Yes'),
         ('national_squad_move_screen_handle_input', 'national_squad_move_screen_handle_input', '0x779610', 'national_teams_screens.cpp', 'cdecl',
          'Sim tira o jogador do elenco atual (chamando um substituto quando o elenco não é o principal, a convocação está ativa e o humano não controla todas as seleções) e o põe no elenco escolhido; Não fecha (−9); o resto vai à barra de menus',
          'Yes takes the player out of his squad (calling up a replacement when the squad is not the main one, its call-up is active and the human does not control every national team) and puts him in the target squad; No closes (−9); the rest goes to the menu bar'),
         ('national_squad_selection_open_from_news', 'national_squad_selection_open_from_news', '0x7798c0', 'national_teams_screens.cpp', 'cdecl',
          'abre, da notícia, a confirmação da seleção do elenco: a seleção e o elenco da notícia, os jogadores ainda sem a marca do elenco contados, as vagas 5 a 7 da notícia',
          'opens, from the news item, the squad selection confirmation: the news item\'s team and squad, the players not yet flagged for the squad counted, the item\'s slots 5 to 7'),
         ('national_squad_selection_screen_build', 'national_squad_selection_screen_build', '0x779a50', 'national_teams_screens.cpp', 'cdecl',
          'confirma a seleção quando o número de jogadores está entre o mínimo e o máximo do elenco (Não/Sim); senão "Elenco Inválido" com os limites e Ok',
          'confirms the selection when the player count is within the squad\'s minimum and maximum (No/Yes); otherwise "Squad Invalid" with the limits and Ok'),
         ('national_squad_selection_screen_handle_input', 'national_squad_selection_screen_handle_input', '0x779df0', 'national_teams_screens.cpp', 'cdecl',
          'Sim marca a seleção feita, publica a notícia da convocação e, sem jogo pendente na notícia, remarca os jogos dos clubes que batem com a data do elenco',
          'Yes marks the selection made, posts the call-up news and, with no fixture pending in the item, moves the club fixtures that clash with the squad date'),
         ('national_squad_call_up_open_from_news', 'national_squad_call_up_open_from_news', '0x77a140', 'national_teams_screens.cpp', 'cdecl',
          'abre, da notícia, a confirmação da convocação: a seleção (vaga 4), o elenco (vaga 5) e o número de jogadores com a marca do elenco',
          'opens, from the news item, the call-up confirmation: the team (slot 4), the squad (slot 5) and the number of players flagged for it'),
         ('national_squad_call_up_screen_build', 'national_squad_call_up_screen_build', '0x77a270', 'national_teams_screens.cpp', 'cdecl',
          'confirma a convocação quando o número está dentro dos limites do elenco; senão "Elenco Inválido" com os limites e Ok',
          'confirms the call-up when the count is within the squad\'s limits; otherwise "Squad Invalid" with the limits and Ok'),
         ('national_squad_call_up_screen_handle_input', 'national_squad_call_up_screen_handle_input', '0x77a610', 'national_teams_screens.cpp', 'cdecl',
          'Sim publica a notícia de cada jogador convocado hoje e limpa a marca de convocação da nação',
          'Yes posts the news of each player called up today and clears the nation\'s call-up mark')])
