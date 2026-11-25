# As notícias do jogador que sai da troca, da transferência parada pelo prazo da nação e do empréstimo aceito
# (transfer_manager.cpp; leitor rx05, rodada 6, 25/11/2026). As três inteiras (0x8de150..0x8de30e,
# 0x8e1150..0x8e1369, 0x8e1f10..0x8e2090), iguais no cm0102-1080.exe; contagem 0 na carreira ENG e no mundo até
# 2002-08-01: só rodam com um humano na oferta.
# Usa os auxiliares do rx05_1 (integrar junto com ele ou depois).
# Convenções conferidas na desmontagem e nos pontos de chamada (todas thiscall com quadro SEH sem destrutor no caminho
# normal):
# - 0x8de150 thiscall(this, oferta, pessoa, valor, motivo) ret 0x10; chamada por 0x8c41c0 e 0x8c6ae0 (em Rust). O
#   contrato do alvo é procurado antes do teste do humano (0x7875e0 thiscall([0xb7431c], comprador) ret 4).
# - 0x8e1150 thiscall(this, oferta, nação) ret 8; chamada por 0x8c4e10 (em Rust). 0x8d1130 com 0xf; 0x8e7a70 com
#   (0, 0, 0, 1); o clube do contrato procurado duas vezes (−1 sem contrato, tipo 5 ou clube nulo).
# - 0x8e1f10 thiscall(this, oferta, termos) ret 8; chamada por 0x8c2180 (em Rust). 0x8d1130 com 0x5f; a vaga 6 da
#   notícia fica sem valor.
# Comparadores: só a original (a notícia vai às caixas dos técnicos e aloca).
SAVE = False
ROTINAS = [('post_exchange_player_removed_news', 0x8de150, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('post_offer_on_hold_news', 0x8e1150, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('post_loan_accepted_news', 0x8e1f10, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as notícias do jogador que sai da troca, da transferência parada pelo prazo da nação e do empréstimo aceito',
        'the news of the player who leaves a part exchange, of the transfer held by a national deadline and of the accepted loan',
        [('post_exchange_player_removed_news', 'TRANSFER_MANAGER::post_exchange_player_removed_news', '0x8de150', 'transfer_manager.cpp', 'thiscall, ret 0x10',
          'a notícia 0xfad de que um jogador saiu da troca: o clube vendedor, o comprador, o alvo, o jogador da troca, o valor, o valor da oferta e o motivo; só vai ao comprador quando um humano o dirige',
          'news 0xfad that a player left the part exchange: the selling club, the buyer, the target, the exchange player, the value, the offer fee and the reason; it only goes to the buyer when a human manages it'),
         ('post_offer_on_hold_news', 'TRANSFER_MANAGER::post_offer_on_hold_news', '0x8e1150', 'transfer_manager.cpp', 'thiscall, ret 8',
          'a notícia 0xfb4 da transferência parada pelo fim da janela de uma nação: o jogador, o comprador, o clube do contrato (ou −1), o valor, o pré-contrato, o valor total, a nação, o estado, a vaga, o número de série e o tipo da oferta; vai aos destinatários da oferta (nenhum, nada)',
          "news 0xfb4 of the transfer held by a nation's window closing: the player, the buyer, the contract club (or −1), the fee, the pre-contract flag, the total value, the nation, the status, slot, serial and type of the offer; it goes to the offer's recipients (none, nothing)"),
         ('post_loan_accepted_news', 'TRANSFER_MANAGER::post_loan_accepted_news', '0x8e1f10', 'transfer_manager.cpp', 'thiscall, ret 8',
          'a notícia 0xfb6 de que o jogador aceitou o empréstimo até uma data: o jogador, o comprador, o clube do contrato e a data final do empréstimo; vai aos destinatários da oferta (nenhum, nada)',
          "news 0xfb6 that the player accepted the loan until a date: the player, the buyer, the contract club and the loan's end date; it goes to the offer's recipients (none, nothing)")])
