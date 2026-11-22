# The news screen (news_screens.cpp, part 1; reader tel28, the screens round, 21/11/2026): opening a manager's inbox
# and building it (the default selection, the list rows and the lower half are the named pieces
# news_screen_pick_default, news_screen_list_rows and news_screen_lower). The rest of the file (the input, the exit,
# the send message screen) is tel36's. Depends on tel28_1 (tel28_lang, tel28_crt, tel28_text_item) and tel26_1.
# Conventions checked in the disassembly (every push against esp):
# - 0x788010 news_screens_open_news_screen cdecl(record, category) ret 0 (match_day and the menu bar call it); 0x7fe4a0
#   open_screen_for_each_record thiscall(record, build 0x7881e0, input 0x7892b0, 0, exit 0x7897a0, 0) ret 0x18;
#   0x944cf1 operator new(0x1e); 0x944c9f sprintf; 0x7ff130 set_screen_param_for_each_record thiscall(record, k, v,
#   flag) ret 0x10.
# - 0x7881e0 news_screens_build_news_screen cdecl(void), SEH, 0x16bc bytes of locals (__alloca_probe): 25 MenuItems
#   (0xd4, the CRT vector iterators), two Texts (8 bytes; 0x4ae590 Text::Text thiscall(0) ret 4, 0x4ae630 ~Text).
#   Calls: 0x945a60 strstr; 0x787350 News::has_urgent_items thiscall(manager, 1) ret 8; 0x7ffe40 is_manager_done
#   thiscall(manager) ret 4; 0x41c0d0 background_set_picture cdecl(0); 0x786960 News::get_inbox_items thiscall(manager,
#   &count, category) ret 0xc; 0x41bfb0 background_set_random_picture; 0x53b640 database_get_club_kit_colours
#   cdecl(club, 1, &front, &back); 0x839590 get_heading_colour cdecl (1 then 0); 0x53dba0 database_format_staff_name
#   cdecl(manager, buf, 0x190, 1); 0x66f4d0 langlib_get_language (2: the French heading, jump table 0x789238 by the
#   name's first letter, d' before AEIOUYaeiouy); 0x4ae660 Text::set_string thiscall(text) ret 4; 0x4ae8a0
#   Text::add_link thiscall(name, 0x7d7, manager) ret 0xc; 0x55fba0 Display::add_area thiscall(11) ret 0x2c; 0x4aeaf0
#   Text::draw_wrapped thiscall(area, x1, x2, &row, font, colour, style, 0) ret 0x20; 0x803f30 add_screen_title
#   thiscall(back, front, text) ret 0xc; 0x5ed780 gui_add_tab_bar cdecl(items, 8, tab, &top, &bottom, 1); 0x5e5cd0
#   font_get_height cdecl; 0x77bc80 NewsItem::write_text thiscall(manager, text, body, flag) ret 0x10; 0x806260 the
#   filter match cdecl(text, filter, 0); 0x4afa40 Text::clear thiscall; 0x782b70 NewsItem::get_action_button
#   thiscall(manager, buf, 0x7d0, &data) ret 0x10 -> al; 0x789280 NewsItem::get_date thiscall(out) ret 4; 0x54b550
#   Date::get_weekday_string thiscall(buf, 0x7d0, slice) ret 0xc; 0x54a650 Date::get_short_date_string thiscall(buf,
#   0x7d0, 0xff) ret 0xc; 0x804100 add_area_text thiscall(9) ret 0x24; 0x8040a0 add_area_button thiscall(11) ret
#   0x2c; 0x787140 News::mark_item_read thiscall(manager, id, 1) ret 0xc; 0x403320 Area::set_scroll_row; 0x55f990
#   Display::add_item thiscall(18) ret 0x48; 0x786fe0 News::count_unread thiscall(manager, 0) ret 8; 0x804030
#   add_button_item thiscall(12) ret 0x30; 0x803fd0 add_text_item thiscall(11) ret 0x2c; 0x75db00 menubar_build
#   cdecl(4, reply); 0x7febb0/0x7fec60/0x5edcc0 Back/Next.
# - The 1080 exe changes three immediates of the build (the date cell's font 1 -> 2 at 0x788a33 and 0x788a86, the
#   body's width 0x302 -> 0x44c at 0x78900f): the copy reads them from the running exe.
# Kept from the original: the dwords the original builds over other locals keep those upper bytes (the list rows
#   over the reset parameter, the day slice over the list pointer, the row colour over the first item's id); the
#   colour words otherwise go zero-extended.
# Defects of the original: none found.
# Comparers: opening only the original, the params checked against the plan when the record's screen is the current
#   one; the build side by side by the display list when it only reads (an open list, a read item selected, no
#   refresh, no reset), otherwise only the original (it marks the item read, allocates or frees the list).
# Identical in cm0102-1080.exe: the opener whole; the build but for the three immediates above.
TELAS = True
ROTINAS = [('news_screens_open_news_screen', 0x788010, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('news_screens_build_news_screen', 0x7881e0, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a tela de notícias: abrir a caixa de entrada e montá-la',
        'the news screen: opening the inbox and building it',
        [('news_screens_open_news_screen', 'news_screens_open_news_screen', '0x788010', 'news_screens.cpp', 'cdecl',
          'abre a caixa de entrada do registro: o filtro de 30 bytes (espaços com o marcador "#;.,234-][+-=-"), parâmetro 0 a categoria, 8 o filtro (liberado com a tela), o resto 0; sem memória, a caixa de news_screens.cpp 0x66',
          'opens the record\'s inbox: the 30-byte filter (spaces with the placeholder "#;.,234-][+-=-"), parameter 0 the category, 8 the filter (freed with the screen), the rest 0; with no memory, the news_screens.cpp 0x66 box'),
         ('news_screens_build_news_screen', 'news_screens_build_news_screen', '0x7881e0', 'news_screens.cpp', 'cdecl',
          'monta a caixa de entrada: as notícias do técnico na categoria (abas Todas, Mensagens, Competições, Lesões e Suspensões, Contratos e Mídia, Transferências, Empregos, Recordes), filtradas pelo texto; sem escolha, a mais urgente (não lida, importante ou pedindo resposta); a escolhida aberta e marcada lida, com a manchete, o texto e o botão de ação (resposta); o filtro, Próxima Não Lida e a contagem de não lidas',
          'builds the inbox: the manager\'s news of the category (tabs All, Messages, Competitions, Injuries and Bans, Contracts and Media, Transfers, Jobs, Records), filtered by the text; with no choice, the most urgent (unread, important or asking for a reply); the chosen one open and marked read, with its headline, text and action button (reply); the filter, Next Unread and the unread count')])
