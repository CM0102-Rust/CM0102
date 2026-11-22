# The competition history screen (comp_screens.cpp, part 1; reader tel22, the screens round, 21/11/2026): opening,
# building (the tab bar and the Records tab, inline) and the Winners tab.
# - 0x498780 comp_screens_open_history_screen cdecl(comp record, restore) ret 0 (0xf0 bytes of locals: the previous
#   screen's 60 params when restoring); 0x7fe600 open_screen thiscall(builder 0x498a30, handler 0x49b250, 0, leave
#   0x49b7e0, 0) ret 0x14; 0x7ff270 set_screen_param thiscall(k, v, 0) ret 0xc. Param 0 = G_COMP_OBJECT_TABLE[id].
# - 0x498a30 comp_screens_draw_history_screen cdecl(void) ret 0, SEH, 0x1f88 bytes of locals (25 MenuItem slots of
#   0xd4 by the CRT vector iterators: text +4, kind +0xcc, value +0xd0); jump table 0x4991c8 (7 tabs) inside the region.
#   Calls: 0x41c8e0 background_set_section_picture cdecl(8, 0); 0x53b7c0 database_get_comp_colours; 0x803f30
#   add_screen_title; 0x75db00 menubar_build(4, 0); the object's virtual slots 0x50 (history kind, thiscall -> al) and
#   0x88 (has hosts); 0x5ed780 gui_add_tab_bar(items, n, tab, &top, &bottom, 1); the tabs cdecl(object, top, bottom):
#   0x4991f0 (here) and 0x499900/0x499e10/0x49a320/0x49a830/0x49ad40 (tel23, by address); Records inline: 0x804030
#   add_button_item ret 0x30, 0x55fba0 Display::add_area (the popup, flags 0x130), 0x4a5010 period label thiscall(k,
#   buf, 0xc8) ret 0xc, 0x456890 add_ticked_popup_item / 0x415640 add_menu_item thiscall ret 0x10, 0x803fd0
#   add_text_item ret 0x2c, 0x4567f0 ScreenManager::add_area ret 0x2c, 0x4a54c0 is_record_type_shown ret 4, 0x4a5120
#   record title ret 0xc, 0x4a5530 get_record_text thiscall(view byte, type, Text*) ret 0xc, 0x4aeaf0
#   Text::draw_wrapped ret 0x20, 0x46ad90 Display::set_area_scroll_row ret 8; 0x49d070 related comps menu cdecl(object,
#   colour, colour, 1, 0xa); 0x7febb0/0x7fec60/0x5edcc0 Back/Next.
# - 0x4991f0 comp_screens_draw_history_winners_tab cdecl(object, top, bottom) ret 0: 0x945333 qsort(table, count, 0x1a,
#   0x540780), 0x600a50 is_human_club cdecl(club, 0, 1, 0, 0), 0x804100/0x8040a0 area text/button, 0x403320
#   Area::set_scroll_row thiscall ret 4.
# Kept from the original: a Records view other than 1 and 2 leaves the heading as stack garbage (empty here); the
#   column and row counts of the Winners area go as dwords whose high bytes are stack garbage in the original, and
#   the colour words in dword slots with register garbage in the high half (0 here; Display keeps bytes and words).
# Comparers: opening only the original, checked against the plan (comp_history_planned_params); the build side by
#   side by the display list for the Records tab and for Winners once its rows are known (param 2 != -1); the first
#   Winners build (qsort of the global club history table), the tel23 tabs and the error box only the original.
TELAS = True
ROTINAS = [('comp_screens_open_history_screen', 0x498780, [0x81, 0xec, 0xf0, 0x00, 0x00, 0x00]),
           ('comp_screens_draw_history_screen', 0x498a30, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('comp_screens_draw_history_winners_tab', 0x4991f0, [0x83, 0xec, 0x1c, 0x53, 0x55])]
DOCS = ('a tela de história da competição: abrir, montar (a barra de abas e a aba Recordes) e a aba Campeões',
        'the competition history screen: opening, building (the tab bar and the Records tab) and the Winners tab',
        [('comp_screens_open_history_screen', 'comp_screens_open_history_screen', '0x498780', 'comp_screens.cpp', 'cdecl',
          'abre a tela de história da competição: parâmetro 0 o objeto da competição, 1 a aba (1 Campeões), 2 e 3 a primeira e a última linha da tabela de campeões (-1: a procurar), 6 a visão dos recordes (1), 0xb, 0xe, 0x11, 0x14 e 0x17 as visões das outras abas (6, 9, 0xb, 3, 0xf), o resto zero; restaurando, a aba e as visões voltam da tela anterior',
          "opens the competition history screen: param 0 the competition object, 1 the tab (1 Winners), 2 and 3 the first and last row of the winners table (-1: to be searched), 6 the records view (1), 0xb, 0xe, 0x11, 0x14 and 0x17 the other tabs' views (6, 9, 0xb, 3, 0xf), the rest zero; restoring, the tab and the views come back from the previous screen"),
         ('comp_screens_draw_history_screen', 'comp_screens_draw_history_screen', '0x498a30', 'comp_screens.cpp', 'cdecl',
          'monta a tela de história: título "<competição> History" nas cores dela, a barra de menus, as abas (Campeões só se a competição tem história, Recordes, Público, Resultados, Sequências, Times, Jogadores) e a aba aberta; na aba Recordes, o botão Ver (os dois períodos), um recorde por linha (título e texto quebrado) e linhas vazias; no fim o menu das competições relacionadas e Voltar/Avançar',
          'builds the history screen: the title "<competition> History" in its colours, the menu bar, the tabs (Winners only when the competition has history, Records, Attendances, Results, Sequences, Teams, Players) and the open tab; in the Records tab, the View button (the two periods), one record per row (title and wrapped text) and empty rows; then the related competitions menu and Back/Next'),
         ('comp_screens_draw_history_winners_tab', 'comp_screens_draw_history_winners_tab', '0x4991f0', 'comp_screens.cpp', 'cdecl',
          'a aba Campeões: na primeira vez ordena a tabela de história dos clubes e acha as linhas da competição; colunas ano, campeão, vice, terceiro (competições de tipo 3; "Minor Premiers" numa) e sedes (quando há); nas Olimpíadas Ouro, Prata e Bronze; o clube humano em amarelo, botão quando o clube tem página',
          "the Winners tab: the first time it sorts the club history table and finds the competition's rows; columns year, winner, runner-up, third (kind 3 competitions; \"Minor Premiers\" in one) and hosts (when there are); at the Olympics Gold, Silver and Bronze; a human's club in yellow, a button when the club has a page")])
