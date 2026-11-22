# The Choose Friendly Teams screen (find_screens.cpp, part 1; reader tel28, the screens round, 21/11/2026): opening
# it, building it (the title, the tab bar and the Nations tab inline, the named piece find_nations_tab), its five
# other tabs and its input. Common helpers of tel28 (tel28_lang, tel28_crt, tel28_text_item, tel28_click_compare):
# tel28_2 uses them, so this item goes first, and if it goes out, tel28_2 goes too. Depends on tel26_1 (tel26_*,
# JOB_TEXT, job_colour).
# Conventions checked in the disassembly (every push against esp):
# - 0x5a0650 open_choose_friendly_teams_screen cdecl(tab byte, continent, nation, mode byte) ret 0 (as tel19_2
#   reads it); 0x7fe600 open_screen thiscall(build 0x5a0840, input 0x5a2780, 0, 0, 0) ret 0x14; 0x7ff270
#   set_screen_param thiscall(k, v, 0) ret 0xc.
# - 0x5a0840 draw_choose_friendly_teams_screen cdecl(void), SEH, 0x1740 bytes of locals (__alloca_probe): 25 MenuItems
#   (0xd4) by the CRT vector iterators (MenuItem::MenuItem 0x4568f0, destructor 0x5a53c0 empty); jump table 0x5a100c
#   (6 tabs) after the ret. Calls: 0x839570 get_game_title cdecl; 0x839590 get_heading_colour cdecl(k), 1 then 0;
#   0x803f30 add_screen_title thiscall(c0, c1, text) ret 0xc; 0x75db00 menubar_build cdecl(4, 0); 0x5ed780
#   gui_add_tab_bar cdecl(items, n, tab, &top, &bottom, 1); 0x41c100 nation picture cdecl(0, 0, 0); 0x41c250
#   background_set_club_picture cdecl(0, 0, 0); 0x41bfb0 background_set_random_picture; the tabs 0x5a1030..0x5a2250
#   cdecl(top, bottom); 0x804030 add_button_item thiscall(12) ret 0x30; 0x55fba0 Display::add_area thiscall(11) ret
#   0x2c on the current human's Display; 0x765010 / 0x7651e0 the continent / nation popup fillers cdecl(area, code,
#   record); 0x6177f0 get_continent_adjective cdecl(continent, buf, 0x64, form); 0x6181d0
#   nation_get_nationality_adjective cdecl(nation, buf, 0x64, gender); 0x66f4d0 langlib_get_language; 0x803fd0
#   add_text_item thiscall(11) ret 0x2c; 0x944e46 malloc; 0x53b900 database_is_national_team cdecl(club); 0x945333
#   qsort cdecl(list, n, 4, cmp: 0x53ffe0 reputation, 0x540080 name); 0x5e5cd0 font_get_height cdecl(3); 0x4567f0
#   ScreenManager::add_area thiscall(11) ret 0x2c and 0x4567a0 set_area_scroll_row thiscall(area, row) ret 8 (tel01_1,
#   by address); 0x600a50 human_manager_is_human_club cdecl(club, 0, 1, 0, 0); 0x944c9f sprintf; 0x8040a0
#   add_area_button thiscall(11) ret 0x2c; 0x804100 add_area_text thiscall(9) ret 0x24; 0x53b3d0
#   database_get_national_team cdecl(nation); 0x403320 Area::set_scroll_row thiscall(row) ret 4; 0x9452ca free;
#   0x945640 _ftol (min(n x 0.25, 100.0)); 0x7febb0/0x7fec60/0x5edcc0 Back/Next.
# - 0x5a2780 choose_friendly_teams_screen_input cdecl(widget) -> eax; jump table 0x5a2b0c (6 tabs); 0x803320
#   get_area_scroll_row thiscall(area) ret 4; 0x468b10 Display::get_widget_id thiscall(w) ret 4; 0x468b40
#   get_widget_action_id thiscall(w) ret 4; 0x468b90 get_widget_action_data thiscall(w) ret 4; 0x5a2b30
#   choose_friendly_teams_other_clubs_input cdecl(w) (tel34, by address); 0x7615d0 / 0x7643d0 menu bar cdecl(w).
# Kept from the original: the list areas' row count goes as the original's dword: the byte over the rest of the stack
#   slot it is stored into (the bottom in the Nations tab, the last scanned club pointer in Under-21s, the scan's
#   last index in the others); the colour words go zero-extended (the original's high halves are register garbage);
#   the League adjective's gender is (language 4) + 2 as a byte (the original adds 2 to a register whose upper bytes
#   a callee left).
# Defects of the original: a missing continent shows the box and goes on (the tab then reads through a null
#   continent); kept.
# Comparers: opening only the original, the 20 params checked against the plan; the build and the five tabs side by
#   side by the display list (compare_match_screen_build), the original only when the open tab's continent is missing
#   (the box); the input side by side for the picks (a continent, a nation, a tab), the original only for the menu
#   bar, the object links and tab 6 (its own input, 0x5a2b30, not in this group).
# Identical in cm0102-1080.exe (the eight routines whole).
TELAS = True
ROTINAS = [('open_choose_friendly_teams_screen', 0x5a0650, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('draw_choose_friendly_teams_screen', 0x5a0840, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('draw_choose_friendly_under21s_tab', 0x5a1030, [0x81, 0xec, 0x7c, 0x02, 0x00, 0x00]),
           ('draw_choose_friendly_major_clubs_tab', 0x5a1480, [0x81, 0xec, 0x80, 0x02, 0x00, 0x00]),
           ('draw_choose_friendly_league_clubs_tab', 0x5a1990, [0x81, 0xec, 0x80, 0x02, 0x00, 0x00]),
           ('draw_choose_friendly_non_league_tab', 0x5a1e10, [0x81, 0xec, 0x80, 0x02, 0x00, 0x00]),
           ('draw_choose_friendly_other_clubs_tab', 0x5a2250, [0x81, 0xec, 0x80, 0x02, 0x00, 0x00]),
           ('choose_friendly_teams_screen_input', 0x5a2780, [0x56, 0x57, 0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('a tela Escolher Amistosos (a tela Procurar): abrir, montar com as seis abas e a entrada',
        'the Choose Friendly Teams screen (the Find screen): opening it, building it with its six tabs and its input',
        [('open_choose_friendly_teams_screen', 'open_choose_friendly_teams_screen', '0x5a0650', 'find_screens.cpp', 'cdecl',
          'abre a tela Escolher Amistosos (também a tela Procurar) numa aba: 20 parâmetros, 0 a aba, 1, 4, 7 e 0x10 o continente, 0xa e 0xd a nação, 0x13 o modo (0 Procurar, 1 clubes, 2 só seleções), o resto 0; sem continente, a caixa de find_screens.cpp 0x7a (e abre assim mesmo)',
          'opens the Choose Friendly Teams screen (also the Find screen) on a tab: 20 parameters, 0 the tab, 1, 4, 7 and 0x10 the continent, 0xa and 0xd the nation, 0x13 the mode (0 Find, 1 clubs, 2 national teams only), the rest 0; with no continent, the find_screens.cpp 0x7a box (and it opens anyway)'),
         ('draw_choose_friendly_teams_screen', 'draw_choose_friendly_teams_screen', '0x5a0840', 'find_screens.cpp', 'cdecl',
          'monta a tela: o título (Escolher Amistosos com modo, senão o título do jogo), a barra de abas (Seleções e Sub-21 fora do modo 1, Grandes Clubes, Liga, Fora da Liga, Outros; no modo 2 nenhuma, só Seleções) e a aba; a aba Seleções lista as seleções do continente (das primeiras vagas da tabela de clubes) por nome, duas por linha com o continente',
          'builds the screen: the title (Choose Friendly Teams with a mode, else the game title), the tab bar (Nations and Under-21s outside mode 1, Major Clubs, League, Non-League, Other; in mode 2 none, Nations only) and the tab; the Nations tab lists the continent\'s national teams (from the first club slots) by name, two per row with their continent'),
         ('draw_choose_friendly_under21s_tab', 'draw_choose_friendly_under21s_tab', '0x5a1030', 'find_screens.cpp', 'cdecl',
          'a aba Sub-21: as seleções do continente entre as últimas vagas da tabela de clubes (as sub-21), por nome, duas por linha com o continente; a rolagem do parâmetro 5, a área no 6',
          'the Under-21s tab: the continent\'s national teams among the last club slots (the under-21 sides), by name, two per row with their continent; the scroll of parameter 5, the area to 6'),
         ('draw_choose_friendly_major_clubs_tab', 'draw_choose_friendly_major_clubs_tab', '0x5a1480', 'find_screens.cpp', 'cdecl',
          'a aba Grandes Clubes: os clubes do continente (não seleções) por reputação, o melhor quarto (no máximo 100) por nome, duas por linha com a nação e a divisão',
          'the Major Clubs tab: the continent\'s clubs (not national teams) by reputation, the best quarter (at most 100) by name, two per row with their nation and division'),
         ('draw_choose_friendly_league_clubs_tab', 'draw_choose_friendly_league_clubs_tab', '0x5a1990', 'find_screens.cpp', 'cdecl',
          'a aba Liga: os clubes da nação numa divisão que o jogo roda, por nome, com a nação e a divisão',
          'the League tab: the nation\'s clubs in a division the game runs, by name, with their nation and division'),
         ('draw_choose_friendly_non_league_tab', 'draw_choose_friendly_non_league_tab', '0x5a1e10', 'find_screens.cpp', 'cdecl',
          'a aba Fora da Liga: os clubes da nação sem divisão rodada, os 250 melhores por reputação, por nome, com a nação',
          'the Non-League tab: the nation\'s clubs with no running division, the best 250 by reputation, by name, with their nation'),
         ('draw_choose_friendly_other_clubs_tab', 'draw_choose_friendly_other_clubs_tab', '0x5a2250', 'find_screens.cpp', 'cdecl',
          'a aba Outros: os clubes do continente de nações sem ligas escolhidas, pulado o melhor quarto, os 100 seguintes no máximo, por nome',
          'the Other tab: the continent\'s clubs from nations whose leagues are not selected, past the best quarter, the next 100 at most, by name'),
         ('choose_friendly_teams_screen_input', 'choose_friendly_teams_screen_input', '0x5a2780', 'find_screens.cpp', 'cdecl',
          'um clique guarda a rolagem da lista da aba e escolhe o continente ou a nação da aba (ou outra aba): guardado o novo valor, devolve −11 (remontar); senão a barra de menus e os vínculos; a aba Outros pergunta antes à sua própria entrada',
          'a click keeps the tab list\'s scroll and picks the tab\'s continent or nation (or another tab): with a new value stored it returns −11 (rebuild); otherwise the menu bar and the links; the Other tab asks its own input first')])
