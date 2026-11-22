# The competition screen's input (comp_screens.cpp, part 1; reader tel22, the screens round, 21/11/2026): the screen's
# clicks and the clicks of its Stages, Results, Fixtures, Schedule, Team Stats, Player Stats and Average Points tabs.
# - 0x4965d0 comp_screens_handle_comp_screen_click cdecl(short widget, pushed as a dword) ret 0 -> eax (-11 rebuild,
#   -4 screen changed, 0 or the menu bar's answer); 0x198 bytes of locals; jump table 0x49701c (10 tabs) inside the
#   region. Calls: 0x7d7c80 printouts_print_comp_report cdecl(comp record, kind, file, option); 0x53b7c0
#   database_get_comp_colours cdecl(comp record, &c1, &c2); 0x591840 confirm_push cdecl(7); 0x5fc5b0
#   HUMAN_MANAGER::get_folder_path thiscall(manager, 4) ret 8; 0x58f500 open_file_dialog cdecl(14); 0x48f4a0
#   comp_screens_open_comp_screen cdecl(4); 0x5455a0 comp_get_english_name cdecl; 0x5f11c0 history_screen_open cdecl;
#   0x414c20 award_screens_open_award_screen cdecl; 0x498780 comp_screens_open_history_screen cdecl(record, 0); 0x468b40
#   ScreenManager::get_widget_action_id thiscall ret 4; 0x468b90 get_widget_action_data thiscall ret 4; 0x803320
#   get_area_scroll_row thiscall ret 4; 0x497910 LLIST::scalar_deleting_destructor thiscall(1) ret 4; 0x49b830
#   comp_open_league_progress_for_fixture cdecl; the tab handlers cdecl(widget) (0x79aba0
#   official_stats_screen_handle_click is another file's); 0x7615d0 / 0x7643d0 the menu bar cdecl(widget).
# - 0x497050 comp_screens_handle_stages_tab_click cdecl(short widget) ret 0: 0x90c510 utils_get_time_ms; 0x51cb90
#   Cup::create_tie_fixtures thiscall(round, -1, 1) ret 0xc; 0x48d200 Comp::get_stage thiscall ret 4; 0x7fe600
#   open_screen (league progress 0x49b990 / 0x49bef0).
# - 0x497930 comp_screens_handle_fixtures_tab_click, 0x498300 ..._player_stats_tab_click, 0x498520
#   ..._average_points_tab_click cdecl(short widget) ret 0.
# - 0x497dd0 comp_screens_handle_schedule_tab_click cdecl(short widget) ret 0, SEH (the local LLIST): 0x5acee0
#   FIXTURE_MANAGER::scan_fixtures thiscall(18 args) ret 0x48 (the slice byte pointer is a local set to 2 / 0).
# Left out: 0x497910 LLIST::scalar_deleting_destructor (30 bytes): its first 8 bytes hold a call rel32 at offset 3, so
#   the 5-byte detour needs an entry (0x497910, 3) in detours.rs RELATIVAS, outside what an item writes; called here by
#   address.
# Defects of the original: the Stages tab's draw tail (ids 9/10 and the live draw) clears param 0x17 without freeing the
#   Fixtures tab's list (a leak).
# Kept from the original: the main handler frees lists through 0x497910, the tab handlers inline (~LLIST + delete).
# Comparers: side by side over the screen manager and the current screen's parameter block (screen_param_regions) and
#   the answer, only for the clicks that just read and write params (no list freed); opening screens, printing, the
#   file box, making fixtures, freeing lists and the menu bar only the original. The main handler runs only the
#   original when a tab with a handler of its own is open (those compare themselves).
TELAS = True
ROTINAS = [('comp_screens_handle_comp_screen_click', 0x4965d0, [0x81, 0xec, 0x98, 0x01, 0x00, 0x00]),
           ('comp_screens_handle_stages_tab_click', 0x497050, [0x53, 0x55, 0x56, 0x66, 0x8b, 0x74, 0x24, 0x10]),
           ('comp_screens_handle_fixtures_tab_click', 0x497930, [0x53, 0x56, 0x66, 0x8b, 0x74, 0x24, 0x0c]),
           ('comp_screens_handle_schedule_tab_click', 0x497dd0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('comp_screens_handle_player_stats_tab_click', 0x498300, [0x56, 0x66, 0x8b, 0x74, 0x24, 0x08]),
           ('comp_screens_handle_average_points_tab_click', 0x498520, [0x66, 0x8b, 0x54, 0x24, 0x04])]
DOCS = ('a tela da competição (entrada): os cliques da tela e das abas Fases, Resultados, Jogos, Calendário, Estatísticas dos Times, dos Jogadores e Média de Pontos',
        "the competition screen (input): the screen's clicks and those of its Stages, Results, Fixtures, Schedule, Team Stats, Player Stats and Average Points tabs",
        [('comp_screens_handle_comp_screen_click', 'comp_screens_handle_comp_screen_click', '0x4965d0', 'comp_screens.cpp', 'cdecl',
          'trata o clique na tela da competição: grava o impresso pendente (rede), abre outra competição, o histórico, os prêmios, imprime ou abre a caixa do arquivo, troca de aba; depois o clique da aba aberta (Resultados: dia anterior ou seguinte, fase e rodada, progresso da liga; Estatísticas dos Times: a estatística) e o resto vai para a barra de menu',
          "handles a click on the competition screen: writes the pending printout (network), opens another competition, the history, the awards, prints or opens the file box, switches tab; then the open tab's click (Results: earlier or later day, stage and round, league progress; Team Stats: the statistic) and the rest goes to the menu bar"),
         ('comp_screens_handle_stages_tab_click', 'comp_screens_handle_stages_tab_click', '0x497050', 'comp_screens.cpp', 'cdecl',
          'cliques da aba Fases e o sorteio ao vivo: um confronto por vez a cada 1,5 s; escolher fase e rodada, ligar o sorteio, criar os jogos de um ou de todos os confrontos, trocar a vista, abrir o progresso da liga, ir de um jogo para Resultados (jogado) ou Jogos (a jogar); defeito: o fim do sorteio zera a lista da aba Jogos sem liberá-la',
          "the Stages tab's clicks and the live draw: one tie at a time every 1.5 s; picking stage and round, starting the draw, making one or all ties' fixtures, switching the view, opening the league progress, going from a fixture to Results (played) or Fixtures (to play); defect: the draw's end clears the Fixtures tab's list without freeing it"),
         ('comp_screens_handle_fixtures_tab_click', 'comp_screens_handle_fixtures_tab_click', '0x497930', 'comp_screens.cpp', 'cdecl',
          'cliques da aba Jogos: dia anterior ou seguinte (a lista liberada), escolher fase e rodada, abrir o progresso da liga de um jogo',
          "the Fixtures tab's clicks: the earlier or later day (the list freed), picking stage and round, opening a fixture's league progress"),
         ('comp_screens_handle_schedule_tab_click', 'comp_screens_handle_schedule_tab_click', '0x497dd0', 'comp_screens.cpp', 'cdecl',
          'cliques da aba Calendário: uma data com jogos já disputados leva a Resultados naquele dia, senão com jogos marcados leva a Jogos; ou escolhe fase e rodada',
          "the Schedule tab's clicks: a date with matches played goes to Results on that day, else with matches due to Fixtures; or picks stage and round"),
         ('comp_screens_handle_player_stats_tab_click', 'comp_screens_handle_player_stats_tab_click', '0x498300', 'comp_screens.cpp', 'cdecl',
          'cliques da aba Estatísticas dos Jogadores: outra estatística ou outro filtro liberam a tabela montada para refazê-la',
          "the Player Stats tab's clicks: another statistic or filter frees the built table so it is rebuilt"),
         ('comp_screens_handle_average_points_tab_click', 'comp_screens_handle_average_points_tab_click', '0x498520', 'comp_screens.cpp', 'cdecl',
          'cliques da aba Média de Pontos: escolhe a fase (o valor / 1000 − 1)',
          "the Average Points tab's clicks: picks the stage (the value / 1000 − 1)")])
