# The media response screen (media.cpp; reader tel35, the screens round, 21/11/2026). Uses the helpers of tel26_1
# (tel26_add_area, tel26_area_button, tel26_item_id, tel26_item_value, tel26_slot_manager, tel26_error_box, JOB_TEXT),
# tel26_4 (manager_menu_bar) and tel35_1 (TEL35_NETWORK_FLAG, tel35_colour, tel35_text_draw_wrapped).
# Conventions checked in the disassembly: the open cdecl(manager, news id) with an SEH frame and a local NewsItem
# (NewsItem::NewsItem 0x77bbe0 thiscall(item, 0, 0), no destructor); News::find_item_by_id 0x786b70 thiscall(news,
# manager, id, item) ret 0xc; NewsItem::get_slot 0x785830 thiscall(item, k) ret 4; open_screen (SM, 0x7423c0,
# 0x742de0, 1, 0x5a8f60, 0). The builder cdecl(void) with a local Text; database_format_name_from_ids 0x53e140
# cdecl(5, 6, 7, name, 400, 1, 4); background_set_club_picture cdecl(club, 0, 0); the kit colours and the staff name as
# the other screens; Media::format_response_option_label 0x735160 thiscall([0xb6b7c8], code, k, out, 2000) ret 0x10;
# the question's two-level jump table (bytes at 0x742d58 for code − 1 in 0..0x81, dwords at 0x742cc4) read. The
# handler cdecl(short) -> eax; Date::Date 0x54a110 thiscall; bsearch 0x946464 cdecl(key, base, n, 0x17, 0x757ee0);
# Media ([0xb6b7c8]) warn_squad_of_position_rival 0x758010 (person, club, club, response) ret 0x10,
# apply_team_story_response 0x73e870 (club, code word, response, person) ret 0x10, apply_sent_off_story_response
# 0x75a340 (person, club, club, response) ret 0x10, apply_wants_first_team_response 0x75b440 (person, club, response)
# ret 0xc, apply_player_story_response 0x741020 (person, club, code word, response) ret 0x10; News::set_item_slot
# 0x786d30 thiscall(news, manager, id, slot, response, 0) ret 0x14; set_widget_text_colour 0x801960; redraw_screen.
# The code word passed to the story responses is the code's low byte sign-extended into the low half, the high half
# what the register held (the code's own high half, or the staff table's for a team story with a person); kept.
# Equal in cm0102-1080.exe.
# Defects of the original: the open leaves parameter 8 as an uninitialised local for the kinds other than 0xbdf,
# 0xbe6 and 0xbe8 (here 0); the question area takes seven column weights from a two-byte array (the rows byte, the
# three high bytes of parameter 6 and an uninitialised byte follow; here the same with 0 for the last); the name
# buffer is 200 bytes and database_format_name_from_ids is told 400 (here 400).
# Comparers: the builder side by side over the display list, tolerating a difference only inside one area's column
# weights (the uninitialised seventh weight); the open and the handler (they open the screen, apply the response and
# mark the news item, redraw, run the menu bar) only in the original.
MOTOR = True
TELAS = True
ROTINAS = [('media_response_screen_open', 0x741f50, [0x6a, 0xff, 0x68, 0x5b, 0xd7, 0x95, 0x00]),
           ('media_response_screen_draw', 0x7423c0, [0x6a, 0xff, 0x68, 0x7b, 0xd7, 0x95, 0x00]),
           ('media_response_screen_handle_event', 0x742de0, [0x6a, 0xff, 0x68, 0x9b, 0xd7, 0x95, 0x00])]
DOCS = ('a tela de responder à imprensa',
        'the media response screen',
        [('media_response_screen_open', 'media_response_screen_open', '0x741f50', 'media.cpp', 'cdecl',
          'abre a resposta a uma notícia de imprensa do técnico, guardando nos parâmetros as pessoas e os clubes da história conforme o tipo da notícia (história de time, de jogador, expulsão, quer jogar no time titular, contratação rival); sem técnico ou sem a notícia: caixa de erro',
          'opens the response to a media news item of the manager, keeping in the parameters the story\'s people and clubs as the news kind lays them out (team story, player story, sent off, wants first-team football, rival signing); with no manager or no item: error box'),
         ('media_response_screen_draw', 'media_response_screen_draw', '0x7423c0', 'media.cpp', 'cdecl',
          'monta a tela: a foto e as cores do clube, o nome do técnico, "Select Media Response", a pergunta da história (com o nome da pessoa, o clube ou a nação, conforme o código) e as três respostas (a escolhida em amarelo), Cancelar e Ok',
          'builds the screen: the club\'s picture and colours, the manager\'s name, "Select Media Response", the story\'s question (with the person\'s name, the club or the nation, by the code) and the three responses (the chosen one yellow), Cancel and Ok'),
         ('media_response_screen_handle_event', 'media_response_screen_handle_event', '0x742de0', 'media.cpp', 'cdecl',
          'uma resposta fica amarela e é guardada; Ok com resposta a registra no histórico de histórias, aplica conforme o tipo da notícia (história de time, expulsão, quer jogar, de jogador; e o aviso ao elenco do rival) e marca a notícia respondida; Cancelar fecha; no jogo em rede sempre fecha',
          'a response turns yellow and is kept; Ok with a response logs it in the story log, applies it by the news kind (team story, sent off, wants to play, player story; and the warning to the rival\'s squad) and marks the news item answered; Cancel closes; in a network game it always closes')])
