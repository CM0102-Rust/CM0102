# The league progress, FIFA world rankings and UEFA coefficients screens (comp_screens.cpp, part 2; reader tel23, the
# screens round, 21/11/2026), and the fixture -> league clubs lookup the progress screen opens with. Depends on tel22_1
# (comp_lang_format, comp_crt_format, comp_screens_error_box, comp_free_block_param), tel22_4 (comp_click_keep_scroll,
# comp_click_menubar, comp_click_compare) and tel26_1 (tel26_add_area, tel26_area_button, tel26_area_text,
# tel26_back_next, tel26_item_id, tel26_item_value, display_area_at, JOB_TEXT, job_colour, JOB_COLOUR_3192).
# Conventions checked at the call sites:
# - 0x49b830 cdecl(fixture) ret 0 (called by 0x468c10, 0x4965d0, 0x497930, 0x718930); 0x7fe600 open_screen
#   thiscall(0x49b990, 0x49bef0, 0, 0, 0) ret 0x14; 0x7ff270 set_screen_param thiscall(k, v, 0) ret 0xc.
# - 0x49ecd0 cdecl(fixture, &league, &home row, &away row) -> eax; 0x7e6f20 comp_is_reserve_or_minor_comp
#   thiscall([0xb74434], fixture +0x14) ret 4; 0x6826d0 League::get_table_row thiscall(k) ret 4.
# - 0x49b990 cdecl(void), SEH, 0x11b8 bytes of locals (chkstk): 0x41c410 background_set_comp_picture cdecl(comp, 0, 0);
#   0x53b7c0 database_get_comp_colours; 0x803f30 add_screen_title thiscall(c2, c1, name) ret 0xc; 0x4b4850
#   comp_util_get_stage_name cdecl(buf, 0x7d0, stage word, 0); 0x803f80 add_screen_subtitle thiscall(text, 1) ret 8;
#   0x75db00 menubar_build cdecl(4, 0); 0x55fba0 Display::add_area ret 0x2c; 0x6827d0 League::sort_table thiscall(0)
#   ret 4 (Rust); 0x944cf1 operator new(0x1c); Plot 0x7c6fc0 ctor thiscall(1, clubs, games, 1, series) ret 0x14,
#   0x7c7100 add_series thiscall(n, colour, name) ret 0xc, 0x7c7240 add_point thiscall(series, x, y) ret 0xc, 0x7c7340
#   draw_line_graph thiscall(0x73, 0x96, 0x25d, 0x212, x label, y label) ret 0x18, 0x7c70a0 dtor; 0x944c94 delete;
#   0x90c310 utils_format_ordinal cdecl(n, 3); 0x8040a0 add_area_button ret 0x2c; 0x804100 add_area_text ret 0x24;
#   0x7febb0/0x7fec60/0x5edcc0 Back/Next; 0x403320 Area::set_scroll_row thiscall(row) ret 4.
# - 0x49bef0, 0x49c510, 0x49cc10 cdecl(widget) -> eax; 0x803320 get_area_scroll_row thiscall(area) ret 4;
#   0x7615d0/0x7643d0 menu bar cdecl(widget).
# - 0x49c0b0 cdecl(void): open_screen(0x49c120, 0x49c510, 0, 0x49c6c0, 0). 0x49c700 cdecl(void): open_screen(0x49c740,
#   0x49cc10, 0, 0, 0). 0x49c6c0 cdecl(void): 0x9452ca crt free.
# - 0x49c120 cdecl(void), frame aligned to 8: 0x41c100 background_set_nation_picture cdecl(0, 0, 0); 0x54cfb0
#   Date::get_month thiscall ret 0; 0x54d3b0 date_get_month_name cdecl(month); 0x5e5cd0 font_get_height cdecl(3);
#   0x58d610 fifa_rankings_collect_ranked_order thiscall(&count) ret 4; 0x5ede20 gui_add_page_buttons cdecl(0x30c,
#   0x91, page, pages, 1, 2, 3, 4); 0x53b3d0 database_get_national_team cdecl(nation); 0x944c9f sprintf "%4.2f" of
#   the f32 +0xc times the f32 10.0 at 0x968a78 (x87 product of two singles, exact as a double).
# - 0x49c740 cdecl(void), frame aligned to 8: 0x903590 UefaSeeding::get_nation_table thiscall(&table) ret 4 -> al.
# Kept from the original / simplifications: the row counts go as their byte (the progress screen's dword carries the
#   freed Plot pointer's high bytes; Display keeps the byte); the colour words go in dword slots whose high half is
#   register garbage in the original (0 here).
# Defects of the original: none found.
# Comparers: the lookup side by side by value (the error box with an argument missing only the original); the
#   openers only the original, the params checked against the plan (league progress: the lookup run before; FIFA:
#   0, 0, 1, 1); the builds side by side by the display list (progress: plus the league's table column; FIFA once the
#   ranked list exists, its first build allocating it only the original); the progress rows and the FIFA page buttons
#   side by side over the screen manager and the params; the menu bar (and the whole UEFA handler) only the original;
#   leaving the FIFA screen only the original, params 0/1 checked.
TELAS = True
ROTINAS = [('comp_open_league_progress_for_fixture', 0x49b830, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('comp_league_progress_screen_draw', 0x49b990, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('comp_league_progress_screen_handle_event', 0x49bef0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('comp_fixture_resolve_league_clubs', 0x49ecd0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_open_fifa_rankings_screen', 0x49c0b0, [0x6a, 0x00, 0x68, 0xc0, 0xc6, 0x49, 0x00]),
           ('comp_fifa_rankings_screen_draw', 0x49c120, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('comp_fifa_rankings_screen_handle_event', 0x49c510, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('comp_fifa_rankings_screen_close', 0x49c6c0, [0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('comp_open_uefa_coefficients_screen', 0x49c700, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x00]),
           ('comp_uefa_coefficients_screen_draw', 0x49c740, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('comp_uefa_coefficients_screen_handle_event', 0x49cc10, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('as telas de evolução na liga, ranking da FIFA e coeficientes da UEFA',
        'the league progress, FIFA world rankings and UEFA coefficients screens',
        [('comp_open_league_progress_for_fixture', 'comp_open_league_progress_for_fixture', '0x49b830', 'comp_screens.cpp', 'cdecl',
          'abre a tela de evolução na liga de um jogo: a liga e as linhas dos dois clubes pela busca do jogo (sem jogo, ou sem nenhuma das linhas, a caixa de erro; um ponteiro abaixo de 10000 não faz nada); parâmetros 0 a liga, 1 e 2 as linhas, 3 e 4 zero',
          "opens a fixture's league progress screen: the league and the two clubs' rows from the fixture lookup (no fixture, or neither row, the error box; a pointer below 10000 does nothing); params 0 the league, 1 and 2 the rows, 3 and 4 zero"),
         ('comp_league_progress_screen_draw', 'comp_league_progress_screen_draw', '0x49b990', 'comp_screens.cpp', 'cdecl',
          'monta a tela de evolução na liga: título nas cores da competição, "<fase> Progress" ou "League Progress", a tabela ordenada, o gráfico das posições jogo a jogo do clube da casa (amarelo) e do visitante (vermelho) e a tabela ao lado com a posição e o clube (os dois destacados), cada linha um botão; Voltar/Avançar',
          'builds the league progress screen: title in the competition colours, "<stage> Progress" or "League Progress", the table sorted, the graph of the home (yellow) and away (red) clubs\' positions game by game and the table beside it with position and club (the two highlighted), each row a button; Back/Next'),
         ('comp_league_progress_screen_handle_event', 'comp_league_progress_screen_handle_event', '0x49bef0', 'comp_screens.cpp', 'cdecl',
          'os cliques da tela de evolução: clicar uma linha da tabela tira o destaque dela ou a destaca (no lugar do primeiro clube se vazio, senão do segundo); o resto vai à barra de menus',
          "the progress screen's clicks: clicking a table row unhighlights it or highlights it (in the first club's place when empty, else the second's); the rest goes to the menu bar"),
         ('comp_fixture_resolve_league_clubs', 'comp_fixture_resolve_league_clubs', '0x49ecd0', 'comp_screens.cpp', 'cdecl',
          'a liga (ou a fase de liga) de um jogo e as linhas da tabela dos dois clubes; 0 para competições reservas ou menores, fases que não são liga ou um clube que não está na tabela',
          "a fixture's league (or league stage) and the two clubs' table rows; 0 for reserve or minor competitions, stages that are not leagues or a club not in the table"),
         ('comp_open_fifa_rankings_screen', 'comp_open_fifa_rankings_screen', '0x49c0b0', 'comp_screens.cpp', 'cdecl',
          'abre a tela do ranking da FIFA: sem lista, página 1 de 1',
          'opens the FIFA world rankings screen: no list, page 1 of 1'),
         ('comp_fifa_rankings_screen_draw', 'comp_fifa_rankings_screen_draw', '0x49c120', 'comp_screens.cpp', 'cdecl',
          'monta a tela do ranking da FIFA: título nas cores da Copa do Mundo, o mês e o ano do ranking, os botões de página e uma página de seleções (posição, nome com botão para a seleção, continente, pontos × 10); a lista é montada na primeira vez',
          'builds the FIFA world rankings screen: title in the World Cup colours, the ranking\'s month and year, the page buttons and one page of nations (rank, name with a button to the national team, continent, points × 10); the list is built the first time'),
         ('comp_fifa_rankings_screen_handle_event', 'comp_fifa_rankings_screen_handle_event', '0x49c510', 'comp_screens.cpp', 'cdecl',
          'os botões de página do ranking: 10 para trás, 1 para trás, 1 para a frente, 10 para a frente, entre 1 e o total de páginas; o resto vai à barra de menus',
          "the rankings' page buttons: 10 back, 1 back, 1 on, 10 on, between 1 and the page count; the rest goes to the menu bar"),
         ('comp_fifa_rankings_screen_close', 'comp_fifa_rankings_screen_close', '0x49c6c0', 'comp_screens.cpp', 'cdecl',
          'ao sair do ranking da FIFA, libera a lista (parâmetro 0) e zera os parâmetros 0 e 1',
          'on leaving the FIFA rankings, frees the list (param 0) and clears params 0 and 1'),
         ('comp_open_uefa_coefficients_screen', 'comp_open_uefa_coefficients_screen', '0x49c700', 'comp_screens.cpp', 'cdecl',
          'abre a tela dos coeficientes da UEFA (parâmetros 0 e 1 zero)',
          'opens the UEFA coefficients screen (params 0 and 1 zero)'),
         ('comp_uefa_coefficients_screen_draw', 'comp_uefa_coefficients_screen_draw', '0x49c740', 'comp_screens.cpp', 'cdecl',
          'monta a tela dos coeficientes da UEFA: título nas cores da Liga dos Campeões, a temporada, as colunas ECC, UEFA e INTER (botões para as copas) e Coef., e cada país: posição, nome com botão para a seleção, as vagas em cada copa e o coeficiente',
          'builds the UEFA coefficients screen: title in the Champions League colours, the season, the ECC, UEFA and INTER columns (buttons to the cups) and Coef., and each nation: rank, name with a button to its national team, its places in each cup and its coefficient'),
         ('comp_uefa_coefficients_screen_handle_event', 'comp_uefa_coefficients_screen_handle_event', '0x49cc10', 'comp_screens.cpp', 'cdecl',
          'os cliques dos coeficientes: guarda a rolagem (parâmetro 0) e tudo vai à barra de menus',
          "the coefficients' clicks: keeps the scroll (param 0) and everything goes to the menu bar")])
