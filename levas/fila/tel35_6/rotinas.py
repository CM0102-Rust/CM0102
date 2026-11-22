# The menu bar's object links (Menubar.cpp; reader tel35, the screens round, 21/11/2026). Uses the helpers of tel26_1
# (tel26_item_id, tel26_item_value) and tel35_2 (tel35_cdecl1).
# Conventions checked in the disassembly: cdecl(short raw) -> eax; ScreenManager::print_screen 0x803bf0 thiscall(SM);
# comp_is_reserve_or_minor_comp 0x7e6f20 thiscall([0xb74434], comp) ret 4; Friendly::is_own_comp_record 0x5c2990
# thiscall([0xae2c74], record) ret 4; comp_screens_open_comp_screen 0x48f4a0 cdecl(comp, stage, round, 0);
# Comp::get_stage 0x48d200 thiscall(comp, k) ret 4; comp_util_get_base_round_id 0x4b0870 cdecl(round);
# open_club_history_screen 0x469e70 cdecl(club, 0, 1); open_tournament_invitations_screen 0x4739d0 cdecl(club, 0);
# Match_Stats::find_record_fixture 0x729e30 thiscall([0xb6b7c0], fixture, out) ret 8; open_match_details 0x71b4a0
# cdecl(fixture); database_is_national_team 0x53b900 cdecl; open_club_squad_screen 0x453d60 cdecl(club, reserves, 0,
# 0, 0); award_screens_open_award_screen 0x414c20 cdecl; staff_screens_open_staff_profile 0x8702a0 cdecl(person, tab,
# kind, 0, extra); Display::get_widget_id 0x468b10 / get_widget_data 0x4739a0 thiscall(display, raw) ret 4;
# ScreenManager::get_widget_action_id 0x468b40 / get_widget_action_data 0x468b90 thiscall(SM, raw) ret 4;
# HUMAN_MANAGER::is_watching_fixture 0x5fe290 (fixture, 0) and set_watched_fixture 0x5fe500 (fixture or 0, 0) ret 8;
# search_screens_open_player_search_screen 0x808540 cdecl.
# The original keeps the slot and the command id in registers and reloads them after each call that can change them;
# the copy reads them for each test (the same values).
# Equal in cm0102-1080.exe.
# Defects of the original: a 0x7d0 link with no item (-1) reads the competition id at address 0 (a crash); kept.
# Left out: menubar_handle_command (0x7615d0, 11771 bytes: the periodic menu bar work (auto save, the Continue button
# states, the date and match line) and about a hundred command ids): too large for this reader's session, for a
# reader of its own (see PRONTO.txt).
# Comparer: only the original (it opens the linked screens, prints, changes the watched fixture).
MOTOR = True
TELAS = True
ROTINAS = [('menubar_handle_object_link', 0x7643d0, [0x83, 0xec, 0x58, 0x66, 0xa1, 0x7a, 0x7c, 0xb7, 0x00])]
DOCS = ('a barra de menus (3): os links dos objetos',
        'the menu bar (3): the object links',
        [('menubar_handle_object_link', 'menubar_handle_object_link', '0x7643d0', 'Menubar.cpp', 'cdecl',
          'um link em qualquer tela abre a tela do objeto: a competição (ou a fase e a rodada do link, quando é copa), o histórico e os convites do clube, os detalhes da partida, o elenco (ou o reserva), o prêmio, o perfil da pessoa na aba do link, a partida a assistir (liga e desliga) e a busca de jogadores; "Print" imprime a tela antes',
          'a link on any screen opens the object\'s screen: the competition (or the link\'s stage and round, for a cup), the club\'s history and invitations, the match details, the squad (or the reserves), the award, the person\'s profile on the link\'s tab, the fixture to watch (on and off) and the player search; "Print" prints the screen first')])
