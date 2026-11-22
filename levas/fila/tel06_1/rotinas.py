# The club screens (6a): club_screens.cpp, part 6 (reader tel06, the screens round, 21/11/2026).
# Conventions checked in the disassembly (every push against esp):
# - 0x483920 club_record_type_to_category cdecl(char type, int* out) ret 0 -> al (0x200 bytes of locals for the error
#   box); jump table 0x483a50 (6 targets) with the byte index 0x483a68 (32 entries), to 0x483a90.
# - 0x4841d0 build_filtered_sorted_squad_list cdecl(14 args) ret 0 -> 1 (0 after the error box); calls 0x540a50
#   club_get_linked_b_team cdecl(club, out, 1), 0x68d080 LLIST::clear, 0x90d4f0 VIRTUAL_STAFF::fill_viewed_club_squad
#   thiscall on [0xde2f70] ret 4, 0x53d980/0x53c190 (national squad foreigner), 0x600c90, 0x5b7d20 (position mask,
#   ax), 0x8972c0 Tactics::find_selection_slot thiscall(club, player, reserves) ret 0xc -> al, 0x4cc130 thiscall
#   (player, club) ret 8 -> al, 0x45cbe0 cdecl(player, fixture, club, 0xae2c90), 0x4cf5a0 thiscall ret 4, 0x775790
#   thiscall ret 4, 0x945333 qsort cdecl(base, n, 4, cmp), 0x68d0d0 LLIST::add_tail thiscall(item, 4) ret 8.
#   The comparators 0x485470..0x487470 stay original (passed by address).
# - 0x484980 / 0x4849c0 cdecl(club, list[, top_n]) ret 0: forwarders to 0x4841d0 (14 dwords, add esp 0x38).
# - 0x4849f0 get_player_status_code cdecl(player, club, fixture) ret 0 -> al, SEH (the local LLIST, 0x68d020 /
#   0x68d040); 0x842180 Shortlist_Manager::collect_interested_clubs thiscall on [[0xde1f54]+0x856](player, list, 1, 2,
#   0) ret 0x14; 0x8cf6b0 thiscall(fixture, out, 1) ret 0xc; 0x7757d0 thiscall(player, 0xae2c90, club, 0xde1f64, 0)
#   ret 0x14; 0x55d270 thiscall(player, comp, 1, 0) ret 0x10; 0x55d360 thiscall(player, comp) ret 8; 0x631550,
#   0x85fa20 (al), 0x7a7eb0 (ecx set, ret 4, al), 0x8676f0 thiscall.
# - 0x483a90 add_club_fixture_row cdecl(club, fixture, area, row, date_link_mode) ret 0 -> eax (0/1), SEH (two Dates):
#   0x8040a0 add_area_button thiscall(11 dwords) ret 0x2c, 0x804100 add_area_text thiscall(9 dwords) ret 0x24, 0x54b550
#   thiscall(date, out, 0x7d0, 0xff) ret 0xc, 0x729500 thiscall(slot, home id, away id, day, year) ret 0x14, 0x5c29b0
#   thiscall(comp, date*) ret 8, 0x5c2990/0x7e6f20/0x5c6710 thiscall ret 4.
# Identical in cm0102-1080.exe (the whole routines).
# Depends on: tel01_1 (is_player_available_for_match), tel01_2 (club_screens_error_box), tel26_1 (tel26_area_button,
#   tel26_area_text, tel26_copy_text, job_colour, JOB_TEXT and the JOB_COLOUR_* words).
# Kept from the original (simplifications noted): the colour words are pushed zero-extended (the original pushes
#   registers with only the low half loaded); the out flag of the second club_get_linked_b_team call starts at 0.
# Comparers: the record type and the status code by value side by side (the status also over the comment buffer
#   0xde1f64); the three squad list routines side by side over the sort context (0xad9db0, 0xadab33.., 0xadadf0), the
#   return and the list's items; the fixture row over the display list (compare_match_screen_build) and the return.
#   The error boxes (no club, no list, no player, a type out of range): original only.
TELAS = True
ROTINAS = [('club_record_type_to_category', 0x483920, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
           ('build_filtered_sorted_squad_list', 0x4841d0, [0x81, 0xec, 0xdc, 0x02, 0x00, 0x00]),
           ('build_squad_list_for_staff_history', 0x484980, [0x8b, 0x44, 0x24, 0x0c, 0x8b, 0x4c, 0x24, 0x08]),
           ('build_squad_list_by_name_for_printout', 0x4849c0, [0x8b, 0x44, 0x24, 0x08, 0x6a, 0x00]),
           ('get_player_status_code', 0x4849f0, [0x6a, 0xff, 0x68, 0x7b, 0x5f, 0x95, 0x00]),
           ('add_club_fixture_row', 0x483a90, [0x6a, 0xff, 0x68, 0x61, 0x5f, 0x95, 0x00])]
DOCS = ('as telas do clube (6a): a aba do histórico de um recorde, as listas do elenco e o código de situação de um jogador, uma linha dos jogos do clube',
        'the club screens (6a): a club record\'s history tab, the squad lists and a player\'s status code, a club fixture row',
        [('club_record_type_to_category', 'club_record_type_to_category', '0x483920', 'club_screens.cpp', 'cdecl',
          'o tipo de recorde do clube (1 a 32) dá a aba da tela de história e a entrada do menu: 1-2 aba 4 (7), 3-6 aba 5 (10), 7-11 aba 6 (13), 12-15 aba 7 (16), 16-27 e 32 aba 8 (19), 28-31 aba 9 (22); outro tipo mostra a caixa de erro e devolve a aba 4',
          'the club record type (1 to 32) gives the history screen tab and the menu entry: 1-2 tab 4 (7), 3-6 tab 5 (10), 7-11 tab 6 (13), 12-15 tab 7 (16), 16-27 and 32 tab 8 (19), 28-31 tab 9 (22); any other type shows the error box and returns tab 4'),
         ('build_filtered_sorted_squad_list', 'build_filtered_sorted_squad_list', '0x4841d0', 'club_screens.cpp', 'cdecl',
          'a lista do elenco de um clube: limpa a lista, completa os jogadores virtuais e fica com os jogadores que passam nos filtros (posições, escalação, disponibilidade para o jogo, idade nas copas continentais, regens escondidos, só os do clube com contrato); a seleção tira os estrangeiros do elenco; ordena pela coluna escolhida (a primeira marca de ordem), opcionalmente só os melhores, e põe na lista (de trás para a frente com a marca 0x1000000)',
          'a club\'s squad list: clears the list, tops up the virtual players and keeps the players that pass the filters (positions, team sheet, availability for the match, age in the continental cups, hidden regens, only those of the club with a contract); a national team removes foreigners from its squad; sorts by the chosen column (the first sort flag), optionally only the best, and adds them to the list (backwards with flag 0x1000000)'),
         ('build_squad_list_for_staff_history', 'build_squad_list_for_staff_history', '0x484980', 'club_screens.cpp', 'cdecl',
          'a lista do elenco das telas da comissão e da história: o time principal pela ordem 0x80000, estilo de nome 4, filtro 0x164c21, opcionalmente os melhores',
          'the squad list of the staff and history screens: the first team by sort 0x80000, name style 4, filter 0x164c21, optionally the best'),
         ('build_squad_list_by_name_for_printout', 'build_squad_list_by_name_for_printout', '0x4849c0', 'club_screens.cpp', 'cdecl',
          'a lista do elenco da impressão das estatísticas: o time principal pelo nome, estilo de nome 3, filtro 0x164c21',
          'the squad list of the squad stats printout: the first team by name, name style 3, filter 0x164c21'),
         ('get_player_status_code', 'get_player_status_code', '0x4849f0', 'club_screens.cpp', 'cdecl, SEH',
          'o código de situação que a lista do elenco mostra ao lado do jogador, o primeiro que valer: ausente sem permissão ou de férias, precisa de visto, inelegível, convocado, suspenso, machucado, acima do limite de estrangeiros, lesão leve, acima do limite de não comunitários, sem condição, observado, a um cartão da suspensão, transferência futura, sob proposta, clubes interessados, listado a pedido, na lista de transferência ou de empréstimo, contrato renovável, aposentando, saindo de graça, insatisfeito, jogador do time principal; sem clube, uma ordem menor',
          'the status code the squad list shows next to the player, the first that applies: away without leave or on holiday, needs a work permit, ineligible, on international duty, banned, injured, over the foreigners limit, lightly injured, over the non-EU limit, unfit, scouted, one booking from a ban, future transfer, under offer, clubs interested, listed by request, transfer or loan listed, rolling contract, retiring, leaving on a free, unhappy, first-team player; without a club, a shorter order'),
         ('add_club_fixture_row', 'add_club_fixture_row', '0x483a90', 'club_screens.cpp', 'cdecl, SEH',
          'uma linha dos jogos do clube: a data (link para os jogos da competição quando pedido e atual), o resultado do lado do clube (P:P adiado, A:A abandonado; link para os detalhes enquanto guardados, "view" para o jogo que o humano assiste, senão link para o jogo), o adversário (amarelo se humano), a nação dele se estrangeiro, o local (H, A ou N) e a rodada (link quando atual)',
          'a row of the club\'s fixtures: the date (a link to the competition\'s fixtures when asked and current), the result from the club\'s side (P:P postponed, A:A abandoned; a link to the details while kept, "view" for the match the human watches, otherwise a link to the fixture), the opponent (yellow when human), its nation when foreign, the venue (H, A or N) and the round (a link when current)')])
