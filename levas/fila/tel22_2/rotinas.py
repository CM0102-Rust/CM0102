# The competition screen (comp_screens.cpp, part 2; reader tel22, the screens round, 21/11/2026): the league Table
# tab and the cup Draw tab, both called by comp_screens_draw_comp_screen (0x48fcd0) as cdecl(stage, top, bottom)
# (the tab bar's top and bottom), ret 0.
# Conventions checked at the call sites: 0x49d380 comp_screens_add_stage_selector cdecl(comp, stage, top) -> eax;
#   0x6826d0 League::get_table_row thiscall(k) ret 4; 0x6827d0 League::sort_table thiscall(0) ret 4; 0x521790
#   Cup::get_fixture_side_teams thiscall(round, tie, side, short*) ret 0x10 (already Rust); 0x519d90
#   Cup::find_scheduled_fixture_for_tie thiscall(round, tie) ret 8; 0x54a650 Date::get_short_date_string
#   thiscall(buf, 0x7d0, 0xff) ret 0xc; 0x54ba20 Date::get_long_date_string the same; 0x54c770/0x54cab0 add/sub weeks
#   thiscall(Date* out, n) ret 8 and 0x417c10 Date::operator= thiscall(out, src) ret 8; the competition's virtual +0x3c
#   thiscall(stage, short* count, *, *) -> the round dates (0x41-byte entries, freed with crt free); 0x5acbe0
#   FixtureManager::find_fixtures thiscall, the two dates passed as the dwords at +0 and +2 of each Date; 0x729500
#   Match_Stats::fixture_entry_matches thiscall(slot, home id, away id, day, year) ret 0x14; ScreenManager
#   add_text_item 0x803fd0 ret 0x2c, add_button_item 0x804030 ret 0x30, add_area_button 0x8040a0 ret 0x2c,
#   add_area_text 0x804100 ret 0x24; Display::add_area 0x55fba0 ret 0x2c and add_item 0x55f990 ret 0x48 on the
#   current human's Display; gui_add_text_area 0x5ed070 cdecl(11).
# Kept from the original: the colour words go in dword slots whose high half is register garbage (0 here); in the
#   cup tab, a tie with status 1 places the next tie after the larger of the previous ends (uninitialised stack for
#   the first tie, 0 here); the bye mask is written at index -1 when a side holds team -1 (as the original).
# Comparers: both side by side over the display list (compare_match_screen_build); the league tab only the original
#   for the error boxes and when a small league's fixture list is still to be built (operator new kept in param 5),
#   with the stage's TABLE_COLUMN as an extra region; the cup tab only the original for the error boxes (its bye mask
#   is calloc'd and freed inside).
TELAS = True
ROTINAS = [('comp_screens_draw_league_table_tab', 0x491380, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('comp_screens_draw_cup_rounds_tab', 0x492980, [0x6a, 0xff, 0x68, 0xfb, 0x61, 0x95, 0x00])]
DOCS = ('a tela da competição (2): a aba Tabela de uma liga e a aba Sorteio de uma copa',
        "the competition screen (2): a league's Table tab and a cup's Draw tab",
        [('comp_screens_draw_league_table_tab', 'comp_screens_draw_league_table_tab', '0x491380', 'comp_screens.cpp', 'cdecl',
          'a aba Tabela de uma fase de liga: antes da data do sorteio, "O sorteio será em ..."; competição inativa, "ainda não foi programada"; senão a lista Ver (Geral, Casa, Fora: o tipo no parâmetro 4), os botões das fases, o título e a tabela (posição, letra da situação C/Q/P/Pl/R, clube, nação nas competições de seleções, jogos, vitórias, empates ou SOW/Pen pelas regras, derrotas, gols pró e contra, pontos), com uma linha divisória antes de cada faixa de acesso, repescagem e rebaixamento; uma liga pequena (até 8 clubes, com turnos) mostra também os jogos de uma semana antes da primeira rodada a três semanas depois da última (a lista no parâmetro 5)',
          "a league stage's Table tab: before the draw date, \"The draw will be made on ...\"; an inactive competition, \"has not been scheduled yet\"; otherwise the View popup (Overall, Home, Away: the type in parameter 4), the stage buttons, the title and the table (position, status letter C/Q/P/Pl/R, club, nation in national team competitions, played, won, drawn or SOW/Pen by the rules, lost, for, against, points), with a divider before each promotion, play-off and relegation band; a small league (up to 8 clubs, in legs) also shows its fixtures from a week before its first round to three weeks after its last (the list in parameter 5)"),
         ('comp_screens_draw_cup_rounds_tab', 'comp_screens_draw_cup_rounds_tab', '0x492980', 'comp_screens.cpp', 'cdecl',
          'a aba Sorteio de uma copa: antes do primeiro sorteio, "O sorteio ainda não foi feito" ou a data dele; depois, a rodada do parâmetro 3 (−2: a última sorteada) com o nome, os botões Sortear o Próximo, Sortear Todos e Sorteio Automático enquanto a rodada está por sortear, e os confrontos: os possíveis mandantes com a divisão ou nação, o placar de cada jogo (pênaltis, W.O., gols fora, prorrogação), os visitantes quando o confronto está decidido e os times de folga quando a rodada está sorteada; parâmetros 0x32/0x33 o último confronto mostrado e " N times a sortear"',
          "a cup's Draw tab: before the first draw, \"The draw has not yet been made\" or its date; afterwards the round of parameter 3 (−2: the last drawn) with its name, the Draw Next Team, Draw All Teams and Automatic Draw buttons while the round is still to be drawn, and the ties: the possible home teams with their division or nation, the score of each leg (penalties, walkover, away goals, extra time), the away teams once the tie is decided and the teams given a bye once the round is drawn; parameters 0x32/0x33 the last tie shown and \" N teams to draw\"")])
