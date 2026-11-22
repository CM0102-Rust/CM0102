# The club screen's Next Match and Fixtures tab answers and its leave (club_screens.cpp, part 3; reader tel03, the
# screens round, 21/11/2026).
# - 0x468c10 club_screen_next_match_tab_input cdecl(short widget) ret 0, SEH, 0x50 bytes of locals (a Date, an LLIST,
#   the date steps' results); 1472 bytes (the map's 451 + the rest of the routine to 0x4691cf). Calls: 0x49b830
#   comp_open_league_progress_for_fixture, 0x46e6b0 open_past_meetings_screen cdecl(club, other, reserves) (tel04),
#   0x68d020 LLIST::LLIST, 0x549ef0 Date::Date thiscall(day, year) ret 8, 0x54c8e0/0x54cab0/0x54c770/0x54c5b0
#   Date::sub_days/sub_weeks/add_weeks/add_days thiscall(out, n) ret 8, 0x5acbe0 FIXTURE_MANAGER::find_fixtures
#   thiscall(10 args) ret 0x24 (Rust), 0x944cf1 operator new.
# - 0x4691d0 club_screen_fixtures_tab_input cdecl(short widget) ret 0, SEH, 0x46c bytes of locals; 3016 bytes (the
#   map's 901 + the rest to 0x469d97). Calls: 0x5c6570/0x5c87b0/0x5c85d0/0x5c8260 Friendly (thiscall on [0xae2c74]),
#   0x4739d0 open_tournament_invitations_screen cdecl(t, 1), 0x54ba20 Date::get_long_date_string thiscall ret 0xc,
#   0x53b640 database_get_club_kit_colours, 0x591840 confirm_push cdecl(7 args), 0x600c90
#   human_manager_current_controls_club, 0x4719a0 open_friendly_tour_nation_screen, 0x4701f0 open_arrange_friendly_screen
#   cdecl(club, kind, data), 0x48f8e0 comp_screens_open_comp_fixtures cdecl(comp, day, year, mode), 0x7fe600
#   open_screen (0x472380, 0x472850, 1, 0, 0).
# - 0x469da0 club_screen_leave cdecl(void) ret 0 (72 bytes; inside the map's 0x4691d0 region).
# Not in this item: 0x4671f0 club_screen_input (the club screen's own handler, 0x4671f0..0x4689e4 plus its jump tables
#   to 0x468b10, 1775 instructions; it holds the map's 0x4671e0, 0x467753 and 0x46835e entries): see
#   tel03_4/PRONTO.txt. And 0x465420..0x467190 is the tail of 0x460590 (tel02).
# Kept from the original: the next match tab does not free the fixture copy it replaces in param 3 (club_screen_leave
#   frees only the last one); the date words passed to find_fixtures have register garbage in their high halves in the
#   original (0 here; the callee compares them as words).
# Comparers: next match: the screen openings run the original only; Previous/Next run the original only and the
#   fixture copy left in param 3 is checked against the copy's pick. Fixtures: switching and leaving the cancel mode
#   side by side; the season page original only with the page and the list checked; the box answers and everything
#   that opens a screen or a box original only. Leave: original only, cleared params checked.
# How to reach them: a club's screen (Squad > the club), Next Match tab (Previous/Next, the competition link, Past
#   Meetings); Fixtures tab (season arrows, Arrange Friendly, Cancel then a friendly/proposed friendly/tour/tournament
#   and the Please Confirm answer, a fixture's competition link, a tour, a tournament); leaving the club screen.
TELAS = True
ROTINAS = [('club_screen_next_match_tab_input', 0x468c10, [0x6a, 0xff, 0x68, 0x80, 0x5b, 0x95, 0x00]),
           ('club_screen_fixtures_tab_input', 0x4691d0, [0x6a, 0xff, 0x68, 0x9b, 0x5b, 0x95, 0x00]),
           ('club_screen_leave', 0x469da0, [0x53, 0x55, 0x56, 0x57, 0x6a, 0x02])]
DOCS = ('as respostas das abas Próximo Jogo e Jogos da tela do clube, e a saída da tela',
        'the club screen\'s Next Match and Fixtures tab answers, and leaving the screen',
        [('club_screen_next_match_tab_input', 'club_screen_next_match_tab_input', '0x468c10', 'club_screens.cpp', 'cdecl, SEH',
          'Responde ao clique na aba Próximo Jogo: ação 0x1d abre o progresso da liga do jogo, 0x1e os confrontos anteriores com o clube do item ou o outro clube do jogo, 0x26/0x27 passam ao jogo anterior/seguinte do clube (de 24 semanas antes ao dia anterior, ou do dia seguinte a 24 semanas depois, 8 para quem não é seleção; o último/primeiro achado): uma cópia nova vai ao parâmetro 3 (−11), ou 0 quando não há.',
          'Answers a click on the Next Match tab: action 0x1d opens the fixture\'s league progress, 0x1e the past meetings with the item\'s club or the fixture\'s other club, 0x26/0x27 move to the club\'s previous/next fixture (from 24 weeks before to the day before, or from the day after to 24 weeks on, 8 for a club that is not a national team; the last/first found): a new copy goes to param 3 (−11), or 0 when there is none.'),
         ('club_screen_fixtures_tab_input', 'club_screen_fixtures_tab_input', '0x4691d0', 'club_screens.cpp', 'cdecl, SEH',
          'Responde ao clique na aba Jogos: antes, a resposta de uma caixa de cancelamento (amistoso, amistoso proposto, excursão, torneio: cancela no sim) ou do convite de torneio; depois a página da temporada (a lista é liberada), e no modo de cancelar um amistoso, amistoso proposto, excursão ou torneio do clube abre a caixa Please Confirm (outro item só sai do modo); fora dele, o botão Cancelar alterna o modo, marcar amistoso (excursão abre a escolha da nação), a competição do jogo, a excursão e os convites do torneio.',
          'Answers a click on the Fixtures tab: first the answer of a cancellation box (friendly, proposed friendly, tour, tournament: cancelled on yes) or of the tournament invitation; then the season page (the list is freed), and in cancel mode a friendly, proposed friendly, tour or the club\'s tournament opens the Please Confirm box (any other item only leaves the mode); outside it, the Cancel button switches the mode, arrange friendly (a tour opens the nation choice), the fixture\'s competition, the tour and the tournament invitations.'),
         ('club_screen_leave', 'club_screen_leave', '0x469da0', 'club_screens.cpp', 'cdecl',
          'Ao fechar a tela do clube, libera os blocos dos parâmetros 2 e 3, a lista do 0xd e o vetor do 0x15 (com o 0x16), zerando cada um.',
          'When the club screen closes, frees the buffers of params 2 and 3, the list of 0xd and the array of 0x15 (with 0x16), clearing each.')])
