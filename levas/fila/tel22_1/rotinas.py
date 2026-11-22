# The competition screen (comp_screens.cpp, part 1; reader tel22, the screens round, 21/11/2026): opening it on the
# fixtures, building it (the tab bar, the Print popup, the Results and Qualifiers tabs, which are inline here and
# become the named pieces comp_screen_results_tab and comp_screen_qualifiers_tab) and leaving it. Its common helpers
# (comp_lang_format, comp_crt_format, comp_screens_error_box, comp_text_item, comp_add_area, comp_set_area_scroll_row,
# comp_add_item, comp_free_list_param, comp_free_block_param) are used by tel22_2..tel22_5 (and tel23): this item goes
# first, and if it goes out, they go too.
# Conventions checked at the call sites:
# - 0x48f8e0 comp_screens_open_comp_fixtures cdecl(comp record, day, year, mode) ret 0 (as tel03_4/tel04_1 call it);
#   0x7fe600 open_screen thiscall(build 0x48fcd0, input 0x4965d0, 0, leave 0x4985e0, 0) ret 0x14; 0x7ff270
#   set_screen_param thiscall(k, v, 0) ret 0xc.
# - 0x48fcd0 comp_screens_draw_comp_screen cdecl(void) ret 0, SEH, 0x1fc0 bytes of locals: 26 MenuItems (0xd4 bytes:
#   word +0 = -1, word +2 = 0 by MenuItem::MenuItem 0x4568f0, text at +4, kind +0xcc, value +0xd0) built and
#   destroyed by the CRT vector iterators (destructor 0x5a53c0, empty); jump table 0x49134c (10 tabs) after the ret.
#   Calls: 0x41c410 background_set_comp_picture cdecl(record, 0, 0); 0x53b7c0 database_get_comp_colours cdecl(record,
#   &c1, &c2); 0x803f30 add_screen_title thiscall(c2, c1, name) ret 0xc; 0x75db00 menubar_build cdecl(4, 0); 0x48d200
#   Comp::get_stage thiscall(k) ret 4; 0x48cec0 Comp::get_current_stage thiscall ret 0; 0x4b0700
#   comp_util_is_league_division cdecl(id); 0x414ac0 AWARD_MANAGER::find_award_by_club_comp thiscall(record) ret 4 on
#   [0xad9c60]; 0x5c6710 friendly_is_friendly_competition thiscall(record) ret 4 on [0xae2c74]; 0x804030
#   add_button_item thiscall(12) ret 0x30; 0x55fba0 Display::add_area thiscall(11) ret 0x2c and 0x55f990
#   Display::add_item thiscall(18) ret 0x48 on the current human's Display; 0x5ed780 gui_add_tab_bar cdecl(items, n,
#   tab, &top, &bottom, narrow); the tabs 0x491380/0x492980 cdecl(stage, top, bottom) (tel22_2), 0x493ee0, 0x4947a0,
#   0x494c60, 0x4956e0, 0x495bc0, 0x495050 cdecl(object, top, bottom) (tel22_3), 0x79a530
#   show_comp_official_stats_screen cdecl(record, top, bottom) (not in this file); 0x68d020 LLIST::LLIST, 0x68d040
#   ~LLIST, 0x68d1b0 get_next thiscall; 0x944cf1 operator new(0xe); 0x5acee0 FIXTURE_MANAGER::scan_fixtures thiscall(18
#   args: &year, &day, &mode byte, forward, comp id, -1 x7, -2, list, 1, -2, -1, 1) ret 0x48 -> ax; 0x4c0520
#   comp_util_sort_fixture_list cdecl(list); 0x549ef0 Date::Date thiscall(day, year) ret 8; 0x54c8e0/0x54c5b0
#   sub_days/add_days thiscall(out, n) ret 8; 0x417c10 Date::operator= thiscall(out, src) ret 8; 0x54bef0/0x54ba20
#   get_long_date_string(_no_year) thiscall(buf, 0x7d0, 0xff) ret 0xc; 0x4bffc0 comp_util_format_stage_round_name
#   cdecl(code, round, buf, 0x7d0, 0); 0x5e5ad0 font_text_width cdecl(3, text); 0x48d410
#   Comp::are_round_fixtures_known thiscall(stage byte, stage index) ret 8; 0x4567f0 ScreenManager::add_area ret 0x2c,
#   0x45cb60 add_item ret 0x48, 0x4567a0 set_area_scroll_row ret 8 (tel01_1, called by address); 0x803fd0
#   add_text_item ret 0x2c, 0x8040a0 add_area_button ret 0x2c, 0x804100 add_area_text ret 0x24; 0x49db00
#   comp_screens_add_fixture_row cdecl(fixture, area, &row, 0xf); 0x600a50 human_manager_is_human_club cdecl(club, 0,
#   1, 0, 0); 0x49d070 comp_screens_add_related_comps_menu cdecl(object, c2, c1, 1, 0x18); 0x7febb0/0x7fec60/0x5edcc0
#   Back/Next.
# - 0x4985e0 comp_screens_close_comp_screen cdecl(void) ret 0: 0x68d040 ~LLIST, 0x944c94 delete, 0x9452ca free.
# Out: 0x497910 LLIST::scalar_deleting_destructor (a call rel32 at byte 3: the 5-byte hook would need an entry in the
#   detours' relocation table; it stays with the exe).
# Kept from the original / simplifications: the row counts of the Results and Qualifiers areas go as their byte (the
#   original's dword carries stack garbage above it); the colour words go in dword slots whose high half is register
#   garbage in the original (0 here).
# Defects of the original: none found.
# Comparers: opening only the original, checked against the plan (comp_screen_planned_params); the build side by side
#   by the display list (compare_match_screen_build) plus [0x99c00c] and [0xadae0c], the original only while the open
#   tab still builds a list it keeps in the params (1: param 5, 2: 0xe, 3: 0x17, 4: 0x1a, 7: 0x24, 8: 0x1f), on tab 10
#   and with no object; leaving only the original, the params checked against the plan (comp_screen_close_plan).
# Identical in cm0102-1080.exe (checked by the item's checker when integrating).
TELAS = True
ROTINAS = [('comp_screens_open_comp_fixtures', 0x48f8e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x15, 0xfc, 0xad, 0xad, 0x00]),
           ('comp_screens_draw_comp_screen', 0x48fcd0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('comp_screens_close_comp_screen', 0x4985e0, [0x56, 0x6a, 0x05, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('a tela da competição (1): abrir nos jogos, montar (a barra de abas, o botão Imprimir, as abas Resultados e Classificados) e sair',
        'the competition screen (1): opening it on the fixtures, building it (the tab bar, the Print button, the Results and Qualifiers tabs) and leaving it',
        [('comp_screens_open_comp_fixtures', 'comp_screens_open_comp_fixtures', '0x48f8e0', 'comp_screens.cpp', 'cdecl',
          'abre a tela da competição (o objeto da tabela das competições pelo id do registro) na aba Resultados numa data (com o modo) ou na aba Jogos (sem ele); 54 parâmetros: a aba, a fase atual (−2), a data na aba escolhida, 0x1e = 0x11, 0x23 = 3, 0x2a e 0x32 = −1, 0x2d = 1, o resto 0',
          "opens the competition screen (the object of the competition table by the record's id) on the Results tab at a date (with the mode) or on the Fixtures tab (without it); 54 parameters: the tab, the current stage (−2), the date in the chosen tab, 0x1e = 0x11, 0x23 = 3, 0x2a and 0x32 = −1, 0x2d = 1, the rest 0"),
         ('comp_screens_draw_comp_screen', 'comp_screens_draw_comp_screen', '0x48fcd0', 'comp_screens.cpp', 'cdecl',
          'monta a tela da competição: título e cores, a barra de abas com as abas que a competição tem (Fases/Tabelas/Tabela/Rodadas/Sorteio, Resultados, Jogos, Calendário, Classificados ou Cabeças de chave, Média de Pontos, Estatísticas dos Times e dos Jogadores, Árbitros, Prêmios, História; uma aba que não existe volta para a 1), o botão Imprimir com a lista (Tabela, Gols, Assistências, Notas Médias, Melhor em Campo, Tudo) e a aba aberta; a aba Resultados mostra um dia de jogos com as setas do dia anterior e do seguinte; a aba Classificados, os clubes em duas metades com a classe (país-sede, último campeão)',
          'builds the competition screen: title and colours, the tab bar with the tabs the competition has (Stages/Tables/Table/Rounds/Draw, Results, Fixtures, Schedule, Qualifiers or Seedings, Average Points, Team and Player Stats, Referee Stats, Awards, History; a tab that is not there goes back to 1), the Print button with its list (Table, Goals, Assists, Average Ratings, Man of Match, All) and the open tab; the Results tab shows one match day with the previous and next day arrows; the Qualifiers tab, the clubs in two halves with their class (host nation, previous winner)'),
         ('comp_screens_close_comp_screen', 'comp_screens_close_comp_screen', '0x4985e0', 'comp_screens.cpp', 'cdecl',
          'ao sair da tela da competição, libera as listas dos parâmetros 5, 0xe e 0x17 e os blocos dos parâmetros 0x1a, 0x24 e 0x1f (zerando também a contagem seguinte)',
          'on leaving the competition screen, frees the lists of parameters 5, 0xe and 0x17 and the blocks of parameters 0x1a, 0x24 and 0x1f (clearing the count after each too)')])
