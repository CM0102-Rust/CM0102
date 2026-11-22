# The manager confirm screens and the board confidence (manager_screens.cpp; reader tel26, the screens round,
# 21/11/2026). Uses the helpers of tel26_1.
# Conventions checked in the disassembly: the opens cdecl(club) (open_job_offer_screen cdecl(manager, news id),
# with an SEH frame and a local NewsItem without destructor); the builders cdecl(void); the handlers cdecl(short)
# -> eax. background_set_club_picture 0x41c250 cdecl(club, 0, 0); menubar_build cdecl(4, 0);
# database_get_club_kit_colours 0x53b640 cdecl(club, 1, &a, &b) and add_screen_title(SM, b, a, text);
# database_format_staff_name 0x53dba0 cdecl(manager, text, 0x7d0, 1); gui_add_text_area 0x5ed070 cdecl (11);
# Display::add_area 0x55fba0 thiscall on the slot's Display (11); add_area_button 0x8040a0 / add_button_item
# 0x804030 thiscall on SM; MANAGER_MANAGER ([0xb67a44]) apply_for_job 0x69ca00 (club, manager), remove_club_manager
# 0x69b790 (club, 2, 0, 0), add_job_vacancy 0x69ba50 (club, manager), appoint_manager_to_club 0x69ebc0 (club,
# manager), build_board_confidence_text 0x690d80 (club, days, out, 0x7d0), build_board_squad_opinion_text 0x693850
# (out, club, 0) -> al; MATCH_DAY::refresh_human_interest_flags 0x6b5040 thiscall([0xb67a50]); News ([0xb7431c])
# find_item_by_id 0x786b70 (manager, id, out) and set_item_slot 0x786d30 (manager, id, 1, value, 0);
# NewsItem::NewsItem 0x77bbe0 (item, 0, 0) and get_slot 0x785830 (item, 0); board_request_screen_open 0x425390 and
# fa_meeting_open_screen 0x58acd0 cdecl(club).
# Defects of the original: the board confidence frame overlaps its two texts (the confidence text is given 2000
# bytes from +8, the squad opinion is written at +0x3f0); kept.
MOTOR = True
TELAS = True
ROTINAS = [('open_apply_for_job_screen', 0x6b19d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('apply_for_job_screen_build', 0x6b1a80, [0x83, 0xec, 0x0c, 0x56, 0x6a, 0x00]),
           ('apply_for_job_screen_handle_action', 0x6b1c30, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('open_resign_job_screen', 0x6b1d10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('resign_job_screen_build', 0x6b1dc0, [0x83, 0xec, 0x0c, 0x56, 0x6a, 0x00]),
           ('resign_job_screen_handle_action', 0x6b1f70, [0x53, 0x56, 0x57, 0x6a, 0x00]),
           ('open_board_confidence_screen', 0x6b2080, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('board_confidence_screen_build', 0x6b2130, [0x81, 0xec, 0xd8, 0x07, 0x00, 0x00]),
           ('board_confidence_screen_handle_action', 0x6b2370, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('open_job_offer_screen', 0x6b2450, [0x6a, 0xff, 0x68, 0x6b, 0xd1, 0x95, 0x00]),
           ('job_offer_screen_build', 0x6b25e0, [0x83, 0xec, 0x0c, 0x56, 0x6a, 0x01]),
           ('job_offer_screen_handle_action', 0x6b2790, [0x53, 0x55, 0x56, 0x57, 0x6a, 0x00])]
DOCS = ('as telas do técnico (2): candidatar-se a uma vaga, demitir-se, a confiança da diretoria e a oferta de emprego',
        'the manager screens (2): applying for a job, resigning, the board confidence and the job offer',
        [('open_apply_for_job_screen', 'open_apply_for_job_screen', '0x6b19d0', 'manager_screens.cpp', 'cdecl',
          'abre a confirmação de candidatura à vaga do clube (clube nulo: caixa de erro)',
          'opens the confirmation to apply for the club\'s job (null club: error box)'),
         ('apply_for_job_screen_build', 'apply_for_job_screen_build', '0x6b1a80', 'manager_screens.cpp', 'cdecl',
          'monta a confirmação: foto e cores do clube, o nome do técnico no título, a pergunta e os botões Não e Sim',
          'builds the confirmation: the club\'s picture and colours, the manager\'s name as the title, the question and the No and Yes buttons'),
         ('apply_for_job_screen_handle_action', 'apply_for_job_screen_handle_action', '0x6b1c30', 'manager_screens.cpp', 'cdecl',
          'Não fecha; Sim candidata o técnico humano à vaga e fecha',
          'No closes; Yes applies the human manager for the job and closes'),
         ('open_resign_job_screen', 'open_resign_job_screen', '0x6b1d10', 'manager_screens.cpp', 'cdecl',
          'abre a confirmação de demissão do clube (clube nulo: caixa de erro)',
          'opens the confirmation to resign from the club (null club: error box)'),
         ('resign_job_screen_build', 'resign_job_screen_build', '0x6b1dc0', 'manager_screens.cpp', 'cdecl',
          'monta a confirmação de demissão (como a de candidatura, com a sua pergunta)',
          'builds the resignation confirmation (like the application one, with its question)'),
         ('resign_job_screen_handle_action', 'resign_job_screen_handle_action', '0x6b1f70', 'manager_screens.cpp', 'cdecl',
          'Não fecha; Sim tira o técnico do clube, atualiza o interesse dos humanos no dia de jogo e abre a vaga',
          'No closes; Yes removes the manager from the club, refreshes the humans\' match day interest and opens the vacancy'),
         ('open_board_confidence_screen', 'open_board_confidence_screen', '0x6b2080', 'manager_screens.cpp', 'cdecl',
          'abre a tela de confiança da diretoria do clube (clube nulo: caixa de erro)',
          'opens the club\'s board confidence screen (null club: error box)'),
         ('board_confidence_screen_build', 'board_confidence_screen_build', '0x6b2130', 'manager_screens.cpp', 'cdecl',
          'monta a confiança da diretoria (ou da federação): os botões de pedido à diretoria e de reunião com a federação (só clubes) e o texto da confiança pelos dias no cargo, com a opinião sobre o elenco quando houver',
          'builds the board (or F.A.) confidence: the board request and FA meeting buttons (clubs only) and the confidence text by the days in the job, with the squad opinion when there is one'),
         ('board_confidence_screen_handle_action', 'board_confidence_screen_handle_action', '0x6b2370', 'manager_screens.cpp', 'cdecl',
          'abre o pedido à diretoria ou a reunião com a federação, ou passa à barra de menus',
          'opens the board request or the FA meeting, or goes to the menu bar'),
         ('open_job_offer_screen', 'open_job_offer_screen', '0x6b2450', 'manager_screens.cpp', 'cdecl',
          'abre a confirmação de uma oferta de emprego da caixa de notícias, guardando o número da notícia e o clube que oferece',
          'opens the confirmation of a job offer news item, keeping the news id and the offering club'),
         ('job_offer_screen_build', 'job_offer_screen_build', '0x6b25e0', 'manager_screens.cpp', 'cdecl',
          'monta a confirmação da oferta (como a de candidatura, com o clube do parâmetro 1)',
          'builds the offer confirmation (like the application one, with the club of parameter 1)'),
         ('job_offer_screen_handle_action', 'job_offer_screen_handle_action', '0x6b2790', 'manager_screens.cpp', 'cdecl',
          'Não marca a notícia respondida e devolve a vaga; Sim marca e nomeia o técnico humano para o clube',
          'No marks the news item answered and puts the vacancy back; Yes marks it and appoints the human manager to the club')])
