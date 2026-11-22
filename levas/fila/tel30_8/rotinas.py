# The club screen's records, the squad screen opener and the squad statistics: club_screens.cpp (reader tel30, the
# screens round, 21/11/2026; a new group from the disassembly sweep: static constructors in the CRT init table,
# constructors passed to the vector iterators, MenuItem::MenuItem passed by address, the opener and the statistics).
# Conventions checked in the disassembly (every push against esp):
# - 0x453b90 / 0x453c60 cdecl() static constructors (pointers in the CRT init table): 0x453be0 on 0xad9db8 / the
#   vector constructor on 0xadab50, then 0x94627a atexit cdecl(0x453bb0 / 0x453d40); 0x453bb0 cdecl() tail-jumps to
#   0x453bc0 match_team_pair_destructor thiscall(0xad9db8); 0x453d40 cdecl().
# - 0x453be0 / 0x453c30 thiscall(this) ret 0 -> this; 0x453c10 thiscall(this) ret 0; 0x9463af `eh vector constructor
#   iterator` stdcall(array, size, count, ctor, dtor) ret 0x14; 0x9462bb `eh vector destructor iterator`
#   stdcall(array, size, count, dtor) ret 0x10; the rating record constructor 0x453c50 and the empty 0x5a53c0.
# - 0x453c90 thiscall(this) -> this, SEH: 0x54a110 Date::Date thiscall(this), 0x68d020 LLIST::LLIST thiscall(this +
#   0x18), then rep stosd ×10 + stosw (0x2a bytes cleared). 0x453cf0 thiscall(this), SEH: [this + 0x26] -> vtable
#   slot 0 thiscall(object, 1), then 0x68d040 LLIST::~LLIST(this + 0x18).
# - 0x453d60 open_club_squad_screen cdecl(club, reserve, restore, tab5, highlight) ret 0, SEH (no object): 0x542740
#   database_club_has_reserve_squad cdecl(club); 0x7ff010 / 0x7ff270 get_ / set_screen_param on 0xb74450 (ret 4 /
#   0xc); 0x600c90 human_manager_current_controls_club cdecl(club, 0, 1); 0x5fe950
#   HUMAN_MANAGER::get_squad_view_settings thiscall([0xb63c98], 0) ret 4; 0x7fe600 open_screen(0x454900, 0x4671f0, 0,
#   0x469da0, 0) ret 0x14; 0x53b900 database_is_national_team cdecl(club).
# - 0x4568f0 MenuItem::MenuItem thiscall(this) ret 0 -> this.
# - 0x487840 club_screen_collect_squad_stats cdecl(club, reserve, out) ret 0, SEH (two local Dates, 0x54a110):
#   0x540a50 club_get_linked_b_team cdecl(club, &senior, 1) / (club, 0, 1); 0x541f90 database_collect_club_squad
#   cdecl(club, list, 1); 0x62c400 Injury::get_player_injury_status thiscall([0xb63d28], player) ret 4; 0x8972c0
#   Tactics::find_selection_slot thiscall([0xde1d08], side, player, 0) ret 0xc; 0x4cf5f0 get_contract_for_club
#   thiscall([0xae1050], player, side) ret 8; 0x417c10 Date::operator= thiscall(date, scratch, source) ret 8; 0x5409b0
#   club_is_national_team cdecl; 0x53d980 database_is_staff_foreign cdecl(player, nation, 1); 0x53d740
#   database_is_staff_eu_national cdecl(player, nation); the averages with fild / fidiv, 0x945640 _ftol and fstp dword.
#   The age sums are added in 32 bits with the upper half of ebp left over (only their low word is read: 16 bits here).
# Identical in cm0102-1080.exe (the whole routines).
# Depends on: tel29_1 (current_screen_node).
# Comparers: MenuItem::MenuItem and the squad statistics side by side (the 0x45-byte record); the club history state
#   constructor the original only with the plan (0x2a bytes cleared); the opener the original only with the plan (the
#   club screen current, the three best players in 0x20..0x22); the static constructors and destructors and the team
#   record constructors the original only (they run once, at the start or the exit).
# Defects of the original: the club history state constructor clears its date and list right after building them;
#   the statistics leave SELECTED_AVERAGE_WAGE (+0x16) as it was when no player is selected.
MOTOR = True
TELAS = True
ROTINAS = [('club_screens_init_match_record', 0x453b90, [0xb9, 0xb8, 0x9d, 0xad, 0x00]),
           ('club_screens_exit_match_record', 0x453bb0, [0xb9, 0xb8, 0x9d, 0xad, 0x00]),
           ('match_team_pair_construct', 0x453be0, [0x56, 0x8b, 0xf1, 0x68, 0x10, 0x3c, 0x45, 0x00]),
           ('match_team_block_destruct', 0x453c10, [0x68, 0xc0, 0x53, 0x5a, 0x00]),
           ('match_team_block_construct', 0x453c30, [0x56, 0x8b, 0xf1, 0x68, 0xc0, 0x53, 0x5a, 0x00]),
           ('club_screens_init_history_states', 0x453c60, [0x68, 0xf0, 0x3c, 0x45, 0x00]),
           ('club_history_state_construct', 0x453c90, [0x6a, 0xff, 0x68, 0x78, 0x57, 0x95, 0x00]),
           ('club_history_state_destruct', 0x453cf0, [0x6a, 0xff, 0x68, 0xa3, 0x57, 0x95, 0x00]),
           ('club_screens_exit_history_states', 0x453d40, [0x68, 0xf0, 0x3c, 0x45, 0x00]),
           ('open_club_squad_screen', 0x453d60, [0x6a, 0xff, 0x68, 0xbb, 0x57, 0x95, 0x00]),
           ('menu_item_menu_item', 0x4568f0, [0x8b, 0xc1, 0x66, 0xc7, 0x00, 0xff, 0xff]),
           ('club_screen_collect_squad_stats', 0x487840, [0x6a, 0xff, 0x68, 0xa6, 0x5f, 0x95, 0x00])]
