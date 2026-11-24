# O jogador dispensado e a resposta do jogador a um contrato (transfer_manager.cpp; leitor sx02, rodada 5,
# 23/11/2026).
# - 0x8df9b0 (TRANSFER_MANAGER::cancel_offers_for_released_player): 6 chamadas ate 2002-06-01 na carreira ENG
#   (dispensas); thiscall(TRANSFER_MANAGER, jogador), ret 4. Chama get_offers_for_player (0x8c5cf0: push 0; push lista;
#   push jogador), remove_future_transfer, schedule_offer (push 0; push 0xa; push oferta), a 0x8c6db0 (push oferta,
#   ecx = this), remove_contract_offer (0x4cca70: push clube; push pessoa, ecx = G_CONTRACT_MANAGER),
#   remove_contract_offer_for_transfer (0x4ccb30: push vaga; push pessoa), LLIST::clear (0x68d080),
#   get_exchange_offers_for_player (0x8c5e40: push lista; push jogador), 0x8e81a0 (TRANSFER_OFFER, tira o jogador da
#   troca: thiscall(oferta, jogador), ret 4) e collect_player_news_recipients (push 0x14; push vetor; push 0xa; push
#   jogador). As tres noticias 0xfb1 (0xc = 0, 1, 2) conferidas push a push.
# - 0x8d8e90 (TRANSFER_MANAGER::post_contract_response_news): 986 chamadas; thiscall(this, pessoa, clube, outro clube,
#   resposta (byte), oferta, contrato, lista dos recusados (LLIST de ids), valor), ret 0x20 (o invólucro
#   exe::transfer_manager_post_contract_response_news e as chamadas de 0x8c2180); 0x846920
#   (staff_interest_in_joining_club): push &bandeiras; push status (nibble alto de +0x4f); push emprestimo; push 0; push
#   clube; push pessoa, devolve st0 guardado em float e x 1000.0 pelo _ftol; 0x4cc9e0 (find_contract_offer):
#   thiscall(CM, id da pessoa, id do clube).
# Comparadores: so a original nas duas (agendam, desfazem ofertas de contrato e postam noticias, que nao rodam duas
# vezes); a copia leva os ramos pelo covered!.
SAVE = False
ROTINAS = [('cancel_offers_for_released_player', 0x8df9b0, [0x6a, 0xff, 0x68, 0x5c, 0x46, 0x96, 0x00]),
           ('post_contract_response_news', 0x8d8e90, [0x6a, 0xff, 0x68, 0x4b, 0x44, 0x96, 0x00])]
DOCS = ('o jogador dispensado e a resposta do jogador a um contrato',
        "the released player and a player's answer to a contract",
        [('cancel_offers_for_released_player', 'TRANSFER_MANAGER::cancel_offers_for_released_player', '0x8df9b0', 'transfer_manager.cpp', 'thiscall, ret 4',
          'regra: cada oferta pelo jogador dispensado é cancelada (a transferência futura perde o registro), volta em 10 dias e perde as ofertas de contrato (do alvo e dos jogadores da troca); o comprador humano recebe a notícia 0xfb1; cada oferta em que ele é jogador de troca avisa o vendedor humano (taxa e valor do jogador + taxa) e ele sai da troca; os clubes humanos interessados nele que ainda não souberam recebem a notícia',
          "rule: each offer for the released player is cancelled (a future transfer loses its record), is due again in 10 days and loses its contract offers (the target's and the exchange players'); the human buyer gets news 0xfb1; each offer where he is an exchange player tells the human seller (the fee and the player's value + the fee) and he leaves the exchange; the human clubs interested in him not told yet get the news"),
         ('post_contract_response_news', 'TRANSFER_MANAGER::post_contract_response_news', '0x8d8e90', 'transfer_manager.cpp', 'thiscall, ret 0x20',
          'regra: a notícia 0xfa4 da resposta do jogador a um contrato vai aos clubes humanos da oferta (bandeiras 0x2f; com contrato 0x2d; na resposta 3 sem contrato, 0x2d se o clube é o comprador, senão 0x2e); na recusa de todos (resposta 2), também aos até três clubes recusados mais conhecidos (os três primeiros; depois cada um substitui o de menor reputação quando é maior); a notícia leva a pessoa, os clubes, a resposta, a oferta, o contrato, os técnicos dos dois clubes, o interesse do jogador em ir para o clube (× 1000) e os clubes recusados',
          "rule: news 0xfa4 of a player's answer to a contract goes to the human clubs of the offer (flags 0x2f; with a contract 0x2d; for answer 3 without one, 0x2d if the club is the buyer, otherwise 0x2e); on refusing them all (answer 2), also to up to three best-known refused clubs (the first three; then each one replaces the lowest-reputation one when higher); the news carries the person, the clubs, the answer, the offer, the contract, both clubs' managers, the player's interest in joining the club (× 1000) and the refused clubs")])
CAMPOS = []
INTERESSA = []
