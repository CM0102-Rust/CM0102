# As ofertas e o contrato do jogador (transfer_manager.cpp): se ele está sob oferta
# (TRANSFER_MANAGER::is_player_under_offer 0x8cde70, ~4,9 milhões de chamadas na carreira; chamada pela 0x8cd150 da
# cpl_1), se o clube do contrato dele segue a lei Bosman (player_club_uses_bosman 0x8d0a70, ~31 milhões) e o veto da
# diretoria na oferta (TRANSFER_MANAGER::check_board_veto 0x8cdb80, ~1,1 mil). SEH só da lista local (0x68d020/
# 0x68d040, já vista na cpl_1) e da data local da 0x8d0a70 (0x54a110, sem efeito fora dela). A 0x8cde70 e a 0x8d0a70
# gravam o contrato achado em [0xae1080], como a original. Sem sorteio; caixa só com a pessoa nula (a original).
# Comparadores: as duas primeiras rodam lado a lado (leve; quentes); o veto roda só a original (a 0x696b10 mexe na
# confiança da diretoria) e é conferido com o plano.
SAVE = False
ROTINAS = [('jogador_sob_oferta', 0x8cde70, [0x6a, 0xff, 0x68, 0x58, 0x3f, 0x96, 0x00]),
           ('clube_do_jogador_com_lei_bosman', 0x8d0a70, [0x6a, 0xff, 0x68, 0x5b, 0x40, 0x96, 0x00]),
           ('veto_da_diretoria_na_oferta', 0x8cdb80, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('as ofertas e o contrato do jogador: se ele está sob oferta, se o clube dele segue a lei Bosman e o veto da diretoria na oferta',
        'player offers and contract: whether he is under offer, whether his club follows the Bosman rule and the board veto on the offer',
        [('jogador_sob_oferta', 'TRANSFER_MANAGER::is_player_under_offer', '0x8cde70', 'transfer_manager.cpp', 'thiscall, ret 8',
          'sem a visão do humano, a marca "sob oferta" (bit 0 de [+0x86e] + id × 8); com ela, 1 se alguma oferta é conhecida (+0x2d), é de clube do humano da vez ou pelo jogador de clube dele, se a pessoa está na shortlist dele ou se ele tem oferta de contrato aceita pela pessoa',
          'without the human view, the "under offer" flag (bit 0 of [+0x86e] + id × 8); with it, 1 when an offer is known (+0x2d), comes from a club of the current human or is for a player of his club, when the person is on his shortlist or when he has an accepted contract offer for the person'),
         ('clube_do_jogador_com_lei_bosman', 'player_club_uses_bosman', '0x8d0a70', 'transfer_manager.cpp', 'stdcall, ret 4',
          '1 se a nação do clube do contrato da pessoa tem transferência livre no fim do contrato (0x544670); sem contrato, clube ou nação: 1',
          '1 when the nation of the club of the person contract has free transfers at contract expiry (0x544670); without contract, club or nation: 1'),
         ('veto_da_diretoria_na_oferta', 'TRANSFER_MANAGER::check_board_veto', '0x8cdb80', 'transfer_manager.cpp', 'thiscall, ret 0x18',
          'com a parte de troca da oferta vazia, a diretoria do clube que decide responde (0x696b10, pelo valor 0x8e7b50); 0x2000: motivo 0x24 e o jogador perde 10% das reputações (+0xb atual, +9 na nação, +0xd mundial); 0x800: 0x17; 0x200: 0x18; cada um devolve 1',
          'with an empty exchange part, the deciding club board answers (0x696b10, by the 0x8e7b50 value); 0x2000: reason 0x24 and the player loses 10% of his reputations (+0xb current, +9 home, +0xd world); 0x800: 0x17; 0x200: 0x18; each returns 1')])
CAMPOS = [('gerente de transferências +0x86e: tabela de 8 bytes por id de pessoa (bit 0 = sob oferta); valor; lê (jogador_sob_oferta)',
           'transfer manager +0x86e: 8-byte table by person id (bit 0 = under offer); value; reads (jogador_sob_oferta)'),
          ('oferta +8/+0xc: id do clube que oferece e da pessoa; id; lê (jogador_sob_oferta)',
           'offer +8/+0xc: offering club id and person id; id; reads (jogador_sob_oferta)'),
          ('oferta +0x2d: oferta conhecida do humano; valor; lê (jogador_sob_oferta)',
           'offer +0x2d: offer known to the human; value; reads (jogador_sob_oferta)'),
          ('oferta +4: a parte de troca (palavra +0 = quantos); ponteiro; lê (veto_da_diretoria_na_oferta)',
           'offer +4: the exchange part (word +0 = how many); pointer; reads (veto_da_diretoria_na_oferta)'),
          ('contrato +4/+0x35: id do clube e situação (& 0x3f = 5: não vale); id e valor; lê (jogador_sob_oferta, clube_do_jogador_com_lei_bosman)',
           'contract +4/+0x35: club id and state (& 0x3f = 5: not valid); id and value; reads (jogador_sob_oferta, clube_do_jogador_com_lei_bosman)'),
          ('clube +0x53: nação do clube; ponteiro; lê (clube_do_jogador_com_lei_bosman)',
           'club +0x53: club nation; pointer; reads (clube_do_jogador_com_lei_bosman)'),
          ('jogador (staff +0x61) +9/+0xb/+0xd: as reputações na nação, atual e mundial (palavras com sinal); valor; lê e grava 90% (veto_da_diretoria_na_oferta)',
           'player (staff +0x61) +9/+0xb/+0xd: home, current and world reputations (signed words); value; reads and writes 90% (veto_da_diretoria_na_oferta)'),
          ('[0xae1080] o último contrato achado; ponteiro; grava (jogador_sob_oferta, clube_do_jogador_com_lei_bosman)',
           '[0xae1080] the last contract found; pointer; writes (jogador_sob_oferta, clube_do_jogador_com_lei_bosman)'),
          ('humano da vez +0x39: o clube dele; ponteiro; lê (jogador_sob_oferta)',
           'current human +0x39: his club; pointer; reads (jogador_sob_oferta)')]
INTERESSA = ['CMRTE', 'CMScout']
