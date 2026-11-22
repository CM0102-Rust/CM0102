# The search screens (3): the sort orders of the remaining staff search columns and of the attribute name menus
# (search_screens.cpp; reader tel21, the screens round, 21/11/2026). Uses the helpers of tel21_1 (search_result_person,
# search_results_compare_names): goes after it.
# Conventions checked in the disassembly: all cdecl(a, b) ret 0 -> int (qsort comparers). The staff ones take two
# search results (the sort table of the Player and Staff Search screen builder 0x808690); the attribute name ones take
# two pointers to attribute id bytes: 0x81ae60 from the search screen builder 0x808690 and from 0x8b5a10, 0x81afa0 and
# 0x81b040 from the Filters screen builder 0x8133a0 (tel21_5), 0x81af00 from 0x808690. Callees:
# PlayerStats::get_recent_form 0x7c1950 thiscall -> al; contract_manager_get_player_morale 0x4cc2d0 thiscall(cm,
# person) -> al; Injury::get_player_injury_status 0x62c400 thiscall -> al; Injury::get_condition 0x62c590
# thiscall(injury, person, 1) -> ax; CONTRACT_MANAGER::get_contract 0x4cf5a0 thiscall -> contract; Date::operator<
# 0x456770 / operator> 0x456740 thiscall(date, lo, hi) ret 8; STAFF_CONTRACT::get_release_clause_abbreviation 0x868020
# thiscall(contract, text, 0x1e) ret 8 -> al; database_get_player_attribute_name 0x5428a0 and
# database_get_non_player_attribute_name 0x543b10 cdecl(id, text, 0xc8, short).
# Notes kept from the original: the wage and contract-end sorts give 0 (no name tie-break) when neither person has a
# contract; the release clause texts are compared with an inline strcmp (byte order), not utils_string_compare.
TELAS = True
ROTINAS = [('search_screens_compare_staff_by_caps', 0x81a630, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0xbc, 0x23, 0xae, 0x00]),
           ('search_screens_compare_staff_by_international_goals', 0x81a6c0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0xbc, 0x23, 0xae, 0x00]),
           ('search_screens_compare_staff_by_form', 0x81a750, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_staff_by_morale', 0x81a860, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_staff_by_fitness', 0x81a940, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_staff_by_wage', 0x81aa50, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_staff_by_contract_end', 0x81ab60, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_staff_by_release_clause', 0x81aca0, [0x81, 0xec, 0xd8, 0x01, 0x00, 0x00]),
           ('search_screens_compare_player_attribute_short_names', 0x81ae60, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x98, 0x01, 0x00, 0x00]),
           ('search_screens_compare_non_player_attribute_short_names', 0x81af00, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x98, 0x01, 0x00, 0x00]),
           ('search_screens_compare_player_attribute_names', 0x81afa0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x98, 0x01, 0x00, 0x00]),
           ('search_screens_compare_non_player_attribute_names', 0x81b040, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x98, 0x01, 0x00, 0x00])]
DOCS = ('as telas de busca (3): as ordens das demais colunas da busca de comissão e dos nomes dos atributos',
        'the search screens (3): the sort orders of the remaining staff search columns and of the attribute names',
        [('search_screens_compare_staff_by_caps', 'search_screens_compare_staff_by_caps', '0x81a630', 'search_screens.cpp', 'cdecl',
          'ordena pelas convocações pela seleção, mais primeiro; empate pelo nome',
          'orders by international caps, most first; ties by name'),
         ('search_screens_compare_staff_by_international_goals', 'search_screens_compare_staff_by_international_goals', '0x81a6c0', 'search_screens.cpp', 'cdecl',
          'ordena pelos gols pela seleção, mais primeiro; empate pelo nome',
          'orders by international goals, most first; ties by name'),
         ('search_screens_compare_staff_by_form', 'search_screens_compare_staff_by_form', '0x81a750', 'search_screens.cpp', 'cdecl',
          'ordena pela forma recente do jogador, melhor primeiro; quem não é jogador por último; empate pelo nome',
          'orders by the recent form of the player, best first; non-players last; ties by name'),
         ('search_screens_compare_staff_by_morale', 'search_screens_compare_staff_by_morale', '0x81a860', 'search_screens.cpp', 'cdecl',
          'ordena pelo moral do jogador, maior primeiro; empate pelo nome',
          'orders by player morale, highest first; ties by name'),
         ('search_screens_compare_staff_by_fitness', 'search_screens_compare_staff_by_fitness', '0x81a940', 'search_screens.cpp', 'cdecl',
          'ordena pela situação de lesão, crescente, depois pela condição física, maior primeiro; empate pelo nome',
          'orders by injury status, ascending, then by condition, highest first; ties by name'),
         ('search_screens_compare_staff_by_wage', 'search_screens_compare_staff_by_wage', '0x81aa50', 'search_screens.cpp', 'cdecl',
          'ordena pelo salário semanal do contrato principal, menor primeiro; sem contrato primeiro (os dois sem contrato: empate sem desempate pelo nome); empate pelo nome',
          'orders by the weekly wage of the main contract, lowest first; no contract first (both without: a tie with no name tie-break); ties by name'),
         ('search_screens_compare_staff_by_contract_end', 'search_screens_compare_staff_by_contract_end', '0x81ab60', 'search_screens.cpp', 'cdecl',
          'ordena pelo fim do contrato principal, o mais cedo primeiro; sem contrato como no salário; empate pelo nome',
          'orders by the end of the main contract, earliest first; no contract as in the wage sort; ties by name'),
         ('search_screens_compare_staff_by_release_clause', 'search_screens_compare_staff_by_release_clause', '0x81aca0', 'search_screens.cpp', 'cdecl',
          'ordena pelo código da cláusula de rescisão (maior primeiro; sem contrato vale -1 e o texto "  -"), depois pelo texto da cláusula (strcmp); empate pelo nome',
          'orders by the release clause code (highest first; no contract counts -1 with the text "  -"), then by the clause text (strcmp); ties by name'),
         ('search_screens_compare_player_attribute_short_names', 'search_screens_compare_player_attribute_short_names', '0x81ae60', 'search_screens.cpp', 'cdecl',
          'ordena dois atributos de jogador (1 a 31) pela sigla de três letras; inválidos por último',
          'orders two player attributes (1 to 31) by their three-letter names; invalid ones last'),
         ('search_screens_compare_non_player_attribute_short_names', 'search_screens_compare_non_player_attribute_short_names', '0x81af00', 'search_screens.cpp', 'cdecl',
          'ordena dois atributos de comissão (1 a 12) pela sigla de três letras; inválidos por último',
          'orders two non-player attributes (1 to 12) by their three-letter names; invalid ones last'),
         ('search_screens_compare_player_attribute_names', 'search_screens_compare_player_attribute_names', '0x81afa0', 'search_screens.cpp', 'cdecl',
          'ordena dois atributos de jogador (1 a 31) pelo nome completo; inválidos por último',
          'orders two player attributes (1 to 31) by their long names; invalid ones last'),
         ('search_screens_compare_non_player_attribute_names', 'search_screens_compare_non_player_attribute_names', '0x81b040', 'search_screens.cpp', 'cdecl',
          'ordena dois atributos de comissão (1 a 12) pelo nome completo; inválidos por último',
          'orders two non-player attributes (1 to 12) by their long names; invalid ones last')])