_A = 'club_screens.cpp'
DOCS = ('os registros da tela do clube, a abertura da tela do elenco e as estatísticas do elenco',
        "the club screen's records, the squad screen opener and the squad statistics",
        [('club_screens_init_match_record', 'club_screens_init_match_record', '0x453b90', _A, 'cdecl',
          'constrói o registro da ficha do último jogo (0xad9db8) na partida do jogo e registra a sua destruição no atexit',
          "builds the last match's team sheet record (0xad9db8) at the game start and registers its destruction with atexit"),
         ('club_screens_exit_match_record', 'club_screens_exit_match_record', '0x453bb0', _A, 'cdecl',
          'destrói o registro da ficha do último jogo', "destroys the last match's team sheet record"),
         ('match_team_pair_construct', 'match_team_pair_construct', '0x453be0', _A, 'thiscall',
          'constrói os dois blocos de time (0x625 bytes) de uma ficha a partir de +0x11f', 'builds the two team blocks (0x625 bytes) of a team sheet from +0x11f'),
         ('match_team_block_destruct', 'match_team_block_destruct', '0x453c10', _A, 'thiscall',
          'destrói os 20 registros de nota (0x49 bytes) de um bloco de time a partir de +0x71', 'destroys the 20 rating records (0x49 bytes) of a team block from +0x71'),
         ('match_team_block_construct', 'match_team_block_construct', '0x453c30', _A, 'thiscall',
          'constrói os 20 registros de nota de um bloco de time', 'builds the 20 rating records of a team block'),
         ('club_screens_init_history_states', 'club_screens_init_history_states', '0x453c60', _A, 'cdecl',
          'constrói os 16 estados do histórico do clube (0xadab50) e registra a sua destruição no atexit',
          'builds the 16 club history states (0xadab50) and registers their destruction with atexit'),
         ('club_history_state_construct', 'club_history_state_construct', '0x453c90', _A, 'thiscall',
          'constrói um estado do histórico: a data e a lista, e depois zera os seus 0x2a bytes', 'builds a history state: its date and list, then clears its 0x2a bytes'),
         ('club_history_state_destruct', 'club_history_state_destruct', '0x453cf0', _A, 'thiscall',
          'destrói um estado do histórico: o objeto em +0x26 (pelo destrutor virtual) e a lista', 'destroys a history state: the object at +0x26 (through its virtual destructor) and the list'),
         ('club_screens_exit_history_states', 'club_screens_exit_history_states', '0x453d40', _A, 'cdecl',
          'destrói os 16 estados do histórico do clube', 'destroys the 16 club history states'),
         ('open_club_squad_screen', 'open_club_squad_screen', '0x453d60', _A, 'cdecl',
          'abre a tela do clube na aba do elenco: os parâmetros da vista (as do técnico humano quando ele dirige o clube, senão as de fábrica), o ano, a aba (5 quando pedida), os parâmetros guardados quando é para restaurar, e nos parâmetros 0x20..0x22 os três melhores jogadores (reputação mais 10 × habilidade, a mundial nas seleções)',
          "opens the club screen at its squad tab: the view parameters (the human manager's when he runs the club, otherwise the defaults), the year, the tab (5 when asked), the saved parameters when restoring, and in parameters 0x20..0x22 the three best players (reputation plus 10 × ability, the world one for national teams)"),
         ('menu_item_menu_item', 'MenuItem::MenuItem', '0x4568f0', _A, 'thiscall',
          'um item de menu: id -1 e valor 0', 'a menu item: id -1 and value 0'),
         ('club_screen_collect_squad_stats', 'club_screen_collect_squad_stats', '0x487840', _A, 'cdecl',
          'junta as estatísticas do elenco: quantos e lesionados, idades médias (de todos e dos escalados), salários (total, médias, o mais e o menos bem pago), o mais velho e o mais novo, jogos pela seleção (total, o mais e o menos convocado), o mais valioso e as contagens de convocados, estrangeiros, fora da UE e da nação do clube',
          "gathers the squad statistics: how many and injured, average ages (all and selected), wages (total, averages, the best and worst paid), the oldest and the youngest, caps (total, most and least capped), the most valuable and the counts of internationals, foreigners, non-EU and the club's nationals")])
