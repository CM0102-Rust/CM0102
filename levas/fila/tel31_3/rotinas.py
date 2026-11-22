# As telas de contrato (contract_screens.cpp; leitor tel31, rodada das telas, 21/11/2026; grupo novo da varredura,
# telas passadas por ponteiro): o pedido de transferência do jogador (aberto pela notícia), a oferta de contrato a
# um membro da comissão, rescindir contrato e a ordem do elenco por jogos.
# Usa os auxiliares do tel26_1 (tel26_text_area, tel26_add_area, tel26_area_button, tel26_item_id, job_colour,
# JOB_TEXT, tel26_slot_manager) e o manager_menu_bar do tel26_4: entra depois deles.
# Convenções conferidas na desmontagem: o abridor da notícia cdecl(manager, id); o abridor de rescisão cdecl(person);
# montadores cdecl sem argumentos; tratadores cdecl(item) -> eax; a ordem do elenco cdecl (qsort). Chamadas:
# 0x786b70 News::find_item_by_id thiscall ret 0xc; 0x77bbe0 NewsItem::NewsItem thiscall ret 8; 0x786d30
# News::set_item_slot thiscall ret 0x14; 0x41c790 background_set_staff_picture cdecl (3); 0x867700
# STAFF_CONTRACT::apply_event_flags thiscall ret 8; 0x8442d0 remove_from_loan_list thiscall ret 4; 0x5fd930
# HUMAN_MANAGER::get_contract_offer thiscall ret 4; 0x8e56a0 TRANSFER_OFFER::TRANSFER_OFFER thiscall ret 0x14;
# 0x8e5710 ~TRANSFER_OFFER thiscall; 0x8f2470 transfer_screens_open_offer_changed_screen cdecl (2); 0x7c1e30
# PlayerStats::get_stat thiscall ret 0x10 (float 6,5 na pilha).
# Rotinas inteiras iguais no cm0102-1080.exe.
# Defeitos do original (mantidos): no "Não" do pedido de transferência, a chance de o jogador retirar o pedido usa a
# lealdade, o profissionalismo e o temperamento da pessoa do parâmetro 0 (o técnico), não do jogador; no "Sim", a
# saída da lista de empréstimo testa e tira o técnico, não o jogador; a moral do contrato soma o byte (lealdade × 5 +
# 0x38), que estoura como byte (lealdade 15 ou mais vira um número negativo); na oferta a um membro da comissão, a
# tela "oferta mudou" recebe o ponteiro da oferta que acabou de ser liberada.
TELAS = True
ROTINAS = [('contract_screens_news_transfer_request_respond_action', 0x4dca40, [0x6a, 0xff, 0x68, 0xbb, 0x70, 0x95, 0x00]),
           ('contract_screens_build_transfer_request_screen', 0x4dcb30, [0xb8, 0x7c, 0x17, 0x00, 0x00]),
           ('contract_screens_handle_transfer_request_screen', 0x4dcd70, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('contract_screens_handle_staff_offer_contract_screen', 0x4e4ea0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('contract_screens_open_terminate_contract_screen', 0x4e53e0, [0x6a, 0x00, 0x6a, 0x00, 0x6a, 0x01]),
           ('contract_screens_build_terminate_contract_screen', 0x4e5420, [0x81, 0xec, 0x38, 0x01, 0x00, 0x00]),
           ('contract_screens_handle_terminate_contract_screen', 0x4e5610, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00]),
           ('compare_squad_players_by_appearances_then_reputation', 0x4e5850, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08])]
DOCS = ('as telas de contrato: o pedido de transferência, a oferta de contrato à comissão, rescindir contrato e a ordem do elenco',
        'the contract screens: the transfer request, the staff contract offer, terminating a contract and the squad order',
        [('contract_screens_news_transfer_request_respond_action', 'contract_screens_news_transfer_request_respond_action', '0x4dca40', 'contract_screens.cpp', 'cdecl',
          'responder ao pedido de transferência pela notícia abre a tela "Transfer Request" com o técnico, a notícia, o clube e o jogador',
          'answering the transfer request from the news opens the "Transfer Request" screen with the manager, the news, the club and the player'),
         ('contract_screens_build_transfer_request_screen', 'contract_screens_build_transfer_request_screen', '0x4dcb30', 'contract_screens.cpp', 'cdecl',
          'a tela pergunta se põe o jogador na lista de transferências a pedido dele, com Voltar, Não e Sim',
          'the screen asks whether to transfer list the player at his own request, with Back, No and Yes'),
         ('contract_screens_handle_transfer_request_screen', 'contract_screens_handle_transfer_request_screen', '0x4dcd70', 'contract_screens.cpp', 'cdecl',
          'Voltar fecha; Não recusa (a notícia respondida, o jogador pode retirar o pedido, a moral do contrato muda pela lealdade e a moral cai 12); Sim põe na lista a pedido dele (sai da lista de empréstimo, moral +12, entra nas listas dos clubes, notícia da lista)',
          'Back closes; No refuses (the news answered, the player may withdraw the request, the contract morale moves by his loyalty and his morale drops 12); Yes lists him at his request (off the loan list, morale +12, onto the clubs\' shortlists, the listing news)'),
         ('contract_screens_handle_staff_offer_contract_screen', 'contract_screens_handle_staff_offer_contract_screen', '0x4e4ea0', 'contract_screens.cpp', 'cdecl',
          'mandar a oferta de contrato a um membro da comissão: oferta de transferência que mudou é descartada (tela "oferta mudou"); pessoa de outro clube ganha antes uma oferta de transferência (com a compensação do jogador ou a data do fim do contrato); depois a oferta de contrato entra e a tela fecha',
          'sending a contract offer to a member of staff: a transfer offer that changed is dropped ("offer changed" screen); a person of another club first gets a transfer offer (with the player\'s compensation or his contract end date); then the contract offer goes in and the screen closes'),
         ('contract_screens_open_terminate_contract_screen', 'contract_screens_open_terminate_contract_screen', '0x4e53e0', 'contract_screens.cpp', 'cdecl',
          'rescindir o contrato de alguém abre a pergunta com a pessoa no parâmetro 0',
          'terminating someone\'s contract opens the question with the person in parameter 0'),
         ('contract_screens_build_terminate_contract_screen', 'contract_screens_build_terminate_contract_screen', '0x4e5420', 'contract_screens.cpp', 'cdecl',
          'a tela de rescisão: foto e nome da pessoa nas cores do clube, "Terminate Contract", a pergunta centrada e Não / Sim',
          'the terminate contract screen: the person\'s picture and name in the club\'s colours, "Terminate Contract", the question centred and No / Yes'),
         ('contract_screens_handle_terminate_contract_screen', 'contract_screens_handle_terminate_contract_screen', '0x4e5610', 'contract_screens.cpp', 'cdecl',
          'Sim rescinde: o contrato sai, a pessoa deixa o clube e fica livre; olheiro de clube humano atualiza as listas de observação e qualquer saída de clube humano as atribuições dos treinadores; Não fecha',
          'Yes terminates: the contract goes, the person leaves the club and becomes free; a human club\'s scout refreshes the scouting lists and any departure from a human club the coaching assignments; No closes'),
         ('compare_squad_players_by_appearances_then_reputation', 'compare_squad_players_by_appearances_then_reputation', '0x4e5850', 'contract_screens.cpp', 'cdecl',
          'a ordem do elenco: mais jogos (estatística 6) primeiro e depois maior reputação local; sem ficha de jogador conta 0',
          'the squad order: most appearances (statistic 6) first, then the highest home reputation; no player sheet counts 0')])
