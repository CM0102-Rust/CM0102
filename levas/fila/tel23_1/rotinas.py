# The competition history screen (comp_screens.cpp, part 2; reader tel23, the screens round, 21/11/2026): the five list
# tabs (Attendances, Results, Sequences, Teams, Players: one code with the tab's numbers), the screen's click handler,
# the Players tab's clicks, leaving it and the record type -> tab table. Depends on tel22_1 (comp_lang_format,
# comp_crt_format, comp_screens_error_box, comp_text_item, comp_free_block_param, COMP_SCREEN_DATE_BUTTONS_FLAG),
# tel22_4 (comp_click_action_id, comp_click_keep_scroll, comp_click_menubar, comp_click_compare) and tel26_1
# (tel26_add_button, tel26_add_area, tel26_add_item, tel26_area_text, tel26_copy_text, tel26_item_id, tel26_item_value,
# display_area_at, JOB_TEXT, job_colour, JOB_COLOUR_31AA).
# Conventions checked at the call sites:
# - 0x499900/0x499e10/0x49a320/0x49a830/0x49ad40 cdecl(object, top, bottom) ret 0 (called by 0x498a30 with the tab
#   bar's top and bottom), SEH, 0x8bc bytes of locals (Text at +0x2c, 8 bytes; title buffer 0xc8; heading 0x7d0).
#   Byte-identical but for the tab number: the view param 0xb/0xe/0x11/0x14/0x17, the popup id 5..9, the tab compared
#   with comp_history_tab_for_record 3..7, the scroll and area params. Calls: 0x4ae590 Text::Text thiscall(0) ret 4,
#   0x4ae630 ~Text; 0x804030 add_button_item thiscall(12) ret 0x30; 0x55fba0 Display::add_area thiscall(11) ret 0x2c
#   and 0x55f990 Display::add_item thiscall(18) ret 0x48 on the current human's Display; 0x49eb80 cdecl(type, 0);
#   0x4a54c0 Comp_Stats::is_record_type_shown thiscall(type) ret 4; 0x4a5120 get_record_title thiscall(type, buf,
#   size) ret 0xc; 0x803fd0 add_text_item ret 0x2c; 0x5e5cd0 font_get_height cdecl(2); 0x4ab0c0
#   build_season_records_list thiscall(&count) ret 4; 0x7ff270 set_screen_param thiscall(k, v, 0) ret 0xc; 0x6697c0
#   key_nation_format_comp_season cdecl(season, comp record, buf, 0x7d0); 0x804100 add_area_text ret 0x24; 0x4a5690
#   comp_stats_format_record_text thiscall(record, type (its byte), Text*) ret 0xc; 0x4aeaf0 Text::draw_wrapped
#   thiscall(area, 0xbe, 0x2ee, &row, 2, colour, 2, 0) ret 0x20; 0x403320 Area::set_scroll_row thiscall(row) ret 4.
# - 0x49b250 cdecl(widget) -> eax; jump table 0x49b6bc (7 tabs) inside the region. 0x468b40 get_widget_action_id
#   thiscall(widget) ret 4; 0x468b90 get_widget_action_data thiscall(widget) ret 4; 0x4739a0
#   Display::get_widget_data thiscall(widget) ret 4; 0x803320 get_area_scroll_row thiscall(area) ret 4; 0x498780
#   comp_screens_open_history_screen cdecl(comp, 1); 0x5455a0 comp_get_english_name cdecl (Rust); 0x5f11c0
#   history_screen_open cdecl(name); 0x7615d0/0x7643d0 menubar_handle_command/object_link cdecl(widget).
# - 0x49b6e0 cdecl(widget word) -> eax; 0x49b7e0 cdecl(void): 0x9452ca crt free; 0x49eb80 cdecl(type byte, out*) -> al,
#   jump table 0x49eca0 (6 cases) and index bytes 0x49ecb8 (22) inside the region.
# Kept from the original / simplifications: the heading buffer is zeroed (stack garbage in the original when no type
#   is ticked); the row count goes as its byte (Display keeps the byte); the colour words go in dword slots whose high
#   half is register garbage in the original (0 here).
# Defect of the original: a Records title with a type outside 1..0x16 opens tab 3 and writes the view into the param
#   numbered by the widget (the out slot of 0x49eb80 is the handler's own argument and the error path leaves it).
# Comparers: the list tabs side by side by the display list (+ [0xadae0c]) once the season records list exists (param
#   9); the first build (it allocates the list) only the original. The handler side by side over the screen manager
#   and the params when the click only writes params (comp_click_compare); opening screens and the menu bar only the
#   original. The Players clicks side by side. Leaving only the original, params 9/0xa checked. The type -> tab table
#   side by side by value (not for a type outside 1..0x16: the error box).
TELAS = True
ROTINAS = [('comp_screens_draw_history_attendances_tab', 0x499900, [0x6a, 0xff, 0x68, 0xdb, 0x62, 0x95, 0x00]),
           ('comp_screens_draw_history_results_tab', 0x499e10, [0x6a, 0xff, 0x68, 0xfb, 0x62, 0x95, 0x00]),
           ('comp_screens_draw_history_sequences_tab', 0x49a320, [0x6a, 0xff, 0x68, 0x1b, 0x63, 0x95, 0x00]),
           ('comp_screens_draw_history_teams_tab', 0x49a830, [0x6a, 0xff, 0x68, 0x3b, 0x63, 0x95, 0x00]),
           ('comp_history_draw_players_tab', 0x49ad40, [0x6a, 0xff, 0x68, 0x5b, 0x63, 0x95, 0x00]),
           ('comp_history_screen_handle_event', 0x49b250, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('comp_history_players_tab_handle_click', 0x49b6e0, [0x56, 0x66, 0x8b, 0x74, 0x24, 0x08]),
           ('comp_history_screen_close', 0x49b7e0, [0x6a, 0x09, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('comp_history_tab_for_record', 0x49eb80, [0x0f, 0xbe, 0x44, 0x24, 0x04])]
_ABA_PT = ('a aba {} da tela de história: o botão Ver com os tipos de recorde da aba (os que a competição guarda marcáveis, '
           'o escolhido marcado e usado como título; os outros apagados), o título em amarelo e uma área de duas colunas: '
           'cada temporada com recorde, o rótulo da temporada e o texto do recorde quebrado; linhas vazias no resto; a '
           'lista das temporadas é montada na primeira vez (parâmetros 9 e 0xa); visão no parâmetro {}, rolagem no {}, área no {}')
_ABA_EN = ("the history screen's {} tab: the View button with the tab's record types (the ones the competition keeps "
           'selectable, the chosen one ticked and used as the heading; the others greyed), the heading in yellow and a '
           "two-column area: each season with a record, the season's label and the record's text wrapped; empty rows "
           "for the rest; the seasons list is built the first time (params 9 and 0xa); view in param {}, scroll in {}, "
           'area in {}')
DOCS = ('a tela de história da competição (2): as abas Público, Resultados, Sequências, Times e Jogadores, os cliques e a saída',
        "the competition history screen (2): the Attendances, Results, Sequences, Teams and Players tabs, its clicks and leaving it",
        [('comp_screens_draw_history_attendances_tab', 'comp_screens_draw_history_attendances_tab', '0x499900', 'comp_screens.cpp', 'cdecl',
          _ABA_PT.format('Público', '0xb', '0xc', '0xd'), _ABA_EN.format('Attendances', '0xb', '0xc', '0xd')),
         ('comp_screens_draw_history_results_tab', 'comp_screens_draw_history_results_tab', '0x499e10', 'comp_screens.cpp', 'cdecl',
          _ABA_PT.format('Resultados', '0xe', '0xf', '0x10'), _ABA_EN.format('Results', '0xe', '0xf', '0x10')),
         ('comp_screens_draw_history_sequences_tab', 'comp_screens_draw_history_sequences_tab', '0x49a320', 'comp_screens.cpp', 'cdecl',
          _ABA_PT.format('Sequências', '0x11', '0x12', '0x13'), _ABA_EN.format('Sequences', '0x11', '0x12', '0x13')),
         ('comp_screens_draw_history_teams_tab', 'comp_screens_draw_history_teams_tab', '0x49a830', 'comp_screens.cpp', 'cdecl',
          _ABA_PT.format('Times', '0x14', '0x15', '0x16'), _ABA_EN.format('Teams', '0x14', '0x15', '0x16')),
         ('comp_history_draw_players_tab', 'comp_history_draw_players_tab', '0x49ad40', 'comp_screens.cpp', 'cdecl',
          _ABA_PT.format('Jogadores', '0x17', '0x18', '0x19'), _ABA_EN.format('Players', '0x17', '0x18', '0x19')),
         ('comp_history_screen_handle_event', 'comp_history_screen_handle_event', '0x49b250', 'comp_screens.cpp', 'cdecl',
          'os cliques da tela de história: guarda a rolagem da lista da aba; Recordes: o período (id 3) e o título de um recorde (id 4: abre a aba do tipo com a visão dele); as abas de lista: o tipo do botão Ver; Jogadores pelo seu tratador; id 1 abre a história de outra competição, id 0xa a história escrita, id 2 troca de aba; o resto vai à barra de menus',
          "the history screen's clicks: keeps the tab list's scroll; Records: the period (id 3) and a record title (id 4: opens its type's tab with that view); the list tabs: the View button's type; Players through its handler; id 1 opens another competition's history, id 0xa the written history, id 2 changes tab; the rest goes to the menu bar"),
         ('comp_history_players_tab_handle_click', 'comp_history_players_tab_handle_click', '0x49b6e0', 'comp_screens.cpp', 'cdecl',
          'os cliques da aba Jogadores: guarda a rolagem (parâmetro 0x18) e o tipo escolhido no botão Ver (id 9) vai ao parâmetro 0x17',
          "the Players tab's clicks: keeps the scroll (param 0x18) and the type picked on the View button (id 9) goes to param 0x17"),
         ('comp_history_screen_close', 'comp_history_screen_close', '0x49b7e0', 'comp_screens.cpp', 'cdecl',
          'ao sair da tela de história, libera a lista das temporadas (parâmetro 9) e zera os parâmetros 9 e 0xa',
          'on leaving the history screen, frees the seasons list (param 9) and clears params 9 and 0xa'),
         ('comp_history_tab_for_record', 'comp_history_tab_for_record', '0x49eb80', 'comp_screens.cpp', 'cdecl',
          'a aba de um tipo de recorde e o parâmetro da visão dela: 1 nenhuma; 2-5 Times (0x14); 6-8 Público (0xb); 9-10 Resultados (0xe); 11-14 Sequências (0x11); 15-22 Jogadores (0x17); outro tipo dá a caixa de erro e a aba 3',
          "a record type's tab and the param of its view: 1 none; 2-5 Teams (0x14); 6-8 Attendances (0xb); 9-10 Results (0xe); 11-14 Sequences (0x11); 15-22 Players (0x17); any other type gives the error box and tab 3")])
