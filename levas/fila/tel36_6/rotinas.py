# The news screen input and the send message screen (news_screens.cpp; reader tel36, the screens round, 21/11/2026).
# The news screen builder is not in the group (the input 0x7892b0 and the exit 0x7897a0 are its handlers). Uses tel36_1
# (tel36_langlib), tel26_1 (tel26_add_item, tel26_add_area, tel26_area_button, tel26_area_text, tel26_item_id,
# tel26_item_value, job_colour) and tel26_4 (manager_menu_bar).
# Conventions checked in the disassembly: NewsItem::get_date thiscall(item, out) ret 4 -> out; the input and the send
# input cdecl(short) -> eax; the exit cdecl(void); the opener cdecl(Staff* addressee, news id); the reply cdecl(Staff*
# manager, news id) with an SEH frame and no destructor; the builder cdecl(void). Calls: ScreenManager::is_manager_done
# 0x7ffe40 thiscall(SM, human) ret 4; get_area_scroll_row 0x803320 thiscall(SM, area) ret 4; News::count_unread
# 0x786fe0 thiscall(news, human, char urgent) ret 8; News::has_urgent_items 0x787350 thiscall(news, human, 0) ret 8;
# the news item's action routine cdecl(human, +0xd6 of parameter 5); get_game_title 0x839570 cdecl() -> 0xdd70a8;
# get_heading_colour 0x839590 cdecl(k) -> ax (eax's high half is k's, 0 for 0 and 1); Display::add_item 0x55f990 (18
# dwords); NewsItem::set_slot 0x785790 thiscall(item, k, v) ret 8; set_slot_string 0x7858c0 thiscall(item, 3, text, 0)
# ret 0xc; News::deliver_to_manager 0x785d40 thiscall(news, &item, staff, 0) ret 0xc; operator new 0x944cf1 / delete
# 0x944c94 / free 0x9452ca cdecl.
# The news input passes count_unread's flag as the dword whose low byte is the filter (the high bytes are the frame's;
# the routine reads the byte). The send input deletes the text and leaves its pointer in parameter 1 (the screen
# closes). With no widget (-1) the action button (id 3) calls address 0 (kept: the same crash).
# Equal in cm0102-1080.exe.
# Comparers: get_date side by side; the builder side by side over the display list (and the game title buffer); the
# rest only the original (they open screens, free, post news, call the news actions or the menu bar).
TELAS = True
ROTINAS = [('news_item_get_date', 0x789280, [0x51, 0x8b, 0x44, 0x24, 0x08]),
           ('news_screens_news_input', 0x7892b0, [0x83, 0xec, 0x10, 0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
           ('news_screens_news_exit', 0x7897a0, [0x6a, 0x02, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('news_screens_open_send_message', 0x7897f0, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('news_screens_reply_to_news_item', 0x789860, [0x6a, 0xff, 0x68, 0xbb, 0xe2, 0x95, 0x00]),
           ('news_screens_build_send_message', 0x789990, [0x81, 0xec, 0xcc, 0x00, 0x00, 0x00]),
           ('news_screens_send_message_input', 0x789bb0, [0x6a, 0xff, 0x68, 0xdb, 0xe2, 0x95, 0x00])]
DOCS = ('a resposta da tela de notícias e a tela de mandar mensagem',
        'the news screen input and the send message screen',
        [('news_item_get_date', 'NewsItem::get_date', '0x789280', 'news_screens.cpp', 'thiscall, ret 4',
          'copia a data da notícia (os 8 bytes em +0xcd) para a saída e devolve a saída',
          'copies the news item\'s date (the 8 bytes at +0xcd) to the output and returns the output'),
         ('news_screens_news_input', 'news_screens_news_input', '0x7892b0', 'news_screens.cpp', 'cdecl',
          'a tela de notícias: com o modo ligado e o técnico ainda não pronto, -5; guarda a rolagem da lista do filtro; contagem de não lidas mudada remonta; aba de filtro (1) troca o filtro; rolagens (2, 9) mudam a posição; o botão de ação (3) chama a rotina da notícia; os botões das urgentes (5, 0x3e9, 0x3e8 com urgentes) passam à próxima ou voltam ao começo; o resto vai para a barra de menus',
          'the news screen: with the mode set and the manager not done, -5; keeps the filter list\'s scroll; a changed unread count rebuilds; a filter tab (1) changes the filter; scrolls (2, 9) move the position; the action button (3) calls the news item\'s routine; the urgent buttons (5, 0x3e9, 0x3e8 with urgent items) move to the next one or back to the start; the rest goes to the menu bar'),
         ('news_screens_news_exit', 'news_screens_news_exit', '0x7897a0', 'news_screens.cpp', 'cdecl',
          'ao sair da tela de notícias, libera a lista do parâmetro 2 e zera os parâmetros 2 e 3',
          'leaving the news screen frees the list of parameter 2 and clears parameters 2 and 3'),
         ('news_screens_open_send_message', 'news_screens_open_send_message', '0x7897f0', 'news_screens.cpp', 'cdecl',
          'abre a tela de mandar mensagem ao destinatário (ou a todos) com um texto novo de 101 bytes, vazio, e o id da notícia',
          'opens the send message screen to the addressee (or everybody) with a new empty 101-byte text and the news id'),
         ('news_screens_reply_to_news_item', 'news_screens_reply_to_news_item', '0x789860', 'news_screens.cpp', 'cdecl',
          'responder a uma notícia: abre a tela de mensagem ao remetente (casa 0 da notícia) quando a casa 1 está ligada, senão a todos',
          'replying to a news item: opens the message screen to the sender (slot 0 of the item) when slot 1 is set, otherwise to everybody'),
         ('news_screens_build_send_message', 'news_screens_build_send_message', '0x789990', 'news_screens.cpp', 'cdecl',
          'monta a tela: o título do jogo, "Send Message To <nome>" (ou "To All"), a caixa de texto da mensagem (100 caracteres) e Cancelar / Enviar (Enviar apagado sem texto)',
          'builds the screen: the game title, "Send Message To <name>" (or "To All"), the message text entry (100 characters) and Cancel / Send (Send greyed with no text)'),
         ('news_screens_send_message_input', 'news_screens_send_message_input', '0x789bb0', 'news_screens.cpp', 'cdecl',
          'digitar remonta; Cancelar apaga o texto e fecha; Enviar manda a notícia 5000 (o remetente, se é para um, o id da notícia e o texto) ao destinatário ou a cada outro técnico humano, apaga o texto e fecha; o resto vai para a barra de menus',
          'typing rebuilds; Cancel deletes the text and closes; Send posts news item 5000 (the sender, whether it is for one, the news id and the text) to the addressee or to every other human manager, deletes the text and closes; the rest goes to the menu bar')])
