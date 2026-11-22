# The squad list comparators (3) and the staff, attribute and date orders: club_screens.cpp (reader tel30, the screens
# round, 21/11/2026; a new group from the disassembly sweep, passed by address to qsort: the player orders by
# build_filtered_sorted_squad_list 0x4841d0, the staff orders by 0x460590, the attribute order by 0x456900 / 0x475520 /
# 0x47c180, the date order by 0x470fe0).
# Conventions checked in the disassembly: every routine is a qsort comparator cdecl(a, b) ret 0 -> eax.
# - 0x85f9f0 STAFF_CONTRACT::is_expired / 0x85fa20 is_rolling thiscall(contract) -> al; 0x85fa00 days_to_end_date
#   thiscall(contract) -> ax; 0x868210 check_fifa_protected_period thiscall(contract, 0, 0, 0) ret 0xc -> al;
#   0x7c1e30 PlayerStats::get_stat thiscall on [G_PLAYER_STATS](player record, 6, field, 6.5f) ret 0x10 -> st0: the
#   second player's stored with fstp dword, then fsubr and 0x945640 _ftol, the low word tested (test ax, ax);
#   0x5428a0 database_get_player_attribute_name cdecl(code, buffer, 200, 1); 0x456770 / 0x456740 Date::operator< /
#   operator> thiscall(date, Date by value) ret 8.
# Identical in cm0102-1080.exe (the whole routines).
# Depends on: tel30_5 (player_sort_by_name, player_sort_pair, player_sort_compare), tel30_6 (player_sort_list_contract,
#   player_sort_by_float_stat).
# Comparers: each comparator's return side by side with the original (pure).
# Defects of the original: none found.
MOTOR = True
TELAS = True
ROTINAS = [('compare_players_by_contract_expiry_then_name', 0x486c70, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('compare_players_by_fifa_protected_period_then_name', 0x486e60, [0x81, 0xec, 0x94, 0x01, 0x00, 0x00]),
           ('compare_players_by_value_then_name', 0x486f80, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('compare_players_by_appearances_then_name', 0x487030, [0x81, 0xec, 0x94, 0x01, 0x00, 0x00]),
           ('compare_players_by_goals_then_name', 0x487140, [0x81, 0xec, 0x94, 0x01, 0x00, 0x00]),
           ('compare_players_by_goals_conceded_then_name', 0x487250, [0x81, 0xec, 0x94, 0x01, 0x00, 0x00]),
           ('compare_players_by_assists_then_name', 0x487360, [0x81, 0xec, 0x94, 0x01, 0x00, 0x00]),
           ('compare_players_by_average_rating_then_name', 0x487470, [0x81, 0xec, 0x94, 0x01, 0x00, 0x00]),
           ('compare_staff_by_club_job_then_name', 0x4875a0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('compare_staff_by_nation_job_then_name', 0x487660, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('compare_attributes_by_name', 0x487720, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x98, 0x01, 0x00, 0x00]),
           ('compare_dates_ascending', 0x4877c0, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
_A = 'club_screens.cpp'
DOCS = ('as ordens das listas do elenco (3) e as da comissão, dos atributos e das datas: contrato, período protegido, valor, jogos, gols, gols sofridos, assistências, nota média, cargo no clube e na seleção, nome do atributo e data',
        'the squad list orders (3) and the staff, attribute and date orders: contract, protected period, value, appearances, goals, goals conceded, assists, average rating, club and nation job, attribute name and date',
        [('compare_players_by_contract_expiry_then_name', 'compare_players_by_contract_expiry_then_name', '0x486c70', _A, 'cdecl',
          'ordena pelo contrato que a lista mostra: sem contrato por último, os vencidos e os renováveis antes, depois os dias até o fim (o menor primeiro), depois pelo nome',
          'orders by the contract the list shows: no contract last, the expired and the rolling ones first, then the days to its end (fewest first), then by name'),
         ('compare_players_by_fifa_protected_period_then_name', 'compare_players_by_fifa_protected_period_then_name', '0x486e60', _A, 'cdecl',
          'ordena pelo período protegido da FIFA do contrato (check_fifa_protected_period; 0 sem contrato), depois pelo nome',
          "orders by the contract's FIFA protected period (check_fifa_protected_period; 0 without a contract), then by name"),
         ('compare_players_by_value_then_name', 'compare_players_by_value_then_name', '0x486f80', _A, 'cdecl',
          'ordena pelo valor, o maior primeiro, depois pelo nome', 'orders by value, highest first, then by name'),
         ('compare_players_by_appearances_then_name', 'compare_players_by_appearances_then_name', '0x487030', _A, 'cdecl',
          'ordena pelos jogos na temporada (estatística 6/1, a diferença truncada para palavra), mais primeiro, depois pelo nome',
          'orders by season appearances (statistic 6/1, the difference truncated to a word), most first, then by name'),
         ('compare_players_by_goals_then_name', 'compare_players_by_goals_then_name', '0x487140', _A, 'cdecl',
          'ordena pelos gols na temporada (estatística 6/2), mais primeiro, depois pelo nome', 'orders by season goals (statistic 6/2), most first, then by name'),
         ('compare_players_by_goals_conceded_then_name', 'compare_players_by_goals_conceded_then_name', '0x487250', _A, 'cdecl',
          'ordena pelos gols sofridos na temporada (estatística 6/3), mais primeiro, depois pelo nome', 'orders by season goals conceded (statistic 6/3), most first, then by name'),
         ('compare_players_by_assists_then_name', 'compare_players_by_assists_then_name', '0x487360', _A, 'cdecl',
          'ordena pelas assistências na temporada (estatística 6/5), mais primeiro, depois pelo nome', 'orders by season assists (statistic 6/5), most first, then by name'),
         ('compare_players_by_average_rating_then_name', 'compare_players_by_average_rating_then_name', '0x487470', _A, 'cdecl',
          'ordena pela nota média (estatística 6/0x11), a maior primeiro, depois pelo nome', 'orders by average rating (statistic 6/0x11), highest first, then by name'),
         ('compare_staff_by_club_job_then_name', 'compare_staff_by_club_job_then_name', '0x4875a0', _A, 'cdecl',
          'ordena a comissão pelo cargo no clube (o 12 conta como 5), depois pelo nome', 'orders staff by club job (12 counts as 5), then by name'),
         ('compare_staff_by_nation_job_then_name', 'compare_staff_by_nation_job_then_name', '0x487660', _A, 'cdecl',
          'ordena a comissão pelo cargo na seleção (o 12 conta como 5), depois pelo nome', 'orders staff by nation job (12 counts as 5), then by name'),
         ('compare_attributes_by_name', 'compare_attributes_by_name', '0x487720', _A, 'cdecl',
          'ordena códigos de atributo pelo nome longo; códigos fora de 1 a 31 vão por último',
          'orders attribute codes by their long name; codes outside 1 to 31 go last'),
         ('compare_dates_ascending', 'compare_dates_ascending', '0x4877c0', _A, 'cdecl',
          'ordena datas em ordem crescente (um ponteiro nulo vai por último)', 'orders dates ascending (a null pointer goes last)')])
