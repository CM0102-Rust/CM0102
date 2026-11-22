# The competition's official stats tab (officials_manager.cpp; reader tel36, the screens round, 21/11/2026). Called by
# the competition screen builder 0x48fcd0 for its officials tab. Uses tel36_1 (tel36_langlib, TEL36_CLUB_COMP_SIZE)
# and tel26_1 (tel26_add_area, tel26_area_button, tel26_area_text, tel26_text_area, tel26_back_next,
# display_area_at, job_colour).
# Conventions checked in the disassembly: cdecl(&comp id, top, bottom) (the third argument's slot is reused for the
# rows byte, the first's for comp id × 0x6b). Calls: background_set_random_picture 0x41bfb0 cdecl; add_text_item
# 0x803fd0 thiscall(SM, 11 dwords) ret 0x2c; OfficialsManager::get_comp_official_count 0x79d300 thiscall(mgr, comp)
# ret 4 -> ax; copy_comp_official_stats 0x79d1f0 thiscall(mgr, comp, list) ret 8; malloc 0x944e46 and qsort 0x945333
# cdecl (comparer 0x79b150); utils_format_ordinal 0x90c310 cdecl(k, -1); Area::set_scroll_row 0x403320 thiscall(area,
# row) ret 4. The rating test is x87 (fld f32, fcomp against the 0.0 at 0x9689a0, C3 also set when unordered: a NaN
# shows "----").
# Equal in cm0102-1080.exe.
# Comparer: side by side over the display list when the list (parameter 0x2e) already exists; building it only the
# original (malloc).
TELAS = True
ROTINAS = [('show_comp_official_stats_screen', 0x79a530, [0x83, 0xec, 0x10, 0x8b, 0x44, 0x24, 0x14])]
DOCS = ('a aba das estatísticas dos árbitros de uma competição',
        "the competition's official stats tab",
        [('show_comp_official_stats_screen', 'show_comp_official_stats_screen', '0x79a530', 'officials_manager.cpp', 'cdecl',
          'monta a aba: "Official Stats"; sem a lista guardada, conta os árbitros da competição e copia as estatísticas deles numa lista nova, ordenada por jogos; com árbitros, a tabela (ordinal, nome, jogos, vermelhos, amarelos, rigor: Harsh, Strict, Lenient ou Fair, e a nota média com 2 casas, ou "----" quando é 0) com a rolagem guardada; sem árbitros, "There are no official stats available."; os botões Voltar / Avançar',
          'builds the tab: "Official Stats"; with no list kept, counts the competition\'s referees and copies their stats into a new list sorted by games; with referees, the table (ordinal, name, games, reds, yellows, harshness: Harsh, Strict, Lenient or Fair, and the average rating with 2 decimals, or "----" when it is 0) with the kept scroll; with none, "There are no official stats available."; the Back / Next buttons')])
