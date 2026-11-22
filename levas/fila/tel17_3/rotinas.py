# The transfer news buttons and the delay request dialog (transfer_screens.cpp, part 1; reader tel17, the screens round,
# 21/11/2026). Equal in cm0102-1080.exe. Needs tel17_2 (ts_* helpers) and tel26_1.
# Conventions checked in the disassembly and at the call sites:
# - 0x8f1730, 0x8f1830, 0x8f1950, 0x8f1a60, 0x8f1b40, 0x8f1c30: news button actions cdecl(news id a, news id b) ret
#   0, SEH frame (a NewsItem of 0xe0 bytes built empty, NewsItem::NewsItem(0, 0), then News::find_item_by_id 0x786b70
#   thiscall on [G_NEWS], 3 args); 0x8f1c30 asserts at line 0x237f without the buyer's contract offer.
# - 0x8f2010 build_delay_request_dialog cdecl() ret 0; 0x8f22c0 handle_delay_request_action cdecl(item) -> int
#   (News::set_item_slot 0x786d30 thiscall, 5 args).
# Comparers: the news buttons only the original (they open screens); the delay dialog side by side over the display
# list and the contract lookup global; its answer: Back side by side, Accept and Reject only the original (news item,
# offer withdrawn or rejected with reason 6).
TELAS = True
ROTINAS = [('transfer_screens_news_respond_bid_action', 0x8f1730, [0x6a, 0xff, 0x68, 0x9b, 0x4c, 0x96, 0x00]),
           ('transfer_screens_news_respond_counter_offer_action', 0x8f1830, [0x6a, 0xff, 0x68, 0xbb, 0x4c, 0x96, 0x00]),
           ('transfer_screens_news_offer_contract_action', 0x8f1950, [0x6a, 0xff, 0x68, 0xdb, 0x4c, 0x96, 0x00]),
           ('transfer_screens_news_respond_delay_request_action', 0x8f1a60, [0x6a, 0xff, 0x68, 0xfb, 0x4c, 0x96, 0x00]),
           ('transfer_screens_news_renegotiate_bid_action', 0x8f1b40, [0x6a, 0xff, 0x68, 0x1b, 0x4d, 0x96, 0x00]),
           ('transfer_screens_news_respond_loan_bid_action', 0x8f1c30, [0x6a, 0xff, 0x68, 0x3b, 0x4d, 0x96, 0x00]),
           ('transfer_screens_build_delay_request_dialog', 0x8f2010, [0x81, 0xec, 0x3c, 0x01, 0x00, 0x00]),
           ('transfer_screens_handle_delay_request_action', 0x8f22c0, [0x53, 0x55, 0x56, 0x57, 0x6a, 0x00])]
_F = 'transfer_screens.cpp'
DOCS = ('os botões das notícias de transferência e a janela de adiamento', 'the transfer news buttons and the delay request dialog',
        [('transfer_screens_news_respond_bid_action', 'transfer_screens_news_respond_bid_action', '0x8f1730', _F, 'cdecl',
          'Responder a uma proposta recebida: com a oferta ainda no mesmo número de série e não retirada, abre a tela da proposta para ela; senão a tela de oferta mudada',
          'Respond to a received bid: with the offer still at the same serial and not withdrawn, opens the bid screen for it; otherwise the offer changed screen'),
         ('transfer_screens_news_respond_counter_offer_action', 'transfer_screens_news_respond_counter_offer_action', '0x8f1830', _F, 'cdecl',
          'Responder a uma contraproposta: com a oferta igual, não retirada nem recusada, abre a tela da proposta para o jogador com a oferta; senão a tela de oferta mudada',
          'Respond to a counter offer: with the offer unchanged, neither withdrawn nor rejected, opens the bid screen for the player with the offer; otherwise the offer changed screen'),
         ('transfer_screens_news_offer_contract_action', 'transfer_screens_news_offer_contract_action', '0x8f1950', _F, 'cdecl',
          'Oferecer contrato depois de proposta aceita: com a oferta igual e não retirada, abre a negociação do contrato com o jogador; senão a tela de oferta mudada',
          'Offer Contract after an accepted bid: with the offer unchanged and not withdrawn, opens the contract talks with the player; otherwise the offer changed screen'),
         ('transfer_screens_news_respond_delay_request_action', 'transfer_screens_news_respond_delay_request_action', '0x8f1a60', _F, 'cdecl',
          'Responder a um pedido de adiamento: abre a janela de adiamento de uma semana com a notícia e a oferta',
          'Respond to a delay request: opens the one week delay dialog with the news item and the offer'),
         ('transfer_screens_news_renegotiate_bid_action', 'transfer_screens_news_renegotiate_bid_action', '0x8f1b40', _F, 'cdecl',
          'Renegociar: abre a tela da proposta para o jogador com a oferta viva ou, se ela mudou, com uma proposta nova',
          'Re-Negotiate: opens the bid screen for the player with the live offer or, when it changed, with a fresh bid'),
         ('transfer_screens_news_respond_loan_bid_action', 'transfer_screens_news_respond_loan_bid_action', '0x8f1c30', _F, 'cdecl',
          'Responder a uma proposta de empréstimo: abre a tela do empréstimo com o salário do contrato, o período, a parte do salário (arredondada a 10%), o mês e o ano do fim e as cláusulas da oferta de contrato do comprador; sem ela, a caixa de erro (linha 0x237f); com a oferta mudada, a tela de oferta mudada',
          "Respond to a loan bid: opens the loan bid screen with the contract's wage, the period, the wage share (rounded to 10%), the end month and year and the clauses of the buyer's contract offer; without it, the error box (line 0x237f); with the offer changed, the offer changed screen"),
         ('transfer_screens_build_delay_request_dialog', 'transfer_screens_build_delay_request_dialog', '0x8f2010', _F, 'cdecl',
          'monta a janela que pergunta se aceita adiar uma semana a transferência entre o clube vendedor e o comprador, com Voltar, Recusar e Aceitar',
          'builds the dialog that asks whether to accept a one week delay of the transfer between the selling and the buying club, with Back, Reject and Accept'),
         ('transfer_screens_handle_delay_request_action', 'transfer_screens_handle_delay_request_action', '0x8f22c0', _F, 'cdecl',
          'responde à janela de adiamento: Aceitar marca a notícia respondida e aceita; Recusar marca a notícia e retira (status 0x11) ou recusa a oferta com o motivo 6; Voltar -9',
          'answers the delay dialog: Accept marks the news item answered and accepted; Reject marks the news item and withdraws (status 0x11) or rejects the offer with reason 6; Back -9')])
