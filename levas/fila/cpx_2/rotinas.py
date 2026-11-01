# O jogador desassossegado pelo lance de um clube maior, a noticia da oferta desfeita e a do premio por jogos
# (transfer_manager.cpp): o desassossego (TRANSFER_MANAGER::unsettle_player_after_bid 0x8c1290, 44 chamadas ate
# 2002-06-01 na cobertura de 17/10: o sorteio contra a reputacao do comprador marca +0x2d da oferta; com a marca, a nota
# do jogador pelos atributos e o interesse leva a queixa 0x80 no contrato, a fuga e as antipatias), a noticia 0xfb0 da
# oferta desfeita (post_offer_collapsed_news 0x8de790) e a 0xfa6 do premio por jogos pago (post_appearance_money_news
# 0x8db8a0, pela 0x8cb340). Chamadas originais pelo endereco: 0x846920 (o interesse), 0x945ff0 (a pow do jogo), 0x4d0c60
# (o valor), 0x4d1600 (o preco), 0x8e7b50, 0x867700, 0x418f60, 0x419840, 0x547af0, 0x8d1130, 0x77bbe0; as outras ja em
# Rust (0x4cf5a0, 0x849f00, 0x866f60, 0x547e70, 0x90cfc0, 0x785790, 0x7861e0). Iguais no cm0102-1080.exe.
# Comparadores: o desassossego pelo ensaio (a copia para antes da queixa; sem chegar la, lado a lado na oferta, em
# [0xae1080] e no sorteio; chegando, so a original); as noticias so a original (alocam).
SAVE = False
ROTINAS = [('desassossegar_pelo_lance', 0x8c1290, [0x83, 0xec, 0x20, 0x53, 0x8b, 0x5c, 0x24, 0x28]),
           ('noticia_oferta_desfeita', 0x8de790, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_premio_por_jogos', 0x8db8a0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o jogador desassossegado pelo lance de um clube maior, a notícia da oferta desfeita e a do prêmio por jogos',
        'a player unsettled by a bigger club\'s bid, the news of a collapsed offer and of appearance money paid',
        [('desassossegar_pelo_lance', 'unsettle_player_after_bid', '0x8c1290', 'transfer_manager.cpp', 'thiscall',
          'o sorteio contra a reputação do comprador marca o jogador desassossegado; com a marca, a nota pelos atributos e o interesse leva à queixa 0x80, à fuga e às antipatias',
          'a draw against the buyer\'s reputation flags the player as unsettled; when flagged, a score from his attributes and interest leads to the 0x80 complaint, going missing and grudges'),
         ('noticia_oferta_desfeita', 'post_offer_collapsed_news', '0x8de790', 'transfer_manager.cpp', 'thiscall',
          'a notícia 0xfb0 da oferta desfeita, com o clube e o motivo, aos clubes humanos interessados',
          'the 0xfb0 news of a collapsed offer, with the club and the reason, to the interested human clubs'),
         ('noticia_premio_por_jogos', 'post_appearance_money_news', '0x8db8a0', 'transfer_manager.cpp', 'thiscall',
          'a notícia 0xfa6 do prêmio por jogos pago, ao clube que paga e ao clube do contrato',
          'the 0xfa6 news of appearance money paid, to the paying club and to the contract club')])
CAMPOS = [('oferta +8 comprador, +0xc jogador; id; lê (desassossegar_pelo_lance, noticia_oferta_desfeita)',
           'offer +8 buyer, +0xc player; id; reads (desassossegar_pelo_lance, noticia_oferta_desfeita)'),
          ('oferta +0x2d: jogador desassossegado; valor; grava 1 e lê (desassossegar_pelo_lance)',
           'offer +0x2d: player unsettled; value; writes 1 and reads (desassossegar_pelo_lance)'),
          ('clube +0x80: reputação; valor; lê (desassossegar_pelo_lance)', 'club +0x80: reputation; value; reads (desassossegar_pelo_lance)'),
          ('pessoa +0x18 idade, +0x57, +0x59, +0x5b, +0x5d: atributos de personalidade; valor; lê (desassossegar_pelo_lance)',
           'person +0x18 age, +0x57, +0x59, +0x5b, +0x5d: personality attributes; value; reads (desassossegar_pelo_lance)'),
          ('ficha +0xb: reputação atual; valor; lê (desassossegar_pelo_lance)', 'player record +0xb: current reputation; value; reads (desassossegar_pelo_lance)'),
          ('contrato +0x4e & 8: listado; +0x44: força do pedido; valor; lê (desassossegar_pelo_lance)',
           'contract +0x4e & 8: transfer listed; +0x44: request strength; value; reads (desassossegar_pelo_lance)'),
          ('gerência de transferências +0x86e: registro de 8 bytes por pessoa, +1; valor; lê (desassossegar_pelo_lance)',
           'transfer manager +0x86e: 8-byte record per person, +1; value; reads (desassossegar_pelo_lance)'),
          ('cláusula +0 pessoa, +4, +9 clube que paga; id; lê (noticia_premio_por_jogos)', 'clause +0 person, +4, +9 paying club; id; reads (noticia_premio_por_jogos)'),
          ('[0xae1080] último contrato consultado; ponteiro; grava (as três)', '[0xae1080] last contract looked up; pointer; writes (all three)')]
INTERESSA = ['CMRTE']
