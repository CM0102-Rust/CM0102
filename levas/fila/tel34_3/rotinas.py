# The ban appeal confirm screen (discipline.cpp; reader tel34, the screens round, 21/11/2026: screens the community
# map did not see, passed by pointer from the news item's "Appeal" button, motor.rs 0x55e620). Uses the helpers of
# tel26_1 and tel34_1 (tel34_lang_text, Tel34FormatFn): goes after tel34_1.
# Conventions checked in the disassembly: discipline_appeal_news_action cdecl(manager, news id) with an SEH frame
# (a local NewsItem without destructor and a staff name reference with its Date); the builder cdecl(void); the
# handler cdecl(short item) -> eax. News::find_item_by_id 0x786b70 thiscall([0xb7431c], manager, id, item) ret 0xc;
# NewsItem::get_slot 0x785830 thiscall(item, char) (pushed in a dword whose high bytes are ecx's);
# read_staff_name_ref_from_news 0x7e52c0 cdecl(item, &slot, out) (staff id at out +0xc); background_set_staff_picture
# 0x41c790 cdecl(staff, 0, 0); MANAGER_MANAGER::submit_ban_appeal 0x6ab800 thiscall([0xb67a44], 11 args) ret 0x2c: the
# original pushes four dwords read across its byte locals (params 10..13), so their high bytes hold the neighbouring
# params and then stack leftovers (zero here), and the dword of its own argument slot with the low byte overwritten by
# param 0xe (kept: (raw & ~0xff) | p14); News::set_item_slot 0x786d30 thiscall(News, manager, id, 0, 1, 0).
TELAS = True
ROTINAS = [('discipline_appeal_news_action', 0x55e620, [0x6a, 0xff, 0x68, 0x26, 0x80, 0x95, 0x00]),
           ('discipline_appeal_dialog_build', 0x55ea10, [0x81, 0xec, 0xf4, 0x03, 0x00, 0x00]),
           ('discipline_appeal_dialog_handle_click', 0x55ed00, [0x83, 0xec, 0x14, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('o recurso contra suspensão: abrir pela notícia, a tela de confirmação e a resposta',
        'the ban appeal: opening it from the news item, the confirm screen and the answer',
        [('discipline_appeal_news_action', 'discipline_appeal_news_action', '0x55e620', 'discipline.cpp', 'cdecl',
          'o botão Recorrer da notícia da suspensão abre a confirmação e copia a suspensão da notícia (as casas 0..14 e o punido) para os parâmetros 0..6 e 8..0x14 da tela',
          "the news item's Appeal button opens the confirm screen and copies the ban out of the news item (slots 0..14 and the banned person) into screen parameters 0..6 and 8..0x14"),
         ('discipline_appeal_dialog_build', 'discipline_appeal_dialog_build', '0x55ea10', 'discipline.cpp', 'cdecl',
          'monta a confirmação: a figura do punido, o nome do técnico nas cores do clube, a pergunta (suspensão de N jogos, de N dias ou sem prazo; em sueco com um jogo, o texto sueco próprio) e os botões Não e Sim',
          "builds the confirm screen: the banned person's picture, the manager's name in the club colours, the question (a ban of N games, of N days or open-ended; in Swedish with one game, its own Swedish text) and the No and Yes buttons"),
         ('discipline_appeal_dialog_handle_click', 'discipline_appeal_dialog_handle_click', '0x55ed00', 'discipline.cpp', 'cdecl',
          'Não fecha (-9); Sim manda o recurso à federação com a suspensão dos parâmetros, marca a notícia como respondida e fecha (-9); o resto vai para a barra de menu',
          'No closes (-9); Yes submits the appeal to the FA with the ban in the parameters, marks the news item answered and closes (-9); anything else goes to the menu bar')])
