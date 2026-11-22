# The FA Cup fifth round entry confirm screen (eng_fa_cup.cpp; reader tel34, the screens round, 21/11/2026: passed by
# pointer from the news item's "Respond" button, motor.rs 0x5708f0). Uses the helpers of tel26_1 and tel34_1
# (tel34_lang_text, Tel34FormatFn): goes after tel34_1.
# Conventions checked in the disassembly: the open cdecl(manager, news id); the builder cdecl(void); the handler
# cdecl(short item) -> eax. background_set_comp_picture 0x41c410 cdecl(comp, 0, 0); database_get_comp_colours 0x53b7c0
# cdecl(comp, &first, &second) and add_screen_title(SM, second, first, name); gui_add_text_area 0x5ed070 cdecl with one
# link triple (name, 0x7d0, comp) and the closing 0 (14 dwords); News::set_item_slot 0x786d30 thiscall(News, manager,
# id, 0, value, 0). "No" is id 2 (left) and "Yes" id 1 (right).
TELAS = True
ROTINAS = [('fa_cup_open_fifth_round_entry_screen', 0x5708f0, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('fa_cup_draw_fifth_round_entry_screen', 0x570940, [0x83, 0xec, 0x0c, 0xa1, 0x4c, 0xf7, 0x9c, 0x00]),
           ('fa_cup_handle_fifth_round_entry_screen', 0x570ae0, [0x53, 0x55, 0x56, 0x57, 0x6a, 0x00])]
DOCS = ('a entrada na quinta fase da Copa da Inglaterra: abrir pela notícia, a tela de confirmação e a resposta',
        'the FA Cup fifth round entry: opening it from the news item, the confirm screen and the answer',
        [('fa_cup_open_fifth_round_entry_screen', 'fa_cup_open_fifth_round_entry_screen', '0x5708f0', 'eng_fa_cup.cpp', 'cdecl',
          'abre a confirmação e guarda o técnico (parâmetro 0) e a notícia (parâmetro 1)',
          'opens the confirm screen and keeps the manager (parameter 0) and the news item (parameter 1)'),
         ('fa_cup_draw_fifth_round_entry_screen', 'fa_cup_draw_fifth_round_entry_screen', '0x570940', 'eng_fa_cup.cpp', 'cdecl',
          'monta a confirmação: figura, nome e cores da Copa da Inglaterra, "Entrar na Copa na quinta fase?" com o nome ligado à competição, Não (id 2) e Sim (id 1)',
          "builds the confirm screen: the FA Cup's picture, name and colours, \"Enter the FA Cup at fifth round ?\" with the name linked to the competition, No (id 2) and Yes (id 1)"),
         ('fa_cup_handle_fifth_round_entry_screen', 'fa_cup_handle_fifth_round_entry_screen', '0x570ae0', 'eng_fa_cup.cpp', 'cdecl',
          'Sim marca a entrada na quinta fase no objeto da copa (+0xb2 = 2) e responde a notícia com 2; Não responde com 3; ambos fecham (-9); o resto vai para a barra de menu',
          'Yes marks the fifth round entry in the cup object (+0xb2 = 2) and answers the news item with 2; No answers it with 3; both close (-9); anything else goes to the menu bar')])
