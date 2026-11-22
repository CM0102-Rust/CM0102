# The search screens (1): the sort orders of the player and staff search results (search_screens.cpp; reader tel21,
# the screens round, 21/11/2026).
# Conventions checked in the disassembly: all cdecl(const search_result* a, const search_result* b) ret 0 -> int (qsort
# comparers; the pointers sit in the sort table of the Player and Staff Search screen builder 0x808690, and the note
# rating and status code ones are also used by the search engine 0x7af160). The callees: database_format_staff_name
# 0x53dba0 cdecl(person, buffer, 0xc8, 3); utils_string_compare 0x90c1d0 cdecl(a, b, 1, 0);
# SCOUT_MANAGER::find_club_search_entry 0x7f8610 thiscall(manager, person) ret 4 (already Rust); the slot sums
# 0x7bd870/0x7bd8c0/0x7bd910 thiscall -> ax; 0x7bd960 thiscall -> st0 (double); 0x7bd830 thiscall(slot, char) ret 4;
# Date::Date 0x549ef0 thiscall(date, day, year) ret 8; national_teams_get_player_squad_slot 0x76e070 thiscall(nt,
# person) ret 4 -> al.
# Every tie-break is the two names (style 3), `a` formatted first, compared as utils_string_compare(name a, name b, 1, 0).
# The cm0102-1080.exe changes no byte of 0x813140..0x81b0e0.
# This item holds the helpers tel21_2 and tel21_3 use (search_result_keys, search_result_person,
# search_results_compare_names): it goes first.
# DEFECT of the original kept: 0x819340 reads the date at address 0 when `a` has a last scouted match and `b`'s slot
# history is empty (the original crashes there too).
TELAS = True
ROTINAS = [('search_screens_compare_results_by_note_rating', 0x8189f0, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_results_by_best_stars', 0x818ab0, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_results_by_matches_watched', 0x818b60, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_results_by_scouted_goals', 0x818ce0, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_results_by_scouted_assists', 0x818e70, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_results_by_scouted_goals_conceded', 0x819000, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_results_by_scouted_average_rating', 0x819190, [0x81, 0xec, 0x98, 0x01, 0x00, 0x00]),
           ('search_screens_compare_results_by_last_scouted_match', 0x819340, [0x6a, 0xff, 0x68, 0xbc, 0x12, 0x96, 0x00]),
           ('search_screens_compare_results_by_ability_stars', 0x819620, [0x8b, 0x54, 0x24, 0x04, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_results_by_potential_stars', 0x8196e0, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x54, 0x24, 0x04]),
           ('search_screens_compare_results_by_national_squad_slot', 0x819780, [0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('search_screens_compare_results_by_status_code', 0x819870, [0x8b, 0x54, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x08]),
           ('search_screens_compare_results_by_name', 0x819910, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00])]
DOCS = ('as telas de busca (1): as ordens dos resultados da busca de jogadores e comissão',
        'the search screens (1): the sort orders of the player and staff search results',
        [('search_screens_compare_results_by_note_rating', 'search_screens_compare_results_by_note_rating', '0x8189f0', 'search_screens.cpp', 'cdecl',
          'ordena dois resultados pela nota do técnico, maior primeiro, sem nota (-1) por último; empate pelo nome',
          "orders two results by the manager's note rating, highest first, unrated (-1) last; ties by name"),
         ('search_screens_compare_results_by_best_stars', 'search_screens_compare_results_by_best_stars', '0x818ab0', 'search_screens.cpp', 'cdecl',
          'ordena pela maior das estrelas de habilidade e de potencial, melhor primeiro; empate pelo nome',
          'orders by the higher of the ability and potential stars, best first; ties by name'),
         ('search_screens_compare_results_by_matches_watched', 'search_screens_compare_results_by_matches_watched', '0x818b60', 'search_screens.cpp', 'cdecl',
          'ordena pelos jogos que o olheiro viu, mais primeiro; quem nenhum olheiro acompanha por último; empate pelo nome',
          'orders by the matches the scout watched, most first; a person no scout follows last; ties by name'),
         ('search_screens_compare_results_by_scouted_goals', 'search_screens_compare_results_by_scouted_goals', '0x818ce0', 'search_screens.cpp', 'cdecl',
          'ordena pelos gols nos jogos vistos pelo olheiro, mais primeiro (diferença em 16 bits); sem olheiro por último; empate pelo nome',
          'orders by the goals in the matches the scout watched, most first (16-bit difference); unfollowed last; ties by name'),
         ('search_screens_compare_results_by_scouted_assists', 'search_screens_compare_results_by_scouted_assists', '0x818e70', 'search_screens.cpp', 'cdecl',
          'ordena pelas assistências nos jogos vistos pelo olheiro, como os gols',
          'orders by the assists in the matches the scout watched, as the goals'),
         ('search_screens_compare_results_by_scouted_goals_conceded', 'search_screens_compare_results_by_scouted_goals_conceded', '0x819000', 'search_screens.cpp', 'cdecl',
          'ordena pelos gols sofridos nos jogos vistos pelo olheiro, mais primeiro, como os gols',
          'orders by the goals conceded in the matches the scout watched, most first, as the goals'),
         ('search_screens_compare_results_by_scouted_average_rating', 'search_screens_compare_results_by_scouted_average_rating', '0x819190', 'search_screens.cpp', 'cdecl',
          'ordena pela nota média nos jogos vistos pelo olheiro, maior primeiro; sem olheiro por último; empate pelo nome; não número conta como menor',
          'orders by the average rating in the matches the scout watched, highest first; unfollowed last; ties by name; not a number counts as lower'),
         ('search_screens_compare_results_by_last_scouted_match', 'search_screens_compare_results_by_last_scouted_match', '0x819340', 'search_screens.cpp', 'cdecl',
          'ordena pela data do último jogo visto pelo olheiro (ano, depois dia), o mais recente primeiro; sem jogo visto por último; empate pelo nome (defeito do original: lê a data no endereço 0 quando só o segundo não tem jogo)',
          'orders by the date of the last match the scout watched (year, then day), latest first; none watched last; ties by name (defect of the original: reads the date at address 0 when only the second has no match)'),
         ('search_screens_compare_results_by_ability_stars', 'search_screens_compare_results_by_ability_stars', '0x819620', 'search_screens.cpp', 'cdecl',
          'ordena pelas estrelas de habilidade, melhor primeiro; resultado nulo por último; empate pelo nome',
          'orders by the ability stars, best first; a null result last; ties by name'),
         ('search_screens_compare_results_by_potential_stars', 'search_screens_compare_results_by_potential_stars', '0x8196e0', 'search_screens.cpp', 'cdecl',
          'ordena pelas estrelas de potencial, melhor primeiro; empate pelo nome',
          'orders by the potential stars, best first; ties by name'),
         ('search_screens_compare_results_by_national_squad_slot', 'search_screens_compare_results_by_national_squad_slot', '0x819780', 'search_screens.cpp', 'cdecl',
          'ordena pela vaga na convocação da seleção, menor primeiro; fora da convocação (-1, contado como 127) por último; empate pelo nome',
          'orders by the slot in the national squad, lowest first; outside the squad (-1, counted as 127) last; ties by name'),
         ('search_screens_compare_results_by_status_code', 'search_screens_compare_results_by_status_code', '0x819870', 'search_screens.cpp', 'cdecl',
          'ordena pelo código de situação do jogador, crescente; empate pelo nome',
          'orders by the player status code, ascending; ties by name'),
         ('search_screens_compare_results_by_name', 'search_screens_compare_results_by_name', '0x819910', 'search_screens.cpp', 'cdecl',
          'ordena só pelo nome (estilo 3, comparação de texto sem caixa)',
          'orders by name only (style 3, case-insensitive text comparison)')])
