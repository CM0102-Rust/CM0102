# The competition screen's tabs (comp_screens.cpp, part 3; reader tel22, the screens round, 21/11/2026): Fixtures,
# Schedule, Seedings, Average Points, Team Stats and Player Stats.
# - All six cdecl(Comp* comp, int top, int bottom) ret 0, called by comp_screens_draw_comp_screen (0x48fcd0, jump
#   table 0x49134c, item tel22_1) with the tab bar's top and bottom. Equal in the cm0102-1080.exe (the whole routines).
# - 0x493ee0 fixtures tab (SEH, 0xa28 bytes of locals; a local LLIST at +0x50 for the neighbour scans): 0x5acee0
#   FIXTURE_MANAGER::scan_fixtures thiscall(&year, &day, &slot, forward, comp id, -1 x7, count -2/-1, list, 0, -2, -1, 1)
#   ret 0x48 -> ax; 0x4c0520 comp_util_sort_fixture_list cdecl(list); 0x549ef0 Date::Date thiscall(day, year) ret 8;
#   0x54c8e0/0x54c5b0 Date::sub_days/add_days thiscall(out, 1) ret 8; 0x417c10 Date::operator= thiscall(out, src) ret 8;
#   0x54bef0/0x54ba20 long date string (no year) thiscall(text, 0x7d0, 0xff) ret 0xc; 0x48d410
#   Comp::are_round_fixtures_known thiscall(stage, round) ret 8; 0x4bffc0 comp_util_format_stage_round_name
#   cdecl(stage, round, text, 0x7d0, 0); 0x5e5ad0 font_text_width cdecl(3, text); 0x49db00 comp_screens_add_fixture_row
#   cdecl(fixture, area, &row, 0x13).
# - 0x4947a0 schedule tab (SEH, 0xa00): 0x48d500 Comp::build_match_date_list thiscall(&count) -> list (0x14-byte
#   entries: stage +0, round +4, day +0xc, year +0xe, stage id +0x10, round id +0x12); 0x4b01d0 parent stage cdecl;
#   0x4b0700 is_league_division cdecl; 0x54ae10 Date::get_short_weekday_date_string thiscall(text, 0x7d0, 0xff) ret 0xc;
#   0x48d2b0 Comp::get_first_stage_of_group thiscall(stage) ret 4.
# - 0x494c60 seedings tab (jump table 0x495024 inside the region, kinds 3..12); 0x600a50 is_human_club cdecl(club, 0, 1,
#   0, 0).
# - 0x495050 average points tab: the vtable slot 0x34 thiscall(comp, 0) ret 4 (update_relegation_averages, ArgSecond;
#   0x583470 elsewhere); 0x49d930 comp_screens_add_stage_menu_item cdecl(&state, 0x28f, top, 0x30c, top+0x14, "Stage",
#   stage id, 0, index, 0); 0x4b4850 comp_util_get_stage_name cdecl(text, 0x7d0, stage id, 0).
# - 0x4956e0 team stats tab: 0x4a2810 comp_stats_get_team_stat_label thiscall(stats, stat, buf, 0xc8) ret 0xc; 0x4a4b20
#   get_sorted_team_table thiscall(stat, &count, 0x4b) ret 0xc; 0x4a39c0 comp_stats_format_team_stat thiscall(entry,
#   stat, text, 0x7d0, 1) ret 0x14; 0x90c310 utils_format_ordinal cdecl(n, -1); 0x53b3d0 database_get_national_team.
# - 0x495bc0 player stats tab: 0x4a2b10 get_player_stat_label thiscall ret 0xc; 0x4a4dc0
#   Comp_Stats::get_sorted_player_table thiscall(stat, &count, 0x4b, filter) ret 0x10; 0x4a3ee0 format_player_stat
#   thiscall ret 0x14; 0x53dba0 database_format_staff_name cdecl(staff, buf, 0x7d0, 1).
# - The display list: 0x55fba0 Display::add_area, 0x55f990 Display::add_item (tel26_1), 0x804030/0x8040a0/0x804100/
#   0x803fd0 ScreenManager add_button_item/add_area_button/add_area_text/add_text_item, 0x5ed070 gui_add_text_area;
#   0x403320 Area::set_scroll_row thiscall(row) ret 4.
# Kept from the original: with no statistic ticked the Team/Player Stats heading is stack garbage (empty here); the
#   colour words go in dword slots whose high half is register garbage in the original (0 here); the statistic and the
#   filter go to the sorting and formatting calls with stack residue above the byte the callees read (0 here).
# Comparers: side by side over the display list (compare_match_screen_build); original only when the tab allocates its
#   list on the first build (parameters 0x17, 0x1a, 0x24, 0x1f) and in the error box (no competition).
TELAS = True
ROTINAS = [('comp_screens_draw_fixtures_tab', 0x493ee0, [0x6a, 0xff, 0x68, 0x55, 0x62, 0x95, 0x00]),
           ('comp_screens_draw_schedule_tab', 0x4947a0, [0x6a, 0xff, 0x68, 0x6b, 0x62, 0x95, 0x00]),
           ('comp_screens_draw_seedings_tab', 0x494c60, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xec, 0x10]),
           ('comp_screens_draw_average_points_tab', 0x495050, [0x83, 0xec, 0x18, 0x53, 0x8b, 0x5c, 0x24, 0x20]),
           ('comp_screens_draw_team_stats_tab', 0x4956e0, [0x81, 0xec, 0xb0, 0x08, 0x00, 0x00]),
           ('comp_screens_draw_player_stats_tab', 0x495bc0, [0x81, 0xec, 0x7c, 0x09, 0x00, 0x00])]
