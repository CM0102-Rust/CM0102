# The search screens (2): the sort orders of the staff search columns (search_screens.cpp; reader tel21, the screens
# round, 21/11/2026). Uses the helpers of tel21_1 (search_result_keys, search_result_person,
# search_results_compare_names) and of tel26_1 (tel26_slot_manager): goes after both.
# Conventions checked in the disassembly: all cdecl(const search_result* a, const search_result* b) ret 0 -> int (qsort
# comparers from the sort table of the Player and Staff Search screen builder 0x808690). Callees: utils_string_compare
# 0x90c1d0 cdecl(a, b, 1, 0); database_format_player_positions 0x53ee50 cdecl(player, buffer, 0xc8, 4);
# PlayerStats::get_stat 0x7c1e30 thiscall(stats, player, 6, column, 6.5f) ret 0x10 -> st0;
# staff_is_player_attribute_visible 0x543280 cdecl(viewer, person, attribute); staff_get_player_attribute 0x5431e0 and
# staff_get_non_player_attribute 0x543f30 cdecl(person, attribute) -> al. The settings byte +8 / +9 of
# [0xdd6cc8] and the viewer [0xdd6ccc] are set by the search screen before it sorts.
# The original copies the club/nation names into stack buffers before comparing them; the copy compares the strings in
# place (same result).
TELAS = True
ROTINAS = [('search_screens_compare_staff_by_club', 0x819990, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_staff_by_employer', 0x819a80, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_staff_by_league', 0x819ba0, [0x81, 0xec, 0x34, 0x03, 0x00, 0x00]),
           ('search_screens_compare_staff_by_position', 0x819da0, [0x81, 0xec, 0x20, 0x03, 0x00, 0x00]),
           ('search_screens_compare_staff_by_job', 0x819ea0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_staff_by_value', 0x819f70, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0xbc, 0x23, 0xae, 0x00]),
           ('search_screens_compare_staff_by_reputation', 0x81a000, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_staff_by_stat', 0x81a0f0, [0x81, 0xec, 0x94, 0x01, 0x00, 0x00]),
           ('search_screens_compare_staff_by_player_attribute', 0x81a220, [0x81, 0xec, 0x98, 0x01, 0x00, 0x00]),
           ('search_screens_compare_staff_by_non_player_attribute', 0x81a360, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_staff_by_nationality', 0x81a450, [0x81, 0xec, 0x24, 0x03, 0x00, 0x00]),
           ('search_screens_compare_staff_by_age', 0x81a5a0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0xbc, 0x23, 0xae, 0x00])]
DOCS = ('as telas de busca (2): as ordens das colunas da busca de comissão',
        'the search screens (2): the sort orders of the staff search columns',
        [('search_screens_compare_staff_by_club', 'search_screens_compare_staff_by_club', '0x819990', 'search_screens.cpp', 'cdecl',
          'ordena pelo nome curto do clube da pessoa; sem clube por último; empate pelo nome',
          "orders by the short name of the person's club; no club last; ties by name"),
         ('search_screens_compare_staff_by_employer', 'search_screens_compare_staff_by_employer', '0x819a80', 'search_screens.cpp', 'cdecl',
          'ordena pelo nome curto do empregador: o clube ou, sem ele, a seleção (cargo nacional diferente de 11); sem empregador por último; empate pelo nome',
          'orders by the short name of the employer: the club or, without one, the national side (national job other than 11); no employer last; ties by name'),
         ('search_screens_compare_staff_by_league', 'search_screens_compare_staff_by_league', '0x819ba0', 'search_screens.cpp', 'cdecl',
          'ordena pela liga: a divisão de um clube da nação do clube do humano da vez pela reputação (maior primeiro), o resto pelo nome da nação (do clube, senão da pessoa), que fica depois das divisões; sem nação nenhuma por último; empate pelo nome',
          "orders by league: the division of a club in the nation of the current human's club by reputation (highest first), the rest by nation name (the club's, else the person's), placed after the divisions; no nation at all last; ties by name"),
         ('search_screens_compare_staff_by_position', 'search_screens_compare_staff_by_position', '0x819da0', 'search_screens.cpp', 'cdecl',
          'ordena pelo texto curto das posições do jogador; empate pelo nome',
          "orders by the player's short positions text; ties by name"),
         ('search_screens_compare_staff_by_job', 'search_screens_compare_staff_by_job', '0x819ea0', 'search_screens.cpp', 'cdecl',
          'ordena pelo código do cargo (no clube, senão na seleção quando não é 11), crescente; sem cargo ou cargo 0 por último; empate pelo nome',
          'orders by the job code (at the club, else at the national side when it is not 11), ascending; no job or job 0 last; ties by name'),
         ('search_screens_compare_staff_by_value', 'search_screens_compare_staff_by_value', '0x819f70', 'search_screens.cpp', 'cdecl',
          'ordena pelo valor estimado, maior primeiro; empate pelo nome',
          'orders by estimated value, highest first; ties by name'),
         ('search_screens_compare_staff_by_reputation', 'search_screens_compare_staff_by_reputation', '0x81a000', 'search_screens.cpp', 'cdecl',
          'ordena pela reputação guardada no resultado, maior primeiro: abaixo de 3750 quem tem menos de 35 anos conta como 3750, e 3750 exato conta 3751; empate pelo nome',
          'orders by the reputation stored in the result, highest first: below 3750 a person under 35 counts as 3750, and exactly 3750 counts 3751; ties by name'),
         ('search_screens_compare_staff_by_stat', 'search_screens_compare_staff_by_stat', '0x81a0f0', 'search_screens.cpp', 'cdecl',
          'ordena pela estatística escolhida (get_stat com 6 e 6,5), maior primeiro; empate pelo nome; não número conta como menor',
          'orders by the selected statistic (get_stat with 6 and 6.5), highest first; ties by name; not a number counts as lower'),
         ('search_screens_compare_staff_by_player_attribute', 'search_screens_compare_staff_by_player_attribute', '0x81a220', 'search_screens.cpp', 'cdecl',
          'ordena pelo atributo de jogador escolhido, onde quem olha o pode ver (escondido vale -1), maior primeiro; empate pelo nome',
          'orders by the selected player attribute where the viewer can see it (hidden counts -1), highest first; ties by name'),
         ('search_screens_compare_staff_by_non_player_attribute', 'search_screens_compare_staff_by_non_player_attribute', '0x81a360', 'search_screens.cpp', 'cdecl',
          'ordena pelo atributo de comissão escolhido, maior primeiro; empate pelo nome',
          'orders by the selected non-player attribute, highest first; ties by name'),
         ('search_screens_compare_staff_by_nationality', 'search_screens_compare_staff_by_nationality', '0x81a450', 'search_screens.cpp', 'cdecl',
          'ordena pela sigla de três letras da nação da pessoa; sem nação por último; empate pelo nome',
          "orders by the three-letter code of the person's nation; no nation last; ties by name"),
         ('search_screens_compare_staff_by_age', 'search_screens_compare_staff_by_age', '0x81a5a0', 'search_screens.cpp', 'cdecl',
          'ordena pela idade, mais velho primeiro; empate pelo nome',
          'orders by age, oldest first; ties by name')])
