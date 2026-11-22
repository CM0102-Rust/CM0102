# The board ultimatum confirm screen (ultimatum.cpp; reader tel36, the screens round, 21/11/2026). Uses the helpers of
# tel36_1 (tel36_confirm_build, tel36_confirm_answer, tel36_staff_param, tel36_club_param, tel36_langlib) and tel26_1
# (tel26_error_box).
# Conventions checked in the disassembly: the news action cdecl(Staff*, news id) with an SEH frame and no destructor
# (NewsItem::NewsItem 0x77bbe0 thiscall(item, 0, 0) ret 8; News::find_item_by_id 0x786b70 thiscall(news, person, id,
# &item) ret 0xc; NewsItem::get_slot 0x785830 thiscall(item, k) ret 4; the error box ultimatum.cpp 69); the builder
# cdecl(void) as in tel36_1; the handler cdecl(short) -> eax (MANAGER_MANAGER::handle_manager_ultimatum 0x6ad610
# thiscall(mm, person, club, kind) ret 0xc; News::set_item_slot 0x786d30 thiscall(news, person, id, 5, 1, 0) ret 0x14).
# The handler pushes the kind as a dword whose high bytes are the ecx left on entry (the copy passes the byte).
# Equal in cm0102-1080.exe.
# Comparers: the builder side by side over the display list; the news action and the handler only the original.
TELAS = True
ROTINAS = [('ultimatum_news_issue_ultimatum_action', 0x9064a0, [0x6a, 0xff, 0x68, 0xab, 0x52, 0x96, 0x00]),
           ('ultimatum_screen_draw', 0x906610, [0x83, 0xec, 0x70, 0x53, 0x56]),
           ('ultimatum_screen_handle_event', 0x906830, [0x51, 0x53, 0x55, 0x56, 0x57])]
DOCS = ('a tela de confirmar o ultimato à diretoria',
        'the board ultimatum confirm screen',
        [('ultimatum_news_issue_ultimatum_action', 'ultimatum_news_issue_ultimatum_action', '0x9064a0', 'ultimatum.cpp', 'cdecl',
          'a ação da notícia: com técnico de clube e a notícia achada, abre a tela (parâmetros: o técnico, o tipo lido da casa 1 da notícia, o clube e o id da notícia); notícia não achada dá a caixa de erro de ultimatum.cpp 69',
          'the news action: with a club manager and the news item found, opens the screen (params: the manager, the kind read from slot 1 of the item, the club and the news id); an item not found shows the ultimatum.cpp 69 error box'),
         ('ultimatum_screen_draw', 'ultimatum_screen_draw', '0x906610', 'ultimatum.cpp', 'cdecl',
          'monta o "Please Confirm" com a pergunta do ultimato pelo tipo: 1 ampliar o estádio, 2 melhorar o centro de treinamento, 3 liberar verba de transferências, outro aumentar a folha salarial; com o nome curto do clube',
          'builds the "Please Confirm" with the ultimatum question by kind: 1 expand the stadium, 2 improve the training facilities, 3 make transfer funds available, another increase the wage budget; with the club\'s short name'),
         ('ultimatum_screen_handle_event', 'ultimatum_screen_handle_event', '0x906830', 'ultimatum.cpp', 'cdecl',
          'Sim: com clube, entrega o ultimato à diretoria (MANAGER_MANAGER::handle_manager_ultimatum com o tipo) e marca a notícia (casa 5 = 1); Não fecha; no jogo em rede sempre fecha; o resto vai para a barra de menus',
          'Yes: with a club, hands the ultimatum to the board (MANAGER_MANAGER::handle_manager_ultimatum with the kind) and marks the news item (slot 5 = 1); No closes; in a network game it always closes; the rest goes to the menu bar')])
