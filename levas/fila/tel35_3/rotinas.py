# The retire and holiday screens and the two list comparers of manager_screens.cpp (reader tel35, the screens round,
# 21/11/2026). Uses the helpers of tel26_1 (tel26_add_area, tel26_area_button, tel26_area_text, tel26_text_area,
# tel26_item_id, tel26_item_value, tel26_slot_manager, JOB_TEXT), tel26_4 (manager_menu_bar), tel26_5
# (MANAGER_STATS_SORT_NATION, MANAGER_STATS_SORT_STAT, MANAGER_STATS_UNKNOWN_BELOW), tel35_1 (tel35_colour,
# tel35_text_draw_wrapped) and tel35_2 (tel35_text_item).
# Conventions checked in the disassembly: the opens cdecl(void) (open_screen thiscall(SM, builder, handler, 1, 0, 0));
# the builders cdecl(void) (holiday_screen_build with an SEH frame and a local Text); the handlers cdecl(short) ->
# eax; the comparers cdecl(a, b) -> eax (qsort). background_set_club_picture 0x41c250 cdecl(team, 0, 0);
# background_set_random_picture 0x41bfb0; get_heading_colour 0x839590 cdecl(k); database_get_club_kit_colours
# cdecl(team, 1, &fore, &back); database_format_staff_name cdecl(person, out, size, style); gui_add_text_area 0x5ed070
# cdecl (11); add_text_item 0x803fd0 thiscall(SM, 11) ret 0x2c; Text::add_link thiscall(text, phrase, 3, k) ret 0xc;
# Text::draw_wrapped 0x4aeaf0 thiscall(text, area, 0x96, 0x2f8, &line, 3, colour, 2, 0) ret 0x20; Text::clear 0x4afa40
# thiscall; HUMAN_MANAGER ([0xb63c98]) get_holiday_option 0x5fea50 (option, 0) ret 8 -> al, set_holiday_option
# 0x5feb60 (option, value, 0) ret 0xc, set_on_holiday 0x5fd2f0 / set_controls_reserve_team 0x5fd590 /
# set_computer_controlled 0x5fe140 (0, person) ret 8; MANAGER_MANAGER remove_club_manager 0x69b790 (team, 3, 0, 0)
# and add_job_vacancy 0x69ba50 (team, person); MATCH_DAY::refresh_human_interest_flags 0x6b5040;
# TRANSFER_MANAGER::cancel_club_offers 0x8c4950 (club); News::clear_inbox 0x787630; CONTRACT_MANAGER
# remove_pending_offers_for_staff 0x4ccbf0, remove_contract 0x4d0b50, remove_loan_contract 0x4d0bd0;
# NationalTeams::remove_player 0x775ba0; Staff_Records rewind_staff_history 0x86fb80 and clear_human_manager_record
# 0x86fc70; SCOUT_MANAGER::clear_manager_lists 0x7fc310 (all thiscall(person) ret 4); Date::set 0x54a130
# thiscall(date, 1, 0, 1966, -1) ret 0x10; ScreenManager::remove_manager_record 0x7ff990 thiscall(SM, person, -1)
# ret 8; Date::days_since 0x54cdb0 thiscall(this, date by value) ret 8; utils_string_compare 0x90c1d0 cdecl(a, b, 1,
# 0). The holiday handler's jump table (0x6b36c8, 7 entries) read.
# Equal in cm0102-1080.exe.
# Defects of the original: compare_manager_stats_rows returns the address of a's staff record for a statistic
# outside 1..16 (never chosen by the menu); the holiday "Yes" writes options 2 and 3 back with their own values (no
# effect); both kept.
# Comparers: the builders side by side over the display list; the comparers by value (compare_order); the opens and
# the handlers (they retire the manager, change the holiday settings, run the menu bar) only in the original.
MOTOR = True
TELAS = True
ROTINAS = [('open_retire_screen', 0x6b28b0, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('retire_screen_build', 0x6b28d0, [0x83, 0xec, 0x0c, 0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
           ('retire_screen_handle_action', 0x6b2b40, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('open_holiday_screen', 0x6b2d60, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('holiday_screen_build', 0x6b2d80, [0x6a, 0xff, 0x68, 0x88, 0xd1, 0x95, 0x00]),
           ('holiday_screen_handle_action', 0x6b3440, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('compare_manager_stats_rows', 0x6b36f0, [0x81, 0xec, 0x98, 0x01, 0x00, 0x00]),
           ('compare_job_information_rows', 0x6b3be0, [0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9])]
DOCS = ('as telas de aposentar e de férias do técnico e a ordem das listas de estatísticas e de vagas',
        'the manager\'s retire and holiday screens and the order of the stats and job lists',
        [('open_retire_screen', 'open_retire_screen', '0x6b28b0', 'manager_screens.cpp', 'cdecl',
          'abre a confirmação de aposentadoria do técnico humano atual',
          'opens the current human manager\'s retirement confirmation'),
         ('retire_screen_build', 'retire_screen_build', '0x6b28d0', 'manager_screens.cpp', 'cdecl',
          'monta a confirmação: a foto e as cores do clube (ou da seleção; sem cargo, uma foto ao acaso e as cores de título), o nome do técnico, a pergunta de deixar o clube e a seleção (os dois, um ou nenhum) e se aposentar, e os botões Não e Sim',
          'builds the confirmation: the club\'s (or national team\'s; with no job, a random picture and the heading colours) picture and colours, the manager\'s name, the question of leaving the club and the national team (both, one or none) and retiring, and the No and Yes buttons'),
         ('retire_screen_handle_action', 'retire_screen_handle_action', '0x6b2b40', 'manager_screens.cpp', 'cdecl',
          'Não fecha; Sim tira o técnico do clube (abrindo a vaga e cancelando as ofertas do clube) e da seleção, limpa a caixa de entrada, as férias, o controle do time reserva e pelo computador, as propostas, o contrato e o empréstimo, as convocações, o histórico e o registro de técnico humano e as listas de observação, apaga os nomes, zera os cargos e a data de nascimento (1º de 1966) e tira o registro da tela',
          'No closes; Yes takes the manager out of his club (opening the vacancy and cancelling the club\'s offers) and national team, clears his inbox, holiday, reserve and computer control, offers, contract and loan, squad calls, history and human manager record and scouting lists, blanks his names, clears his jobs and birth date (day 1 of 1966) and removes his screen record'),
         ('open_holiday_screen', 'open_holiday_screen', '0x6b2d60', 'manager_screens.cpp', 'cdecl',
          'abre a confirmação de férias do técnico humano atual',
          'opens the current human manager\'s holiday confirmation'),
         ('holiday_screen_build', 'holiday_screen_build', '0x6b2d80', 'manager_screens.cpp', 'cdecl',
          'monta a confirmação de férias: a foto do clube (ou seleção, ou ao acaso), o nome do técnico nas cores de título, "Go on holiday ?", as sete opções das férias (as ligadas em cinza claro, as desligadas em cinza) e Não e Sim',
          'builds the holiday confirmation: the club\'s (or national team\'s, or a random) picture, the manager\'s name in the heading colours, "Go on holiday ?", the seven holiday settings (the ones on in pale grey, the ones off in grey) and No and Yes'),
         ('holiday_screen_handle_action', 'holiday_screen_handle_action', '0x6b3440', 'manager_screens.cpp', 'cdecl',
          'Não e Sim saem deixando a resposta pendente (comando 8 com 1 ou 2); uma opção liga ou desliga (as ofertas e as compras alternam entre os seus dois valores e desligado)',
          'No and Yes leave with the answer pending (command 8 with 1 or 2); a setting switches on or off (the offers and the purchases switch between their two values and off)'),
         ('compare_manager_stats_rows', 'compare_manager_stats_rows', '0x6b36f0', 'manager_screens.cpp', 'cdecl',
          'a ordem da lista de estatísticas de técnicos pela estatística escolhida, o maior primeiro (reputação, dinheiro, dias no cargo, títulos, jogos, vitórias, valores...), e no empate pelo nome',
          'the order of the manager stats list by the chosen statistic, the highest first (reputation, money, days in the job, honours, games, wins, amounts...), and by name on a tie'),
         ('compare_job_information_rows', 'compare_job_information_rows', '0x6b3be0', 'manager_screens.cpp', 'cdecl',
          'a ordem da lista de informações de vagas pela palavra +8 das linhas',
          'the order of the job information list by the rows\' word +8')])
