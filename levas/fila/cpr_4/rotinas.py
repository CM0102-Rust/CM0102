# As ofertas abertas e as desfeitas (transfer_manager.cpp): as ofertas pelos jogadores do clube
# (TRANSFER_MANAGER::get_open_offers_for_club_players 0x8c6100, pela lista das transferências futuras do clube 0x8ccb80,
# que a verba de transferências 0x598800 chama), desfazer as ofertas pelo jogador (TRANSFER_MANAGER::
# collapse_offers_for_player 0x8c47d0, pelos jogadores da troca em 0x8c3310 e 0x8c6ae0; quadro SEH da LLIST local,
# construída e destruída como na original) e guardar o percentual da próxima venda (TRANSFER_MANAGER::
# add_next_sale_percentage 0x8caaf0, pelo pagamento da transferência 0x8ca8a0; irmã da 0x8cac40, usa cpl3_crescer).
# Iguais no cm0102-1080.exe. Comparadores: lado a lado sem lista (com lista, a contagem antes e a original); só a
# original conferida pelo plano nas que agendam ou crescem vetores. A irmã 0x8cad90 (prêmio pela seleção) fica para
# outra leva.
SAVE = False
ROTINAS = [('ofertas_abertas_pelos_jogadores_do_clube', 0x8c6100, [0x51, 0x55, 0x56, 0x8b, 0xf1]),
           ('desfazer_ofertas_pelo_jogador', 0x8c47d0, [0x6a, 0xff, 0x68, 0x88, 0x3c, 0x96, 0x00]),
           ('guardar_percentual_da_proxima_venda', 0x8caaf0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as ofertas abertas e as desfeitas: as ofertas pelos jogadores do clube, desfazer as ofertas pelo jogador e guardar o percentual da próxima venda',
        'the open and collapsed offers: the offers for the club\'s players, collapsing the offers for a player and storing the next sale percentage',
        [('ofertas_abertas_pelos_jogadores_do_clube', 'TRANSFER_MANAGER::get_open_offers_for_club_players', '0x8c6100', 'transfer_manager.cpp', 'thiscall, ret 8',
          'conta (e lista os índices de) as ofertas abertas por jogadores com contrato no clube, fora dos estados 2, 3, 4, 0x10, 0x14, das desfeitas e dos empréstimos',
          'counts (and lists the indices of) the open offers for players contracted to the club, except states 2, 3, 4, 0x10, 0x14, collapsed ones and loans'),
         ('desfazer_ofertas_pelo_jogador', 'TRANSFER_MANAGER::collapse_offers_for_player', '0x8c47d0', 'transfer_manager.cpp', 'thiscall, ret 4',
          'cada oferta pelo jogador perde a transferência futura (0x8cbf70), fica desfeita (0xf, +0x2e = 3), solta os lances do comprador e do vendedor (0x84aeb0) e é agendada (0x8c6220, 10 dias)',
          'each offer for the player loses its future transfer (0x8cbf70), is collapsed (0xf, +0x2e = 3), clears the buyer\'s and seller\'s bid markers (0x84aeb0) and is scheduled (0x8c6220, 10 days)'),
         ('guardar_percentual_da_proxima_venda', 'TRANSFER_MANAGER::add_next_sale_percentage', '0x8caaf0', 'transfer_manager.cpp', 'thiscall, ret 4',
          'acrescenta os 13 bytes do percentual da próxima venda ao vetor +0x880 (cresce de 50 em 50) e reordena (0x8e54d0)',
          'appends the 13-byte next sale percentage to the +0x880 array (growing by 50) and sorts it (0x8e54d0)')])
CAMPOS = [('oferta de transferência +4: a lista da troca; +8: o comprador; +0xc: o jogador; +0x2c: o estado; +0x2e: o motivo do fim (3 = desfeita); id e valor; lê e grava +0x2c/+0x2e (desfazer_ofertas_pelo_jogador)',
           'transfer offer +4: the exchange list; +8: the buyer; +0xc: the player; +0x2c: the state; +0x2e: the end reason (3 = collapsed); id and value; reads and writes +0x2c/+0x2e (desfazer_ofertas_pelo_jogador)'),
          ('contrato +4: o clube; +0x35 & 0x3f: o tipo (5 = empréstimo); id e valor; lê (ofertas_abertas_pelos_jogadores_do_clube, desfazer_ofertas_pelo_jogador)',
           'contract +4: the club; +0x35 & 0x3f: the type (5 = loan); id and value; reads (ofertas_abertas_pelos_jogadores_do_clube, desfazer_ofertas_pelo_jogador)'),
          ('[0xae1080] o último contrato achado; ponteiro; grava (ofertas_abertas_pelos_jogadores_do_clube, desfazer_ofertas_pelo_jogador)',
           '[0xae1080] the last contract found; pointer; writes (ofertas_abertas_pelos_jogadores_do_clube, desfazer_ofertas_pelo_jogador)'),
          ('TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa, byte +1 (ofertas pela pessoa); valor; lê (desfazer_ofertas_pelo_jogador)',
           'TRANSFER_MANAGER +0x86e: person\'s 8-byte record, byte +1 (offers for the person); value; reads (desfazer_ofertas_pelo_jogador)'),
          ('TRANSFER_MANAGER +0x880 / +0x884 / +0x888: os percentuais da próxima venda (13 bytes), a capacidade e a conta; ponteiro e valor; grava (guardar_percentual_da_proxima_venda)',
           'TRANSFER_MANAGER +0x880 / +0x884 / +0x888: the next sale percentages (13 bytes), the capacity and the count; pointer and value; writes (guardar_percentual_da_proxima_venda)')]
INTERESSA = ['CMRTE', 'SGE']
