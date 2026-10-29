# As ofertas pelo jogador (transfer_manager.cpp): se o clube tem oferta por ele
# (TRANSFER_MANAGER::club_has_offer_for_player 0x8cdc90, ~10 mil chamadas na carreira), a oferta do clube por ele
# (TRANSFER_MANAGER::find_club_offer_for_player 0x8cdd80, ~30) e se ele tem oferta pendente
# (TRANSFER_MANAGER::player_has_pending_offer 0x8cd150, ~2,6 mil). As três montam uma lista local de índices (0x68d020;
# a 0x8c5cf0, já em Rust, enche; 0x68d040 destrói), por isso o quadro SEH; a 0x8cde70 (sob oferta) segue original
# (vem na cpl_2). Sem sorteio, sem caixa. Comparadores: as duas rodam (a lista nasce e morre nelas) e a devolução e o
# cursor da lista própria da oferta (que a 0x8c5cf0 repõe) são conferidos.
SAVE = False
ROTINAS = [('clube_tem_oferta_pelo_jogador', 0x8cdc90, [0x6a, 0xff, 0x68, 0x18, 0x3f, 0x96, 0x00]),
           ('oferta_do_clube_pelo_jogador', 0x8cdd80, [0x6a, 0xff, 0x68, 0x38, 0x3f, 0x96, 0x00]),
           ('jogador_com_oferta_pendente', 0x8cd150, [0x6a, 0xff, 0x68, 0xd8, 0x3e, 0x96, 0x00])]
DOCS = ('as ofertas pelo jogador: se o clube tem oferta por ele, a oferta do clube e se ele tem oferta pendente',
        'offers for a player: whether a club has an offer for him, the club offer and whether he has a pending offer',
        [('clube_tem_oferta_pelo_jogador', 'TRANSFER_MANAGER::club_has_offer_for_player', '0x8cdc90', 'transfer_manager.cpp', 'thiscall, ret 8',
          '1 se a pessoa marcada como sob oferta (bit 0 de [+0x86e] + id × 8) tem, entre as ofertas em aberto (0x8c5cf0), uma do clube (+8 da oferta); senão 0',
          '1 when the person flagged as under offer (bit 0 of [+0x86e] + id × 8) has, among the open offers (0x8c5cf0), one from the club (offer +8); else 0'),
         ('oferta_do_clube_pelo_jogador', 'TRANSFER_MANAGER::find_club_offer_for_player', '0x8cdd80', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'a oferta do clube pela pessoa (com `todas`, também as encerradas e sem exigir a marca), ou 0',
          'the club offer for the person (with `all`, closed ones too and without requiring the flag), or 0'),
         ('jogador_com_oferta_pendente', 'TRANSFER_MANAGER::player_has_pending_offer', '0x8cd150', 'transfer_manager.cpp', 'thiscall, ret 4',
          '1 se a pessoa está sob oferta (0x8cde70) e uma oferta em aberto está na situação (+0x2c) 0, 1, 5 ou 6; senão 0',
          '1 when the person is under offer (0x8cde70) and an open offer is in state (+0x2c) 0, 1, 5 or 6; else 0')])
CAMPOS = [('gerente de transferências +0x86e: tabela de 8 bytes por id de pessoa (bit 0 = sob oferta); valor; lê (clube_tem_oferta_pelo_jogador, oferta_do_clube_pelo_jogador)',
           'transfer manager +0x86e: 8-byte table by person id (bit 0 = under offer); value; reads (clube_tem_oferta_pelo_jogador, oferta_do_clube_pelo_jogador)'),
          ('gerente de transferências +0/+4: vetor de ponteiros das ofertas e a conta; ponteiro; lê (as três)',
           'transfer manager +0/+4: offer pointer vector and count; pointer; reads (all three)'),
          ('oferta +8: id do clube que oferece; id; lê (clube_tem_oferta_pelo_jogador, oferta_do_clube_pelo_jogador)',
           'offer +8: offering club id; id; reads (clube_tem_oferta_pelo_jogador, oferta_do_clube_pelo_jogador)'),
          ('oferta +0x2c: situação da oferta (0, 1, 5, 6 = pendente); valor; lê (jogador_com_oferta_pendente)',
           'offer +0x2c: offer state (0, 1, 5, 6 = pending); value; reads (jogador_com_oferta_pendente)'),
          ('[0xae2364] número de pessoas; valor; lê', '[0xae2364] number of people; value; reads')]
INTERESSA = ['CMRTE']
