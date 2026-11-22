# Past meetings and the friendly dates (club_screens.cpp, part 4a; reader tel04, the screens round, 21/11/2026).
# The tel04 list (0x46e6b0..0x4771a5, "direto"/"vizinhanca" blocks of the coverage) holds 28 routines of the map
# (0x472070 is a fragment of 0x471a90, item tel04_2) plus the first 7.3 KB of squad_numbers_screen_build (0x475520,
# 21 KB, to 0x47a9d0): that one is left to tel05, whose list holds the other 13.6 KB (a routine is hooked whole).
# Conventions checked at the call sites (every push against esp):
# - 0x46e6b0 open_past_meetings_screen cdecl(club, opposition, reserves) ret 0; 0x7fe600 open_screen thiscall(builder,
#   handler, history flag, leave, 0) ret 0x14; 0x7ff270 set_screen_param thiscall(k, v, 0) ret 0xc.
# - 0x46e820 past_meetings_screen_build cdecl(void), SEH, 0x140 of locals; the map's 1914 bytes go on to 0x46ffc1.
#   0x41c250 background_set_club_picture cdecl(club, 0, 0); 0x542100 club_get_national_team_name cdecl(club,
#   reserves, out, 2000, 3, 1); 0x53b640 kit colours cdecl(club, 1, &c1, &c2), title 0x803f30 thiscall(c2, c1, text);
#   0x803f80 subtitle thiscall(text, 1); 0x75db00 menubar_build cdecl(4, 0); 0x804030 add_button_item thiscall 12
#   args ret 0x30; 0x55fba0 add_area thiscall 11 args ret 0x2c; 0x55f990 add_item thiscall 18 args ret 0x48; 0x804100
#   add_area_text thiscall 9 ret 0x24; 0x8040a0 add_area_button thiscall 11 ret 0x2c; LLIST 0x68d020/0x68d040 ctor/dtor
#   thiscall, 0x68d080 clear, 0x68d0d0 add_tail thiscall(item, 0x4f) ret 8 (a copy), 0x68d1b0 get_next; 0x417c10
#   Date::operator= thiscall(this, hidden, src) ret 8; 0x5acbe0 find_fixtures thiscall on [0xae2a58] 9 args ret 0x24;
#   0x4afa80 venue side cdecl(club, fixture); 0x5c6710 is_friendly_competition thiscall on [0xae2c74] ret 4; 0x88c6d0
#   SubLeague ctor thiscall 12 args ret 0x30 (new 0xee); the old table freed by its vtable[0](1); 0x6827d0 sort_table
#   thiscall(0) ret 4, 0x6826d0 get_table_row thiscall(k) ret 4; 0x90c310 ordinal cdecl(n, -1); 0x600a50 is_human_club
#   cdecl(club, 0, 1, 0, 0); 0x549ef0 Date::Date thiscall(day, year) ret 8; 0x54a650 short date thiscall ret 0xc;
#   0x7e6f20 thiscall on [0xb74434] ret 4; 0x5c2990 / 0x5c29b0 thiscall on [0xae2c74] ret 4 / 8; 0x729500 thiscall
#   on [0xb6b7c0] 5 args ret 0x14; 0x4bf930 round label cdecl(fixture, 3, out, 200); 0x403320 Area::set_scroll_row
#   thiscall(row) ret 4; 0x803320 get_area_scroll_row thiscall ret 4; 0x7febb0 / 0x7fec60 / 0x5edcc0 Back/Next.
# - 0x46ffd0 past_meetings_screen_input cdecl(short) ret 0 (jump table 0x4701b8/0x4701c4 inside); 0x48f8e0
#   comp_screens_open_comp_fixtures cdecl(comp, day, year, mode); 0x7615d0 / 0x7643d0 the menu bar cdecl(widget).
# - 0x4701f0 open_arrange_friendly_screen cdecl(club, char kind, nation) ret 0.
# - 0x470400 arrange_friendly_dates_screen_build cdecl(void), SEH, 0x5c of locals: 0x944cf1 new, 0x9463af vector
#   ctor stdcall(p, 8, 10, 0x54a110, 0x5a53c0); 0x5ede20 page buttons cdecl 8 args; 0x54c5b0 Date::add_days
#   thiscall(out, days) ret 8; 0x68d2e0 LLIST::get_current thiscall; 0x483a90 add_club_fixture_row cdecl(club,
#   fixture, area, row, 6); 0x5c75a0 Friendly::get_club_date_status thiscall on [0xae2c74] 9 args ret 0x24 (al);
#   0x54b550 weekday date string thiscall ret 0xc; 0x53b900 is_national_team, 0x53b3d0 get_national_team cdecl.
# - 0x470fe0 arrange_friendly_dates_screen_input cdecl(short) ret 0, SEH: 0x472900 open_configure_tournament_screen
#   cdecl(settings, arg); 0x945333 qsort(dates, n, 8, 0x4877c0); 0x5c74d0 (rest days, ret 4, ecx = [0xae2c74]);
#   0x5c68b0 thiscall ret 0; 0x944c94 delete; 0x5c6f00 add_friendly_match thiscall 4 args ret 0x10; 0x5a0650
#   open_choose_friendly_teams_screen cdecl 4 args; 0x5c71f0 add_friendly_dates_request thiscall 4 args ret 0x10;
#   0x5c6e00 add_tournament thiscall 10 args ret 0x28.
# - 0x4718f0 arrange_friendly_dates_screen_leave cdecl(void): 0x9462bb vector dtor stdcall(p, 8, n, 0x5a53c0).
# The original's inline strcpy is pure Rust (tel26_copy_text). Colour words go in dword slots whose high half is
# register garbage in the original (0 here); the date words of find_fixtures in the past meetings search likewise.
# The 1080 exe changes the screen-wide area of 0x470400 (0x31f/0x257 at 0x470fb3/0x470fae): read from the running exe.
# Defects of the original: none found (the past meetings filter can hold Anywhere and a venue at once only through
# the 0x221 the screen opens with: Anywhere and Ignore Friendlies with All Time).
# Comparers: the openers only the original, with the params checked against the plan; the builds side by side by the
# display list (past meetings: with the cache and its table, original only when the cache is stale; friendly dates:
# with the chosen dates, original only on the first build and when a day of the page has a fixture of the club); the
# inputs side by side for the clicks that only write params (and the cache's filter, the chosen dates); the leave
# only the original, the cleared params checked.
# How to reach it: a club's fixtures or next match > Past Meetings (View and Filter popups, a meeting's date and
# competition); the club's fixtures > Arrange Friendly (home/away), Arrange Tour > a nation > Next (tour dates), Arrange
# Tournament (CM Cup: Configure, then the dates); page buttons, choosing and dropping dates, Next, Cancel.
TELAS = True
ROTINAS = [('open_past_meetings_screen', 0x46e6b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('past_meetings_screen_build', 0x46e820, [0x6a, 0xff, 0x68, 0x0a, 0x5d, 0x95, 0x00]),
           ('past_meetings_screen_input', 0x46ffd0, [0x53, 0x55, 0x56, 0x57, 0x6a, 0x02]),
           ('open_arrange_friendly_screen', 0x4701f0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('arrange_friendly_dates_screen_build', 0x470400, [0x6a, 0xff, 0x68, 0x3e, 0x5d, 0x95, 0x00]),
           ('arrange_friendly_dates_screen_input', 0x470fe0, [0x6a, 0xff, 0x68, 0x5b, 0x5d, 0x95, 0x00]),
           ('arrange_friendly_dates_screen_leave', 0x4718f0, [0x56, 0x6a, 0x02, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('os jogos anteriores e as datas dos amistosos (club_screens.cpp, parte 4)',
        'past meetings and the friendly dates (club_screens.cpp, part 4)',
        [('open_past_meetings_screen', 'open_past_meetings_screen', '0x46e6b0', 'club_screens.cpp', 'cdecl',
          'abre a tela de jogos anteriores do clube contra o adversário (sem um dos dois, a caixa de club_screens.cpp 11728): parâmetros clube, adversário, filtro 0x221 (todos os tempos, qualquer lugar, sem amistosos), o cache do técnico do canal, rolagem e área 0 e a marca de reservas',
          'opens the past meetings screen of the club against the opposition (either missing: the club_screens.cpp 11728 box): params club, opposition, filter 0x221 (all time, anywhere, ignore friendlies), the slot manager\'s cache, scroll and area 0 and the reserves flag'),
         ('past_meetings_screen_build', 'past_meetings_screen_build', '0x46e820', 'club_screens.cpp', 'cdecl, SEH',
          'monta a tela: título com o nome do clube nas cores do uniforme, "Past Meetings With <adversário>", os menus Ver (todos os tempos, últimos 3, 5, 10, 25 anos) e Filtro (qualquer lugar, no estádio de cada clube, campo neutro, ignorar amistosos); refaz a busca quando o cache é de outro dia, clubes, reservas ou filtro (os jogos disputados contra o adversário desde o início do período que passam no filtro, e a tabela do confronto); a tabela (J, V, E, D, GP, GC, P) e uma linha por jogo (data, mandante, placar, visitante, M/V/N, fase, com as ligações), linhas vazias até o fim, a rolagem e os botões voltar e avançar',
          'builds the screen: title with the club\'s name in its kit colours, "Past Meetings With <opposition>", the View (all time, last 3, 5, 10, 25 years) and Filter (anywhere, at each club\'s ground, neutral ground, ignore friendlies) popups; searches again when the cache is from another day, clubs, reserves or filter (the played games against the opposition since the period\'s start that pass the filter, and the head-to-head table); the table (P, W, D, L, F, A, Pts) and a row per meeting (date, home, score, away, H/A/N, round, with their links), empty rows to the end, the scroll and the back and forward buttons'),
         ('past_meetings_screen_input', 'past_meetings_screen_input', '0x46ffd0', 'club_screens.cpp', 'cdecl',
          'responde ao clique: guarda a rolagem; uma escolha dos menus troca os bits do período (0x1f), do lugar (0x1e0) ou dos amistosos (0x600) no filtro e invalida o cache (refaz a tela); a data de um jogo abre os jogos da competição naquele dia; o resto vai para a barra de menus',
          'answers a click: saves the scroll; a popup choice replaces the period (0x1f), venue (0x1e0) or friendlies (0x600) bits of the filter and makes the cache stale (rebuild); a meeting\'s date opens the competition\'s fixtures on that day; the rest goes to the menu bar'),
         ('open_arrange_friendly_screen', 'open_arrange_friendly_screen', '0x4701f0', 'club_screens.cpp', 'cdecl',
          'abre a tela das datas do amistoso: tipo 3 em casa, 4 fora, 0xb excursão (com a nação; sem ela, ou sem clube, a caixa de club_screens.cpp 12257), 10 torneio (novas definições da CM Cup: liga, 3 pontos, saldo, sem prorrogação nem desempate, 4 times); parâmetros clube, página 1, datas e contagem vazias, tipo, linhas e lista 0, 20 páginas, a nação e as definições',
          'opens the friendly dates screen: kind 3 home, 4 away, 0xb tour (with the nation; without it, or without a club, the club_screens.cpp 12257 box), 10 tournament (new CM Cup settings: league, 3 points, goal difference, no extra time nor decider, 4 teams); params club, page 1, empty dates and count, kind, rows and list 0, 20 pages, the nation and the settings'),
         ('arrange_friendly_dates_screen_build', 'arrange_friendly_dates_screen_build', '0x470400', 'club_screens.cpp', 'cdecl, SEH',
          'monta o calendário: cria na primeira vez o vetor das 10 datas escolhidas e a lista de jogos; título pelo tipo, "Select Date(s)", botões de página, Configurar no torneio; uma linha por dia a partir de hoje + (página - 1) x linhas: o jogo do clube naquele dia, ou a situação da data (amistoso marcado, pedido de excursão, torneio, indisponível, escolhida, disponível); Cancelar (Voltar na excursão) e Avançar (só com data escolhida); com o parâmetro 10, a área da tela inteira que abre a configuração',
          'builds the calendar: makes on the first build the vector of the 10 chosen dates and the fixture list; title by the kind, "Select Date(s)", page buttons, Configure for a tournament; one row per day from today + (page - 1) x rows: the club\'s fixture that day, or the date\'s status (friendly arranged, tour request, tournament, unavailable, chosen, available); Cancel (Back for a tour) and Next (only with a chosen date); with param 10, the screen-wide area that opens the configuration'),
         ('arrange_friendly_dates_screen_input', 'arrange_friendly_dates_screen_input', '0x470fe0', 'club_screens.cpp', 'cdecl, SEH',
          'responde ao clique: o primeiro do torneio novo abre a configuração; páginas (-10, -1, +1, +10, entre 1 e o total); configurar; os jogos da competição; tirar uma data (excursão: sai do vetor ordenado; os outros zeram a contagem); escolher uma data (amistoso: uma; excursão: até 10, ordenadas; torneio: os dias seguidos que o tipo pede); cancelar (fecha o convite aberto ou libera as definições); avançar (marca o amistoso e abre a escolha do adversário, pede as datas à nação ou cria o torneio e enfileira o comando 0x11; falha: caixa de erro); o resto vai para a barra de menus',
          'answers a click: the first one of a new tournament opens the configuration; pages (-10, -1, +1, +10, between 1 and the count); configure; the competition\'s fixtures; drop a date (tour: out of the sorted vector; the others clear the count); choose a date (friendly: one; tour: up to 10, sorted; tournament: the consecutive days its kind needs); cancel (closes the open invitation or frees the settings); next (arranges the friendly and opens the opponent choice, asks the nation for dates or adds the tournament and queues command 0x11; failure: error box); the rest goes to the menu bar'),
         ('arrange_friendly_dates_screen_leave', 'arrange_friendly_dates_screen_leave', '0x4718f0', 'club_screens.cpp', 'cdecl',
          'ao sair, destrói e libera o vetor das datas escolhidas (zera os parâmetros 2 e 3) e a lista de jogos (zera o 6)',
          'on leaving, destroys and frees the chosen dates vector (clears params 2 and 3) and the fixture list (clears 6)')])
