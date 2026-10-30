# As ofertas de transferência (transfer_manager.cpp): o clube recusa a oferta (TRANSFER_MANAGER::club_rejects_offer
# 0x8c36b0, 7 chamadas: conta a tentativa, um sorteio(2 × tentativas) decide entre rejeitar/retirar e reagendar com a
# notícia de adiamento 0x8dda90, que fica na original), soltar os jogadores da oferta
# (TRANSFER_MANAGER::detach_offer_players 0x8c6ae0, pela 0x8c34e0: tira o jogador e os de troca das outras ofertas;
# a LLIST local que a original constrói e destrói sem usar é reproduzida) e montar a oferta em dinheiro
# (TRANSFER_MANAGER::build_cash_offer_option 0x8c8740, pela 0x8c8090: aloca e inicia a oferta pelo valor, atração 50;
# sem finanças do clube, a original com a caixa "Error"). Iguais no cm0102-1080.exe. Comparadores: as três só a
# original (sorteiam, liberam ou alocam); a 0x8c8740 confere o resultado (1, o valor +0x1b da oferta, atração 50).
SAVE = False
ROTINAS = [('clube_recusa_a_oferta', 0x8c36b0, [0x56, 0x8b, 0xf1, 0x8b, 0x4c, 0x24, 0x08]),
           ('soltar_jogadores_da_oferta', 0x8c6ae0, [0x6a, 0xff, 0x68, 0x18, 0x3d, 0x96, 0x00]),
           ('montar_oferta_em_dinheiro', 0x8c8740, [0x6a, 0xff, 0x68, 0xce, 0x3d, 0x96, 0x00])]
DOCS = ('as ofertas de transferência: o clube recusa a oferta, soltar os jogadores da oferta e montar a oferta em dinheiro',
        'transfer offers: a club rejects the offer, detaching the offer\'s players and building the cash offer',
        [('clube_recusa_a_oferta', 'TRANSFER_MANAGER::club_rejects_offer', '0x8c36b0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'conta a tentativa da oferta; pelo lado que recusa, um sorteio(2 × tentativas) rejeita ou retira a oferta, senão ela é reagendada com a notícia de adiamento',
          'counts the offer\'s attempt; by the refusing side, a draw(2 × attempts) rejects or withdraws the offer, otherwise it is rescheduled with the delay news'),
         ('soltar_jogadores_da_oferta', 'TRANSFER_MANAGER::detach_offer_players', '0x8c6ae0', 'transfer_manager.cpp', 'thiscall, ret 4',
          'tira o jogador da oferta e cada jogador de troca das outras ofertas e desfaz as ofertas por cada jogador de troca',
          'takes the offer\'s player and each exchange player out of other offers and collapses the offers for each exchange player'),
         ('montar_oferta_em_dinheiro', 'TRANSFER_MANAGER::build_cash_offer_option', '0x8c8740', 'transfer_manager.cpp', 'thiscall, ret 0x14',
          'aloca e inicia uma oferta em dinheiro pelo jogador com o valor dado (0x8e78f0), guarda na saída, atração 50 e devolve 1',
          'allocates and initialises a cash offer for the player at the given fee (0x8e78f0), stores it in the output, appeal 50, and returns 1')])
CAMPOS = [('oferta (0x8e5680, 0x41 bytes) +0x2c: o estado (0x11 recusada pelo outro lado, 0x12 pelo vendedor); valor; lê e grava (clube_recusa_a_oferta)',
           'offer (0x8e5680, 0x41 bytes) +0x2c: the state (0x11 refused by the other side, 0x12 by the seller); value; reads and writes (clube_recusa_a_oferta)'),
          ('oferta +0x2e: as tentativas; valor; lê e grava (clube_recusa_a_oferta)',
           'offer +0x2e: the attempts; value; reads and writes (clube_recusa_a_oferta)'),
          ('oferta +8 e +0xc: o id do clube vendedor e o do jogador; id; lê (clube_recusa_a_oferta, soltar_jogadores_da_oferta)',
           'offer +8 and +0xc: the selling club id and the player id; id; reads (clube_recusa_a_oferta, soltar_jogadores_da_oferta)'),
          ('oferta +4: a lista dos jogadores de troca (itens: id); ponteiro; lê (soltar_jogadores_da_oferta)',
           'offer +4: the exchange player list (items: id); pointer; reads (soltar_jogadores_da_oferta)'),
          ('oferta +0x1b: o valor; valor; grava pela 0x8e78f0 (montar_oferta_em_dinheiro)',
           'offer +0x1b: the fee; value; written by 0x8e78f0 (montar_oferta_em_dinheiro)'),
          ('contrato +0x35 (& 0x3f; 5 fora); valor; lê (clube_recusa_a_oferta); [0xae1080] o último contrato; ponteiro; grava',
           'contract +0x35 (& 0x3f; 5 skips); value; reads (clube_recusa_a_oferta); [0xae1080] the last contract; pointer; writes')]
INTERESSA = ['CMRTE']
