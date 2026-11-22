# The squad list comparators (2): club_screens.cpp (reader tel30, the screens round, 21/11/2026; a new group from the
# disassembly sweep, passed by address to qsort by build_filtered_sorted_squad_list 0x4841d0).
# Conventions checked in the disassembly: every routine is a qsort comparator cdecl(Staff** a, Staff** b) ret 0 -> eax.
# - 0x4cc2d0 contract_manager_get_player_morale (ecx = [G_CONTRACT_MANAGER], one stack argument, ret 4) -> al;
#   0x62c3d0 Injury::get_fatigue thiscall on [G_INJURY](staff id) ret 4 -> ax; 0x62c400 get_player_injury_status
#   (player) ret 4 -> al; 0x62c590 get_condition(player, 1) ret 8 -> ax; 0x7c1e30 PlayerStats::get_stat thiscall on
#   [G_PLAYER_STATS](player record, category, field, 6.5f) ret 0x10 -> st0 (the first stored with fstp dword, the
#   second compared with fcom); 0x543280 staff_is_player_attribute_visible cdecl(viewer, staff, attribute) -> al;
#   0x5431e0 staff_get_player_attribute cdecl(staff, attribute) -> al; 0x4cf5a0 / 0x4cf6d0 get_contract /
#   get_loan_contract thiscall on [G_CONTRACT_MANAGER](staff) ret 4; 0x53b900 database_is_national_team cdecl(club).
# Identical in cm0102-1080.exe (the whole routines).
# Depends on: tel30_5 (player_sort_by_name, player_sort_pair, player_sort_compare, the PLAYER_SORT_* constants).
#   tel30_7 uses player_sort_list_contract and player_sort_by_float_stat of this one.
# Comparers: each comparator's return side by side with the original (pure).
# Defects of the original: none found (the nationality order copies both names to 400-byte stack buffers first).
MOTOR = True
TELAS = True
ROTINAS = [('compare_players_by_morale_then_name', 0x4860e0, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('compare_players_by_staff_state_byte_then_name', 0x4861e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('compare_players_by_fatigue_then_name', 0x4862c0, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('compare_players_by_injury_recovery_then_name', 0x4863c0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('compare_players_by_selected_stat_then_name', 0x4864e0, [0x81, 0xec, 0x94, 0x01, 0x00, 0x00]),
           ('compare_players_by_selected_attribute_then_name', 0x486620, [0x81, 0xec, 0x94, 0x01, 0x00, 0x00]),
           ('compare_players_by_nationality_then_name', 0x486760, [0x81, 0xec, 0x24, 0x03, 0x00, 0x00]),
           ('compare_players_by_age_then_name', 0x4868c0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('compare_players_by_caps_then_name', 0x486970, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('compare_players_by_international_goals_then_name', 0x486a30, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('compare_players_by_wage_then_name', 0x486af0, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00])]
_A = 'club_screens.cpp'
DOCS = ('as ordens das listas do elenco (2): moral, estado, cansaço, lesão, estatística, atributo, nacionalidade, idade, jogos e gols pela seleção e salário',
        'the squad list orders (2): morale, state, fatigue, injury, statistic, attribute, nationality, age, caps, international goals and wage',
        [('compare_players_by_morale_then_name', 'compare_players_by_morale_then_name', '0x4860e0', _A, 'cdecl',
          'ordena pela moral, a maior primeiro, depois pelo nome', 'orders by morale, highest first, then by name'),
         ('compare_players_by_staff_state_byte_then_name', 'compare_players_by_staff_state_byte_then_name', '0x4861e0', _A, 'cdecl',
          'ordena pelo byte +0xa da tabela de estado das pessoas (o primeiro menos o segundo), depois pelo nome',
          'orders by byte +0xa of the staff state table (first minus second), then by name'),
         ('compare_players_by_fatigue_then_name', 'compare_players_by_fatigue_then_name', '0x4862c0', _A, 'cdecl',
          'ordena pelo cansaço (Injury::get_fatigue), o menor primeiro, depois pelo nome',
          'orders by fatigue (Injury::get_fatigue), lowest first, then by name'),
         ('compare_players_by_injury_recovery_then_name', 'compare_players_by_injury_recovery_then_name', '0x4863c0', _A, 'cdecl',
          'ordena pela situação de lesão (o menor primeiro), depois pela condição física em porcentagem (a maior primeiro), depois pelo nome',
          'orders by injury status (lowest first), then by condition in percent (highest first), then by name'),
         ('compare_players_by_selected_stat_then_name', 'compare_players_by_selected_stat_then_name', '0x4864e0', _A, 'cdecl',
          'ordena pela estatística escolhida (PlayerStats::get_stat com a categoria e o campo da lista, 6,5 de reserva), a maior primeiro, depois pelo nome',
          "orders by the chosen statistic (PlayerStats::get_stat with the list's category and field, 6.5 as default), highest first, then by name"),
         ('compare_players_by_selected_attribute_then_name', 'compare_players_by_selected_attribute_then_name', '0x486620', _A, 'cdecl',
          'ordena pelo atributo escolhido, o maior primeiro; o que o observador não vê vale -1; depois pelo nome',
          'orders by the chosen attribute, highest first; one the viewer cannot see counts -1; then by name'),
         ('compare_players_by_nationality_then_name', 'compare_players_by_nationality_then_name', '0x486760', _A, 'cdecl',
          'ordena pela sigla de três letras da nação do jogador, depois pelo nome; sem nação vai por último',
          "orders by the three-letter name of the player's nation, then by name; no nation goes last"),
         ('compare_players_by_age_then_name', 'compare_players_by_age_then_name', '0x4868c0', _A, 'cdecl',
          'ordena pela idade, o mais velho primeiro, depois pelo nome', 'orders by age, oldest first, then by name'),
         ('compare_players_by_caps_then_name', 'compare_players_by_caps_then_name', '0x486970', _A, 'cdecl',
          'ordena pelos jogos pela seleção, mais primeiro, depois pelo nome', 'orders by caps, most first, then by name'),
         ('compare_players_by_international_goals_then_name', 'compare_players_by_international_goals_then_name', '0x486a30', _A, 'cdecl',
          'ordena pelos gols pela seleção, mais primeiro, depois pelo nome', 'orders by international goals, most first, then by name'),
         ('compare_players_by_wage_then_name', 'compare_players_by_wage_then_name', '0x486af0', _A, 'cdecl',
          'ordena pelo salário do contrato que a lista mostra (o de empréstimo quando o contrato é de outro clube e a lista não é de seleção), o maior primeiro; sem contrato por último; depois pelo nome',
          "orders by the wage of the contract the list shows (the loan one when the contract is with another club and the list is not a national team's), highest first; no contract last; then by name")])
