# The squad list comparators (1): club_screens.cpp (reader tel30, the screens round, 21/11/2026; a new group from the
# disassembly sweep: functions the community map did not see, passed by address to qsort).
# Conventions checked in the disassembly: every routine is a qsort comparator cdecl(Staff** a, Staff** b) ret 0 -> eax.
# - 0x600c90 human_manager_current_controls_club cdecl(club, 0, 1); 0x8972c0 Tactics::find_selection_slot thiscall on
#   [G_TACTICS](club, player, reserves) ret 0xc -> al; 0x601880 HUMAN_MANAGER::get_player_squad_number thiscall on
#   [G_HUMAN_MANAGER](player, 0) ret 8 -> al; 0x4849f0 get_player_status_code cdecl(player, club, fixture) -> al;
#   0x53ee50 database_format_player_positions cdecl(player record, buffer, 200, 4); 0x4cf5f0 / 0x4cf5a0
#   CONTRACT_MANAGER::get_contract_for_club / get_contract thiscall on [G_CONTRACT_MANAGER]; 0x85a760
#   SQUAD_MANAGER::get_squad_number thiscall on [0xdd7ef0](player, side) ret 8 -> al; 0x868020
#   STAFF_CONTRACT::get_release_clause_abbreviation thiscall(contract, buffer, 30) ret 8 -> al; 0x7c1950
#   PlayerStats::get_recent_form thiscall on [G_PLAYER_STATS](player record) ret 4 -> al; the name tie-break
#   0x53dba0 database_format_staff_name cdecl(staff, buffer, 200, style) and 0x90c1d0 utils_string_compare cdecl(a, b,
#   1, 0).
# - The sort context (0xad9db0 club, 0xadab33 name style, 0xadab34 reserves, 0xadab38 team sheet, 0xadab3c side,
#   0xadab44 fixture) is written by build_filtered_sorted_squad_list (0x4841d0, tel06_1) before its qsort.
# Identical in cm0102-1080.exe (the whole routines).
# Item tel30_6 and tel30_7 use player_sort_by_name, player_sort_pair, player_sort_compare and the PLAYER_SORT_*
#   constants of this one: this item goes first.
# Comparers: each comparator's return side by side with the original (pure: they only read and format names).
# Defects of the original: none found (compare_players_by_human_squad_slot has no name tie-break; the selection slot
#   of a club the human does not control takes the last of the matching team sheet entries).
MOTOR = True
TELAS = True
ROTINAS = [('compare_players_by_selection_slot_then_name', 0x485470, [0x81, 0xec, 0x94, 0x01, 0x00, 0x00]),
           ('compare_players_by_human_squad_slot', 0x485630, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('compare_players_by_status_code_then_name', 0x4856a0, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('compare_players_by_name', 0x4857a0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('compare_players_by_positions_then_name', 0x485830, [0x81, 0xec, 0x20, 0x03, 0x00, 0x00]),
           ('compare_players_by_club_name_then_name', 0x485960, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('compare_players_by_squad_status_then_name', 0x485a60, [0x81, 0xec, 0x94, 0x01, 0x00, 0x00]),
           ('compare_players_by_squad_number_then_name', 0x485ba0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('compare_players_by_release_clause_then_name', 0x485d20, [0x81, 0xec, 0xd8, 0x01, 0x00, 0x00]),
           ('compare_players_by_euro_registration_then_name', 0x485ed0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('compare_players_by_form_then_name', 0x485fd0, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00])]
_A = 'club_screens.cpp'
DOCS = ('as ordens das listas do elenco (1): seleção, número do técnico, situação, nome, posições, clube, situação no elenco, número da camisa, multa rescisória, inscrição europeia e forma',
        'the squad list orders (1): selection, the human\'s squad slot, status, name, positions, club, squad status, squad number, release clause, European registration and form',
        [('compare_players_by_selection_slot_then_name', 'compare_players_by_selection_slot_then_name', '0x485470', _A, 'cdecl',
          'ordena pela vaga na escalação (Tactics::find_selection_slot no clube do humano; senão a última das 20 vagas da ficha do time com o id do jogador; sem ficha dá 1); os não escalados por último, empate pelo nome',
          'orders by the selection slot (Tactics::find_selection_slot at the human\'s club; otherwise the last of the team sheet\'s 20 slots holding the player\'s id; no sheet gives 1); the unselected last, ties by name'),
         ('compare_players_by_human_squad_slot', 'compare_players_by_human_squad_slot', '0x485630', _A, 'cdecl',
          'ordena pelo número do jogador no elenco do técnico humano (HUMAN_MANAGER::get_player_squad_number); os sem número por último; sem desempate',
          'orders by the player\'s number in the human manager\'s squad (HUMAN_MANAGER::get_player_squad_number); the unnumbered last; no tie-break'),
         ('compare_players_by_status_code_then_name', 'compare_players_by_status_code_then_name', '0x4856a0', _A, 'cdecl',
          'ordena pelo código de situação do jogador (get_player_status_code com o clube e a partida da lista), depois pelo nome',
          'orders by the player\'s status code (get_player_status_code with the list\'s club and fixture), then by name'),
         ('compare_players_by_name', 'compare_players_by_name', '0x4857a0', _A, 'cdecl',
          'ordena pelo nome no estilo da lista (um ponteiro nulo vai por último)',
          'orders by name in the list\'s style (a null pointer goes last)'),
         ('compare_players_by_positions_then_name', 'compare_players_by_positions_then_name', '0x485830', _A, 'cdecl',
          'ordena pelo texto das posições (database_format_player_positions, estilo 4), depois pelo nome; sem ficha de jogador vai por último',
          'orders by the positions text (database_format_player_positions, style 4), then by name; no player record goes last'),
         ('compare_players_by_club_name_then_name', 'compare_players_by_club_name_then_name', '0x485960', _A, 'cdecl',
          'ordena pelo nome do clube do jogador, depois pelo nome; sem clube vai por último',
          'orders by the player\'s club name, then by name; no club goes last'),
         ('compare_players_by_squad_status_then_name', 'compare_players_by_squad_status_then_name', '0x485a60', _A, 'cdecl',
          'ordena pela situação no elenco (o nibble alto do byte +0x4f do contrato com o lado da lista, só quando o humano o dirige; 100 sem contrato), depois pelo nome',
          'orders by the squad status (the high nibble of byte +0x4f of the contract with the list\'s side, only when the human runs it; 100 without a contract), then by name'),
         ('compare_players_by_squad_number_then_name', 'compare_players_by_squad_number_then_name', '0x485ba0', _A, 'cdecl',
          'ordena pelo valor absoluto do número da camisa no lado da lista; sem ficha de jogador ou com número 0 por último; empate pelo nome',
          'orders by the absolute squad number at the list\'s side; no player record or number 0 last; ties by name'),
         ('compare_players_by_release_clause_then_name', 'compare_players_by_release_clause_then_name', '0x485d20', _A, 'cdecl',
          'ordena pelo código da multa rescisória do contrato (o segundo menos o primeiro; -1 sem contrato), depois pela abreviatura ("  -" sem contrato), depois pelo nome',
          'orders by the contract\'s release clause code (second minus first; -1 without a contract), then by its abbreviation ("  -" without a contract), then by name'),
         ('compare_players_by_euro_registration_then_name', 'compare_players_by_euro_registration_then_name', '0x485ed0', _A, 'cdecl',
          'ordena pela inscrição europeia (o byte +0x6d na ordem 1, 2, 4, 0, os outros), depois pelo nome',
          'orders by the European registration (byte +0x6d in the order 1, 2, 4, 0, the others), then by name'),
         ('compare_players_by_form_then_name', 'compare_players_by_form_then_name', '0x485fd0', _A, 'cdecl',
          'ordena pela forma recente (PlayerStats::get_recent_form), a melhor primeiro; sem ficha de jogador ou com id fora da tabela por último; empate pelo nome',
          'orders by recent form (PlayerStats::get_recent_form), the best first; no player record or an id outside the table last; ties by name')])
