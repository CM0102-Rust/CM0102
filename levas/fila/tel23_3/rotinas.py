# The Intertoto application screen, the news link to a competition, the related competitions and stage menus and the
# fixture row of the competition lists (comp_screens.cpp, part 2; reader tel23, the screens round, 21/11/2026).
# Depends on tel23_2 (comp_record_of_key, comp_colours, comp_colours_title), tel22_1 (comp_lang_format,
# comp_crt_format, comp_screens_error_box, COMP_SCREEN_BUILT_FLAG, COMP_SCREEN_DATE_BUTTONS_FLAG), tel22_4
# (comp_click_menubar) and tel26_1 (tel26_add_button, tel26_add_area, tel26_add_item, tel26_area_button,
# tel26_area_text, tel26_copy_text, tel26_item_id, JOB_TEXT, job_colour, JOB_COLOUR_31AA/BDD4/BDE8).
# Conventions checked at the call sites:
# - 0x49cc60 cdecl(news id a, news id b) ret 0 (a news action handler: 0x782b70 passes the item's two ids); 0x7fe600
#   open_screen thiscall(0x49ccb0, 0x49ce50, 1, 0, 0) ret 0x14.
# - 0x49ccb0 cdecl(void): 0x41c410 background_set_comp_picture cdecl(record, 0, 0); 0x75db00 menubar_build(4, 0);
#   0x53b7c0 database_get_comp_colours; 0x803f30 add_screen_title thiscall(c2, c1, name) ret 0xc; 0x803f80
#   add_screen_subtitle thiscall(text, 1) ret 8; 0x5ed070 gui_add_text_area cdecl with 14 arguments (x1, y1, x2, y2, 2,
#   0, 0x10, 4, colour, text, link text, 0x7d0, link value, 0); 0x55fba0 Display::add_area ret 0x2c; 0x8040a0
#   add_area_button ret 0x2c.
# - 0x49ce50 cdecl(widget) -> eax, SEH (no unwind action), NewsItem at +0x10: 0x77bbe0 NewsItem::NewsItem thiscall(0,
#   0) ret 8; 0x786b70 News::find_item_by_id thiscall(a, b, item) ret 0xc (Rust); 0x785830 NewsItem::get_slot
#   thiscall(k) ret 4; 0x9037b0 UefaSeeding::add_intertoto_volunteer thiscall(club) ret 4; 0x786d30 News::set_item_slot
#   thiscall(a, b, slot, value, 0) ret 0x14; 0x7615d0/0x7643d0 menu bar.
# - 0x49eac0 cdecl(news id a, news id b) ret 0 (a news action handler), SEH: 0x48f4a0 comp_screens_open_comp_screen
#   cdecl(record, stage, round, 0).
# - 0x49d070 cdecl(object, colour a, colour b, comps id, history id) -> eax (called as (object, c2, c1, 1, 0xa) by
#   0x48fcd0 and 0x498a30): 0x669470 key_nation_index_of_comp cdecl(record) -> al; 0x804030 add_button_item ret 0x30;
#   0x55f990 Display::add_item ret 0x48; 0x5455a0 comp_get_english_name (Rust); 0x5f2500 history_his_file_exists
#   cdecl(name).
# - 0x49d380 cdecl(comp, selected stage, top) -> eax (called by 0x491380 and 0x492980): 0x48d200 Comp::get_stage
#   thiscall(k) ret 4; 0x4b01d0 comp_util_get_parent_stage_id cdecl(id); 0x49d930 cdecl with 10 arguments.
# - 0x49d930 cdecl(word* area, x1, y1, x2, y2, button text, code, round, stage, index) -> eax (called by 0x495050 and
#   0x49d380): 0x4bffc0 comp_util_format_stage_round_name cdecl(code, round, buf, 0x7d0, 0).
# - 0x49db00 cdecl(fixture, area, int* row, id) -> eax (called by 0x48fcd0, 0x493ee0, 0x717db0): 0x7e6f20
#   comp_is_reserve_or_minor_comp thiscall([0xb74434], record) ret 4; 0x6826d0 get_table_row ret 4; 0x90c310
#   utils_format_ordinal cdecl(n, 3); 0x909ec0 usa_mls_get_club_division cdecl(club); 0x53b900 database_is_national_team;
#   0x53b3d0 database_get_national_team; 0x600a50 is_human_club cdecl(club, 0, 1, 0, 0); 0x729500
#   MatchStats::fixture_entry_matches thiscall(slot, home id, away id, day word, year word) ret 0x14; 0x5fe290
#   HumanManager::is_watching_fixture thiscall(fixture, 0) ret 8 on [0xb63c98]; 0x5e5ad0 font_text_width cdecl(2,
#   text).
# Kept from the original / simplifications: the colour words go in dword slots whose high half is register garbage in
#   the original (0 here); the popup word goes zero-extended into the stage item's area slot.
# Defects of the original: none found.
# Comparers: the builds side by side by the display list (the menus and the fixture row with their answer and the
#   caller's word/row; the error boxes only the original); the Intertoto opener and answer, and the news link (they
#   open screens or write the news and the seeding) only the original, the opener's params and the No answer checked.
TELAS = True
ROTINAS = [('comp_open_intertoto_apply_screen', 0x49cc60, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('comp_intertoto_apply_screen_draw', 0x49ccb0, [0x83, 0xec, 0x0c, 0xa1, 0xf8, 0xf6, 0x9c, 0x00]),
           ('comp_intertoto_apply_screen_handle_event', 0x49ce50, [0x6a, 0xff, 0x68, 0x9b, 0x63, 0x95, 0x00]),
           ('comp_screens_open_comp_from_news_item', 0x49eac0, [0x6a, 0xff, 0x68, 0xbb, 0x63, 0x95, 0x00]),
           ('comp_screens_add_related_comps_menu', 0x49d070, [0xa1, 0x0c, 0xc0, 0x99, 0x00]),
           ('comp_screens_add_stage_selector', 0x49d380, [0x81, 0xec, 0x68, 0x04, 0x00, 0x00]),
           ('comp_screens_add_stage_menu_item', 0x49d930, [0x8b, 0x44, 0x24, 0x18, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_screens_add_fixture_row', 0x49db00, [0x81, 0xec, 0x08, 0x0a, 0x00, 0x00])]
DOCS = ('a candidatura à Intertoto, o link de notícia para uma competição, os menus de competições relacionadas e de fases e a linha de jogo das listas',
        'the Intertoto application, the news link to a competition, the related competitions and stage menus and the fixture row of the lists',
        [('comp_open_intertoto_apply_screen', 'comp_open_intertoto_apply_screen', '0x49cc60', 'comp_screens.cpp', 'cdecl',
          'abre a tela da candidatura à Intertoto de uma notícia (parâmetros 0 e 1 os ids da notícia)',
          "opens a news item's Intertoto application screen (params 0 and 1 the item's ids)"),
         ('comp_intertoto_apply_screen_draw', 'comp_intertoto_apply_screen_draw', '0x49ccb0', 'comp_screens.cpp', 'cdecl',
          'monta a tela da candidatura: título nas cores da Intertoto, "Please Confirm", a pergunta "Apply for the one English Intertoto Cup place ?" (o nome da copa como link) e os botões Não e Sim',
          'builds the application screen: title in the Intertoto colours, "Please Confirm", the question "Apply for the one English Intertoto Cup place ?" (the cup\'s name as a link) and the No and Yes buttons'),
         ('comp_intertoto_apply_screen_handle_event', 'comp_intertoto_apply_screen_handle_event', '0x49ce50', 'comp_screens.cpp', 'cdecl',
          'a resposta da candidatura: Sim inscreve o clube da notícia como voluntário da Intertoto e marca a notícia respondida (vagas 0 e 3 = 1); Não marca respondida sem a vaga (3 = 0); notícia não achada dá a caixa de erro',
          'the application\'s answer: Yes enters the news item\'s club as an Intertoto volunteer and marks the item answered (slots 0 and 3 = 1); No marks it answered without the place (3 = 0); an item not found gives the error box'),
         ('comp_screens_open_comp_from_news_item', 'comp_screens_open_comp_from_news_item', '0x49eac0', 'comp_screens.cpp', 'cdecl',
          'o link de uma notícia para uma competição: abre a tela da competição da vaga 0 na fase da vaga 4 e na rodada da vaga 5',
          "a news item's link to a competition: opens the screen of slot 0's competition at slot 4's stage and slot 5's round"),
         ('comp_screens_add_related_comps_menu', 'comp_screens_add_related_comps_menu', '0x49d070', 'comp_screens.cpp', 'cdecl',
          'o menu das competições relacionadas no canto da tela: as outras competições do país da competição e, havendo o arquivo da história escrita, "History"; o botão e o menu só aparecem com algum item',
          'the related competitions menu in the screen corner: the other competitions of the competition\'s nation and, when the written history file exists, "History"; the button and the menu only appear with an item'),
         ('comp_screens_add_stage_selector', 'comp_screens_add_stage_selector', '0x49d380', 'comp_screens.cpp', 'cdecl',
          'os menus de fases sobre as abas: as fases da competição no menu "Stage" (grupos irmãos juntos como um item, ou cada um como "Group" quando são todas as fases; copas pelas rodadas) e, na fase escolhida, um segundo menu com os grupos ou as rodadas; nenhum para uma liga simples ou uma copa de rodada única',
          'the stage menus above the tabs: the competition\'s stages on the "Stage" menu (sibling groups together as one item, or each as a "Group" when they are every stage; cups by their rounds) and, for the selected stage, a second menu with its groups or rounds; none for a plain league or a single-round cup'),
         ('comp_screens_add_stage_menu_item', 'comp_screens_add_stage_menu_item', '0x49d930', 'comp_screens.cpp', 'cdecl',
          'um item de menu de fases: cria o botão e o menu no primeiro item; o nome da fase ou rodada ("League" quando vazio) com o valor 1000 × (fase + 1) + índice',
          'a stage menu item: makes the button and the menu at the first item; the stage or round name ("League" when empty) with the value 1000 × (stage + 1) + index'),
         ('comp_screens_add_fixture_row', 'comp_screens_add_fixture_row', '0x49db00', 'comp_screens.cpp', 'cdecl',
          'uma linha de jogo das listas de competição: dos dois lados a posição na tabela (ou a conferência da MLS, a divisão, o país), os nomes dos clubes (amarelo o clube humano), as marcas do confronto, o placar com prorrogação, adiado (P) ou abandonado (A), o link para as estatísticas, ou "view"/"v" num jogo por jogar; em campo neutro uma linha com o estádio e a cidade',
          'one fixture row of the competition lists: on both sides the table position (or the MLS conference, the division, the nation), the club names (the human\'s club in yellow), the tie marks, the score with extra time, postponed (P) or abandoned (A), the statistics link, or "view"/"v" for a match to play; on neutral ground a line with the stadium and the city')])