DOCS = ('a tela da competição (3): as abas Jogos, Calendário, Cabeças de chave, Média de pontos, Estatísticas dos times e dos jogadores',
        'the competition screen (3): the Fixtures, Schedule, Seedings, Average Points, Team Stats and Player Stats tabs',
        [('comp_screens_draw_fixtures_tab', 'comp_screens_draw_fixtures_tab', '0x493ee0', 'comp_screens.cpp', 'cdecl',
          'monta a aba Jogos: uma data de jogos (a atual na primeira vez; a lista dos jogos da competição a partir dela é guardada no parâmetro 0x17), botões para a data anterior e a seguinte quando existem, a data por extenso e, rodada a rodada, o nome da rodada (link quando os confrontos são conhecidos) e uma linha por jogo; sem jogos, "No fixtures have been scheduled."',
          "builds the Fixtures tab: one match date (the current one at first; the list of the competition's matches from it kept in parameter 0x17), buttons to the previous and next dates when there are any, the long date and, round by round, the round name (a link when the ties are known) and one row per match; with none, \"No fixtures have been scheduled.\""),
         ('comp_screens_draw_schedule_tab', 'comp_screens_draw_schedule_tab', '0x4947a0', 'comp_screens.cpp', 'cdecl',
          'monta a aba Calendário: uma linha por data de jogo da competição (a lista é montada uma vez e guardada nos parâmetros 0x1a/0x1b), com a data (link) e a fase e rodada daquele dia, "Game n" na fase única de uma liga; sem calendário ainda, o aviso',
          "builds the Schedule tab: one row per match date of the competition (the list built once and kept in parameters 0x1a/0x1b), with the date (a link) and the stage and round of that day, \"Game n\" in a league's single stage; not scheduled yet, the notice"),
         ('comp_screens_draw_seedings_tab', 'comp_screens_draw_seedings_tab', '0x494c60', 'comp_screens.cpp', 'cdecl',
          'monta a aba Cabeças de chave: os clubes com cabeça de chave em duas metades lado a lado, cada um com o tipo (Seeded, First/Second seeds, Pool 1 a 3, ou link para a Intertoto e a Copa dos Campeões)',
          'builds the Seedings tab: the seeded clubs in two halves side by side, each with its kind (Seeded, First/Second seeds, Pool 1 to 3, or a link to the Intertoto and the European Cup)'),
         ('comp_screens_draw_average_points_tab', 'comp_screens_draw_average_points_tab', '0x495050', 'comp_screens.cpp', 'cdecl',
          'monta a aba Média de pontos (o rebaixamento por média): atualiza as médias, um menu das fases com tabela, e por clube os pontos das duas ou três últimas temporadas, o total, os jogos e a média',
          'builds the Average Points tab (relegation by average): updates the averages, a menu of the stages with a table, and per club the points of the last two or three seasons, the total, the games and the average'),
         ('comp_screens_draw_team_stats_tab', 'comp_screens_draw_team_stats_tab', '0x4956e0', 'comp_screens.cpp', 'cdecl',
          'monta a aba Estatísticas dos times: o menu View com as 17 estatísticas, e os clubes ordenados pela escolhida (a tabela é ordenada uma vez e guardada nos parâmetros 0x24/0x25), com a posição, o clube, a nação nas competições de seleções e o valor',
          'builds the Team Stats tab: the View menu with the 17 statistics, and the clubs ranked by the chosen one (the table sorted once and kept in parameters 0x24/0x25), with the position, the club, the nation in national team competitions and the value'),
         ('comp_screens_draw_player_stats_tab', 'comp_screens_draw_player_stats_tab', '0x495bc0', 'comp_screens.cpp', 'cdecl',
          'monta a aba Estatísticas dos jogadores: os menus View (17 estatísticas) e Filter (goleiros, defensores, meias, atacantes, todos), e os jogadores ordenados (tabela guardada nos parâmetros 0x1f/0x20) com a posição, o nome, o clube ou a seleção e o valor',
          'builds the Player Stats tab: the View (17 statistics) and Filter (goalkeepers, defenders, midfielders, attackers, all) menus, and the players ranked (the table kept in parameters 0x1f/0x20) with the position, the name, the club or the national team and the value')])
