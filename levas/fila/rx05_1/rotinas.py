# As notícias das negociações e do jogador da troca que recusa (transfer_manager.cpp; leitor rx05, rodada 6,
# 25/11/2026). As três inteiras (0x8d6bf0..0x8d6e76, 0x8e4610..0x8e4792, 0x8d8910..0x8d8ae7), iguais no
# cm0102-1080.exe; contagem 0 na carreira ENG e no mundo até 2002-08-01 (docs/cobertura_mundo.txt): só rodam com um
# humano no meio da negociação.
# Convenções conferidas na desmontagem e nos pontos de chamada:
# - 0x8d6bf0 thiscall(this, negociação, oferta) ret 8 (quadro SEH sem destrutor no caminho normal); chamada por
#   0x8be9d0 (add_transfer_negotiation, em Rust: exe::transfer_manager_post_offer_negotiation_news(this, neg, oferta)).
#   Dentro: 0x8d1130 thiscall(this, oferta, 1, lista, 0x1e) ret 0x10; 0x77bbe0 NewsItem::NewsItem(0xfa2, 0) ret 8;
#   0x785790 set_slot ret 8; 0x4cf5a0 get_contract ret 4 (o resultado em [0xae1080]); 0x8e7a70 thiscall(neg, 0, 1, 0,
#   1) ret 0x10; 0x68d1b0 LLIST::get_next ret 0; 0x7861e0 thiscall([0xb7431c], &notícia, clube) ret 8.
# - 0x8e4610 thiscall(this, negociação, oferta) ret 8; a pessoa vem da oferta e o comprador da negociação; vai ao
#   clube comprador da oferta.
# - 0x8d8910 thiscall(this, oferta, pessoa, motivo) ret 0xc; chamada por 0x8c0080 (accept_transfer_bid, em Rust);
#   0x8d1130 com 3 e no máximo 2 destinatários.
# Os três auxiliares (news_contract_club_id_or_none, news_contract_club_id, send_news_to_clubs) são usados pelos
# itens rx05_2..rx05_4.
# Comparadores: só a original (a notícia vai às caixas dos técnicos e aloca).
SAVE = False
ROTINAS = [('post_offer_negotiation_news', 0x8d6bf0, [0x6a, 0xff, 0x68, 0x6b, 0x43, 0x96, 0x00]),
           ('post_loan_negotiation_news', 0x8e4610, [0x6a, 0xff, 0x68, 0x8b, 0x48, 0x96, 0x00]),
           ('post_swap_player_rejects_news', 0x8d8910, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as notícias das negociações de transferência e de empréstimo e do jogador da troca que recusa',
        'the news of the transfer and loan negotiations and of the exchange player who refuses',
        [('post_offer_negotiation_news', 'TRANSFER_MANAGER::post_offer_negotiation_news', '0x8d6bf0', 'transfer_manager.cpp', 'thiscall, ret 8',
          'a notícia 0xfa2 da contraproposta do clube vendedor: o jogador, o comprador, o clube do contrato dele, as vagas e os números de série da oferta e da negociação, o valor pedido, o estado da oferta, o valor total da negociação e até 3 jogadores da troca (com a quantidade de todos); vai aos destinatários da oferta',
          "news 0xfa2 of the selling club's counter demand: the player, the buyer, his contract club, the slots and serials of the offer and of the negotiation, the fee asked, the offer status, the negotiation total value and up to 3 exchange players (with the count of all); it goes to the offer's recipients"),
         ('post_loan_negotiation_news', 'TRANSFER_MANAGER::post_loan_negotiation_news', '0x8e4610', 'transfer_manager.cpp', 'thiscall, ret 8',
          'a notícia 0xfc0 de que o clube do jogador quer negociar o empréstimo: o jogador, o comprador, o clube do contrato, as vagas e os números de série da oferta e da negociação e o estado da oferta; vai ao clube comprador',
          "news 0xfc0 that the player's club wants to negotiate the loan: the player, the buyer, the contract club, the slots and serials of the offer and the negotiation and the offer status; it goes to the buying club"),
         ('post_swap_player_rejects_news', 'TRANSFER_MANAGER::post_swap_player_rejects_news', '0x8d8910', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'a notícia 0xfaf de que o jogador oferecido na troca recusou a mudança: o clube vendedor, o comprador, o alvo, o jogador da troca, o motivo e o valor da oferta; vai a até 2 destinatários da oferta (nenhum, nada)',
          "news 0xfaf that the player offered in exchange refused the move: the selling club, the buyer, the target, the exchange player, the reason and the offer fee; it goes to up to 2 of the offer's recipients (none, nothing)")])
