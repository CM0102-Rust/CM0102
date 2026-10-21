ROTINAS = [('fechar_propostas_da_oferta', 0x8c6db0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('enfileirar_transferencia_livre', 0x8c16b0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('negociacoes_de_troca_do_jogador', 0x8c5fc0, [0x83, 0xec, 0x0c, 0x8b, 0x44, 0x24, 0x10])]
DOCS = ('o fechamento das propostas de contrato da oferta, a fila da transferência livre e as negociações de troca do jogador',
        'closing the contract offers of a transfer offer, the free transfer queue and the player exchange negotiations',
        [('fechar_propostas_da_oferta', 'transfer_manager_close_offer_contract_offers', '0x8c6db0', 'transfer_manager.cpp', 'stdcall',
          'fecha (estado 4) as propostas de contrato ligadas à oferta: a do jogador-alvo e as dos jogadores da troca',
          'closes (state 4) the contract offers linked to the transfer offer: the target player and the exchange players'),
         ('enfileirar_transferencia_livre', 'TRANSFER_MANAGER::queue_free_transfer', '0x8c16b0', 'transfer_manager.cpp', 'thiscall',
          'põe o jogador na fila da transferência livre (id, motivo e compensação), recusando o emprestado',
          'queues the player for a free transfer (id, reason and compensation), refusing players out on loan'),
         ('negociacoes_de_troca_do_jogador', 'TRANSFER_MANAGER::get_exchange_negotiations_for_player', '0x8c5fc0', 'transfer_manager.cpp', 'thiscall',
          'conta as negociações abertas do clube do jogador que o incluem na troca e, se pedido, lista os índices das ofertas',
          'counts the open negotiations of the player\'s club that include him in a part exchange and, if asked, lists the offer indices')])
