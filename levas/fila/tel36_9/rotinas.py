# The staff profile actions and the compare players dialog (staff_screens.cpp; reader tel36, the screens round,
# 21/11/2026). The action handler 0x889c30 answers the compare players screen (opened by 0x870650, tel07_1) and is
# called by the profile handler 0x888260 for the Action menu; the two tab answers are called by 0x888260 (and the
# profile one by 0x889c30); the leave routine is the profile's and the compare screen's. Uses tel36_1 (tel36_langlib,
# TEL36_CLUB_COMP_SIZE), tel26_1 (tel26_add_area, tel26_area_button, tel26_text_area, tel26_item_id,
# tel26_item_value, tel26_copy_text, job_colour), tel26_4 (manager_menu_bar) and tel35_1 (TEL35_NETWORK_FLAG).
# Conventions checked in the disassembly: the handlers cdecl(short) -> eax (0x889c30 with an SEH frame around two
# local Dates); the leave and the builder cdecl(void); the opener cdecl(void) -> eax (open_screen's answer). The
# actions call by address, cdecl unless noted: staff_screens_open_staff_profile 0x8702a0 (person, 1, 1, 1, 0);
# contract_screens_open_terminate_contract_screen 0x4e53e0; history_screen_open 0x5f11c0; confirm_push 0x591840 (title,
# heading, message, yes/no, colour, colour, code); database_get_comp_colours 0x53b7c0 (comp, &a, &b);
# database_get_national_team 0x53b3d0; AWARD_SHORTLIST::cast_vote 0x417fe0 thiscall(list, human, person) ret 8;
# CONTRACT_MANAGER::toggle_squad_placement 0x4cbd20 thiscall(cm, person, club, &club) ret 0xc;
# Shortlist_Manager::add_player_to_club_shortlists 0x842c00 thiscall(sm, person, 1) ret 8; notes 0x7961b0; treatment
# 0x7e5b10 (person, leave); club_get_linked_b_team 0x540a50 (club, &reserve, 1); database_get_club_assistant 0x53cb80;
# coach_open_report_request_screen 0x48bc00 (person, coach); physio 0x7a2a50 (person, physio);
# staff_normalise_player_attributes 0x538bc0; transfer status 0x8f6770 (person, 0, -1, 0); discipline 0x5a8380;
# TRANSFER_MANAGER::get_agreed_contract_club 0x8c5450 thiscall ret 8; contract_screens_offer_contract_to_staff 0x4dd0f0
# (person, kind, 1|0, 0); transfer list 0x8f8120 (person, 0|1); release 0x8f8630; shortlist 0x8f4160 (person, 0|1);
# offer changed 0x8f2470 (offer, serial); withdraw 0x8f4c70 (slot); bid screens 0x8e9350 (person, slot) and 0x8e9550
# (offer, 1); TRANSFER_MANAGER::can_club_bid_for_player 0x8ce990 thiscall(tm, person, club, &reason, 0) ret 0x10;
# cannot sign 0x8f0e40 (person, club, reason, kind); trial 0x8f8be0; TRANSFER_MANAGER::get_loan_period 0x8d0590
# thiscall(tm, club, seller, person, &a, &b, &c, &d) ret 0x1c; Date::Date 0x549f70 thiscall(date, 1, x, y, -1) ret 0x10;
# loan bid 0x8f9690; end loan 0x8fbcc0; squad move 0x778f60 (person, value, 1|0); HUMAN_MANAGER::has_nickname /
# get_nickname 0x600740 / 0x6005e0 thiscall(hm, person, 0) ret 8; SCOUT_MANAGER::get_scout_request_status 0x7f8ff0 and
# add_to_shortlist 0x7f75f0 thiscall(sm, person, 0) ret 8; the dialogs' builders/handlers 0x88c100/0x88c360 and
# 0x88bc70/0x88bee0. Date::get_month and get_day_of_month take no argument (the values pushed before them are the
# sprintf arguments).
# Kept from the original: the vote box takes the competition's colours from the award entry, the national team's or
# the World Cup's; the loan action builds the dates from the loan period's outputs; the compare dialog formats both
# players' names without using them and keeps the text buffer as it was when both are chosen; with language 0 its
# title is always "Can not compare players"; the nickname buffer leaks when the dialog does not open.
# Equal in cm0102-1080.exe.
# Comparers: the two tab answers side by side over the screen params; the action handler's 0x3c / 0x3d side by side,
# the rest only the original; the dialog builder side by side over the display list; the opener, the leave and the
# dialog handler only the original.
TELAS = True
ROTINAS = [('staff_screens_handle_compare_action', 0x889c30, [0x6a, 0xff, 0x68, 0xbb, 0x2f, 0x96, 0x00]),
           ('staff_screens_handle_profile_tab_action', 0x88b2f0, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('staff_screens_handle_history_tab_action', 0x88b4b0, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('staff_screens_leave_profile_screen', 0x88b8a0, [0x56, 0x57, 0x6a, 0x03, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('staff_screens_open_compare_players_dialog', 0x88c440, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('staff_screens_build_compare_players_dialog', 0x88c460, [0x81, 0xec, 0x9c, 0x01, 0x00, 0x00]),
           ('staff_screens_handle_compare_players_action', 0x88c660, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('as ações do perfil da pessoa e o diálogo de comparar jogadores',
        "the staff profile actions and the compare players dialog",
        [('staff_screens_handle_compare_action', 'staff_screens_handle_compare_action', '0x889c30', 'staff_screens.cpp', 'cdecl',
          'o menu de ações do perfil e da comparação: ver o perfil, histórico, votar num prêmio, mover para o elenco (caixa quando não há vaga), listas de observação, anotação, tratamentos, relatórios do auxiliar e do fisioterapeuta, normalizar atributos, situação de transferência, disciplinar, contrato (caixa quando já aceitou outro), lista de transferência, dispensa, propostas, empréstimo (caixa quando o período não fecha), teste, fim de empréstimo, mover para a seleção, apelido, observação por olheiro (caixas quando não há olheiro, verba ou já observa); no jogo em rede resolve o comando pendente do voto; o resto vai para a aba do perfil e a barra de menus',
          'the action menu of the profile and of the compare screen: view the profile, history, vote for an award, move to the squad (box when there is no room), shortlists, note, treatments, assistant and physio reports, normalise attributes, transfer status, discipline, contract (box when he accepted another), transfer list, release, bids, loan (box when the period does not fit), trial, end the loan, move to the national squad, nickname, scouting (boxes when there is no scout, no money or already scouting); in a network game it settles the pending vote command; the rest goes to the profile tab and the menu bar'),
         ('staff_screens_handle_profile_tab_action', 'staff_screens_handle_profile_tab_action', '0x88b2f0', 'staff_screens.cpp', 'cdecl',
          'na aba do perfil: a vista de atributos (0x12) troca para a do botão; "-" e "+" (0x27, 0x28) mudam o zoom entre 0 e 3; senão 0',
          'on the profile tab: the attribute view (0x12) switches to the widget\'s; "-" and "+" (0x27, 0x28) move the zoom between 0 and 3; otherwise 0'),
         ('staff_screens_handle_history_tab_action', 'staff_screens_handle_history_tab_action', '0x88b4b0', 'staff_screens.cpp', 'cdecl',
          'na aba do histórico: guarda a rolagem (para pessoas comuns), os filtros (0x14) trocam os 5 bits de baixo do modo, a temporada (0x20) volta à página 1, as páginas (0x21, 0x22) andam entre 1 e a última, 0x3e/0x3f ligam o detalhe, 0x15 escolhe a linha; senão 0',
          'on the history tab: keeps the scroll (for regular staff), the filters (0x14) replace the low 5 bits of the mode, the season (0x20) goes back to page 1, the pages (0x21, 0x22) move between 1 and the last, 0x3e/0x3f switch the detail, 0x15 picks the row; otherwise 0'),
         ('staff_screens_leave_profile_screen', 'staff_screens_leave_profile_screen', '0x88b8a0', 'staff_screens.cpp', 'cdecl',
          'ao sair do perfil, destrói a lista do parâmetro 3 e libera o buffer do parâmetro 6',
          'leaving the profile destroys the list of parameter 3 and frees the buffer of parameter 6'),
         ('staff_screens_open_compare_players_dialog', 'staff_screens_open_compare_players_dialog', '0x88c440', 'staff_screens.cpp', 'cdecl',
          'abre o diálogo de comparar jogadores',
          'opens the compare players dialog'),
         ('staff_screens_build_compare_players_dialog', 'staff_screens_build_compare_players_dialog', '0x88c460', 'staff_screens.cpp', 'cdecl',
          'monta o diálogo: o nome do humano nas cores do clube (no idioma 0, "Can not compare players"), o aviso de escolher dois jogadores quando falta um, e o botão Okay',
          'builds the dialog: the human\'s name in the club colours (in language 0, "Can not compare players"), the warning to choose two players when one is missing, and the Okay button'),
         ('staff_screens_handle_compare_players_action', 'staff_screens_handle_compare_players_action', '0x88c660', 'staff_screens.cpp', 'cdecl',
          'Okay fecha; o resto vai para a barra de menus',
          'Okay closes; the rest goes to the menu bar')])
