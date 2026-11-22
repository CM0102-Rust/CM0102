# The staff profile's History tab (staff_screens.cpp, part 3; reader tel09, the screens round, 21/11/2026).
# - 0x885820 staff_screens_draw_history_tab cdecl(int top, int bottom), ret 0, SEH (a Text), 0x2f4 bytes of locals;
#   the routine runs to the ret at 0x888258 (10809 bytes): the community map's 0x887fa0 (135) and the first 0x239
#   bytes of 0x888027 are its tail (Manager Stats). Called by the staff profile builder 0x870700 (tel07_3) for tab 5.
# Conventions checked at the call sites: 0x55fba0 Display::add_area thiscall ret 0x2c; 0x55f990 Display::add_item
#   ret 0x48; 0x804100/0x8040a0 add_area_text/add_area_button ret 0x24/0x2c; 0x803fd0 add_text_item ret 0x2c;
#   0x5ed070 gui_add_text_area cdecl; 0x5ede20 gui_add_page_buttons cdecl(8); 0x86f5b0 get_manager_record
#   thiscall(p, 0/1) ret 8; 0x86de70 Staff_Records history list thiscall(p, kind) ret 8; 0x86e0b0 record text
#   thiscall(node, Text*) ret 8; 0x4aeaf0 Text::draw_wrapped ret 0x20; 0x54a650 get_short_date_string ret 0xc;
#   0x68d1d0 LLIST::get_prev (the walk), 0x68d040 ~LLIST, 0x944c94 delete; 0x7c1b20 / 0x7c2750 / 0x7c3060 PlayerStats
#   title / season / total texts thiscall on [0xb74350] ret 0xc / 0x1c / 0x1c; 0x669630 season text cdecl(4);
#   0x43f460 + 0x441c10 the money (modes 5 and 6); 0x62ec20 injury history thiscall(p) ret 4, 0x62ef30 injury name
#   thiscall(type, out, 200, 0) ret 0x10; 0x7e5780 days text cdecl(4); 0x55cbc0 ban history thiscall(id) ret 4,
#   0x55cc20 / 0x55d0a0 ban texts thiscall(ban, out, n) ret 0xc; 0x90c2d0 char_to_upper cdecl (its argument stays on
#   the stack as the "%s" of the sprintf after it); 0x403320 Area::set_scroll_row thiscall ret 4; 0x7ff270
#   set_screen_param ret 0xc.
# Uses the helpers of tel26_1 (tel26_add_area, tel26_add_item, display_area_at, job_colour, JOB_TEXT, JOB_COLOUR_*)
#   and of tel08_1 (staff_tab_cell, staff_tab_button, staff_tab_lang, staff_tab_format, staff_tab_strcpy,
#   staff_tab_no_information, staff_tab_draw_wrapped): it goes after both.
# Comparer: side by side by the display list (compare_match_screen_build, which also holds the screen parameters the
#   tab writes, 0x10 and 10).
# How to reach it: a person's profile (click a name) > the History tab; the View button: Achievements (and its
#   Filter: All Records, Competitions, Awards, Transfers and Jobs for a manager; more than 100 records give the page
#   buttons), Playing Career (the << and >> statistics views, a season button, a loan season), Injuries, Bans,
#   Manager Stats (a manager); a chairman has no history.
TELAS = True
ROTINAS = [('staff_screens_draw_history_tab', 0x885820, [0x6a, 0xff, 0x68, 0x7c, 0x2f, 0x96, 0x00])]
DOCS = ('a aba Histórico do perfil da pessoa',
        "the staff profile's History tab",
        [('staff_screens_draw_history_tab', 'staff_screens_draw_history_tab', '0x885820', 'staff_screens.cpp', 'cdecl, SEH',
          'Desenha a aba Histórico do perfil (a pessoa do parâmetro 0) entre as linhas top e bottom: sem histórico (cargo de clube 1 ou 2, ou id a partir da contagem de pessoas) só "There is no history information available."; senão o botão View com o menu (Conquistas, e para jogador Carreira, Lesões e Suspensões, e Estatísticas de técnico quando há registro de técnico; comando 0x14 com o bit do parâmetro 5; a linha em uso vira o título). Em Conquistas, o botão Filter (comando 0x20: todos, competições, prêmios, e para cargo 5 ou 0xc transferências e cargos; o tipo no parâmetro 0xe), os registros desse tipo em páginas (mais de 100: o número de páginas no parâmetro 0x10 e os botões de página) com data, clube e o texto quebrado. Carreira: as temporadas do parâmetro 6 (linhas de 0x102 bytes; o parâmetro 7 é seis vezes o número delas, o 8 a temporada em uso) com clube, valor da transferência (dinheiro, troca, Bosman, livre, empréstimo) e as colunas de estatística do jogador (goleiro ou não, primeira ou segunda vista do parâmetro 0x15), e o total (sem os jogos das temporadas de empréstimo). Lesões: data, tipo, treino ou jogo e o tempo fora. Suspensões: data, competição e motivo. Estatísticas de técnico: ligas, copas, prêmios, pontos, jogos, vitórias, empates, derrotas, gols, compras e vendas. Linhas vazias até o fim; a linha de rolagem da tabela vem do parâmetro 9 e a área vai para o parâmetro 10',
          "Draws the profile's History tab (the person of parameter 0) between rows top and bottom: without history (club job 1 or 2, or an id from the person count up) only \"There is no history information available.\"; else the View button with its menu (Achievements, and for a player Playing Career, Injuries and Bans, and Manager Stats when there is a manager record; command 0x14 with the bit of parameter 5; the line in use becomes the title). With Achievements, the Filter button (command 0x20: all, competitions, awards, and for job 5 or 0xc transfers and jobs; the kind in parameter 0xe), the records of that kind in pages (over 100: the page count in parameter 0x10 and the page buttons) with date, club and the wrapped text. Playing Career: the seasons of parameter 6 (0x102-byte rows; parameter 7 is six times their number, 8 the season in use) with club, fee (money, exchange, Bosman, free, loan) and the player's statistics columns (goalkeeper or not, first or second view of parameter 0x15), and the total (without the loan seasons' games). Injuries: date, type, training or match and the time out. Bans: date, competition and reason. Manager Stats: leagues, cups, awards, points, games, wins, draws, defeats, goals, purchases and sales. Empty rows to the end; the table's scroll row comes from parameter 9 and its area goes to parameter 10")])
