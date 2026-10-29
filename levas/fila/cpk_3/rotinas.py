# Contratos e suspensão: as cláusulas de rebaixamento do clube rebaixado e do parceiro A/B
# (CONTRACT_MANAGER::trigger_relegation_clauses 0x4cd030, pelas 0x6831a0/0x684cd0/0x689c80 da liga e 0x42b130/0x42b520;
# 0x540a50 já em Rust; a notícia 0x8e3db0, original), tirar a oferta de contrato da pessoa ao clube
# (CONTRACT_MANAGER::remove_contract_offer 0x4cca70, por ~10 rotinas das transferências; 0x7a81a0 e 0x68d1f0
# originais, 0x8c5430 já em Rust) e se o jogador está suspenso para a partida (Discipline::is_player_suspended
# 0x55d270, pela escalação automática 0x6dbc50 e os titulares 0x6dbb80 já em Rust, convocações, telas; 0x7e85b0
# original, 0x600a50 já em Rust). Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Sem
# SEH, sem caixa, sem sorteio, sem leitura de lixo. Comparadores: as cláusulas pelo plano da cópia (a original dá as
# notícias); a oferta pela lista e pelas pendentes esperadas, montadas antes (a 0x68d1f0 libera o item); a suspensão
# lado a lado pelo valor e pelas 20 vagas do clube.
SAVE = False
ROTINAS = [('acionar_clausulas_de_rebaixamento', 0x4cd030, [0x83, 0xec, 0x0c, 0x53, 0x8b, 0x5c, 0x24, 0x14]),
           ('tirar_oferta_de_contrato', 0x4cca70, [0x8b, 0x44, 0x24, 0x08, 0x53]),
           ('esta_suspenso', 0x55d270, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10])]
DOCS = ('contratos e suspensão: as cláusulas de rebaixamento do clube rebaixado, tirar a oferta de contrato da pessoa ao clube e se o jogador está suspenso para a partida',
        'contracts and suspension: the relegation clauses of a relegated club, removing the contract offer of a person to a club and whether the player is suspended for the match',
        [('acionar_clausulas_de_rebaixamento', 'CONTRACT_MANAGER::trigger_relegation_clauses', '0x4cd030', 'contract_manager.cpp', 'thiscall, ret 8',
          'no clube rebaixado e no parceiro A/B, os 50 de +0xd7 com cláusula de rebaixamento por acionar (+0x1f = 1) passam a 2 e sai a notícia (0x8e3db0, tipo 3)',
          'in the relegated club and its A/B partner, the 50 at +0xd7 with a pending relegation clause (+0x1f = 1) go to 2 and the news is posted (0x8e3db0, type 3)'),
         ('tirar_oferta_de_contrato', 'CONTRACT_MANAGER::remove_contract_offer', '0x4cca70', 'contract_manager.cpp', 'thiscall, ret 8',
          'a primeira oferta da pessoa ao clube (+0x34): devolve a reserva de registro (0x7a81a0), desmarca o acertado (+0x50 = 2), sai da lista e as pendentes descem uma',
          'the first offer of the person to the club (+0x34): returns the record reservation (0x7a81a0), unmarks the agreed one (+0x50 = 2), leaves the list and the pending count drops by one'),
         ('esta_suspenso', 'Discipline::is_player_suspended', '0x55d270', 'discipline.cpp', 'thiscall, ret 0x10',
          'suspenso pelo tribunal geral (0x7e85b0 com 1) ou pelo da competição (+0x50, com 0); neste caso, num clube de humano, sai da vaga entre as 20 de +0x1e0',
          'suspended by the general body (0x7e85b0 with 1) or by the competition one (+0x50, with 0); in that case, in a human club, he leaves his slot among the 20 at +0x1e0')])
CAMPOS = [('gerenciador de contratos [this] contratos de 80 bytes (+4 clube, +0x1f cláusula de rebaixamento 1 → 2), +0x34 lista das ofertas (+0 pessoa, +4 clube, +0x50 estado, +0x63/+0x67/+0x6b reserva de registro); registro de 79 bytes [0xae2d0c] +0 índice do contrato, +8 pendentes; clube +0xd7 os 50 da comissão e elenco, +0x1e0 as 20 vagas da partida; pessoa +0x39 clube; competição +0x50 tribunal; lê e escreve (acionar_clausulas_de_rebaixamento, tirar_oferta_de_contrato, esta_suspenso)',
           'contract manager [this] 80-byte contracts (+4 club, +0x1f relegation clause 1 → 2), +0x34 offers list (+0 person, +4 club, +0x50 state, +0x63/+0x67/+0x6b record reservation); 79-byte record [0xae2d0c] +0 contract index, +8 pending; club +0xd7 the 50 staff and squad, +0x1e0 the 20 match slots; person +0x39 club; competition +0x50 ruling body; reads and writes (acionar_clausulas_de_rebaixamento, tirar_oferta_de_contrato, esta_suspenso)')]
INTERESSA = ['SGE', 'CMRTE']
