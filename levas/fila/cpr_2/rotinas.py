# Quem pode negociar (transfer_manager.cpp): se o clube pode chegar ao jogador (TRANSFER_MANAGER::can_club_bid_for_player
# 0x8ce990, pelas listas de alvos dos clubes 0x83f3a0/0x842e10/0x843800/0x84a020, pelos jovens 0x7ad0e0/0x7ad590, pelos
# empréstimos 0x8d0040, pela mídia e pelas telas; quadro SEH da LLIST local, construída e destruída como na original), os
# termos mandados ao jogador da troca (TRANSFER_MANAGER::note_terms_offered_to_exchange_player 0x8c2060, pela oferta de
# contrato 0x4cc490 quando a pessoa não é o jogador principal) e a fila da transferência livre do clube
# (TRANSFER_MANAGER::count_club_free_transfer_queue 0x8c1780, pelo limite do elenco 0x8cd450). Iguais no
# cm0102-1080.exe. Comparadores lado a lado (a marca da pessoa, o motivo, o estado da oferta, as marcas da troca, os
# cursores das listas) com a devolução; casos da caixa "Error", só a original.
SAVE = False
ROTINAS = [('clube_pode_chegar_ao_jogador', 0x8ce990, [0x6a, 0xff, 0x68, 0x78, 0x3f, 0x96, 0x00]),
           ('termos_ao_jogador_da_troca', 0x8c2060, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('fila_livre_do_clube', 0x8c1780, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('quem pode negociar: se o clube pode chegar ao jogador, os termos mandados ao jogador da troca e a fila da transferência livre do clube',
        'who may negotiate: whether the club may approach the player, the terms sent to the exchange player and the club\'s free transfer queue',
        [('clube_pode_chegar_ao_jogador', 'TRANSFER_MANAGER::can_club_bid_for_player', '0x8ce990', 'transfer_manager.cpp', 'thiscall, ret 0x10',
          'não pode (0 e o motivo): nação que não deixa sair (0x27), contrato acertado (0x21), trocas abertas (7), fila livre (1), oferta em negociação (0x12, marca +0x2d) ou transferência futura (0x20), as regras do clube basco (0x19), da nação [0x9cf4dc] (0x28) e dos clubes [0x9d0128]/[0x9d01a0] (0x1a); senão 1',
          'cannot (0 and the reason): nation that keeps its players (0x27), agreed contract (0x21), open exchanges (7), free queue (1), offer in negotiation (0x12, flag +0x2d) or future transfer (0x20), the rules of the Basque club (0x19), of nation [0x9cf4dc] (0x28) and of clubs [0x9d0128]/[0x9d01a0] (0x1a); otherwise 1'),
         ('termos_ao_jogador_da_troca', 'TRANSFER_MANAGER::note_terms_offered_to_exchange_player', '0x8c2060', 'transfer_manager.cpp', 'thiscall, ret 8',
          'marca o jogador da troca a quem foram mandados os termos; nenhum marcado: a oferta vai ao estado 8 (de 7) ou 6',
          'flags the exchange player sent terms; when none is flagged, the offer moves to state 8 (from 7) or 6'),
         ('fila_livre_do_clube', 'TRANSFER_MANAGER::count_club_free_transfer_queue', '0x8c1780', 'transfer_manager.cpp', 'thiscall, ret 4',
          'conta as pessoas da fila da transferência livre do clube ou do parceiro dele (0x540a50)',
          'counts the free transfer queue people of the club or of its partner club (0x540a50)')])
CAMPOS = [('staff +0x1a / +0x1e: as nacionalidades; +0x39: o clube do contrato; +0x61: a ficha de jogador; ponteiro; lê (clube_pode_chegar_ao_jogador, fila_livre_do_clube)',
           'staff +0x1a / +0x1e: the nationalities; +0x39: the contract club; +0x61: the player record; pointer; reads (clube_pode_chegar_ao_jogador, fila_livre_do_clube)'),
          ('TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa, bit 0x40 (contrato acertado); valor; lê e grava (clube_pode_chegar_ao_jogador)',
           'TRANSFER_MANAGER +0x86e: person\'s 8-byte record, bit 0x40 (agreed contract); value; reads and writes (clube_pode_chegar_ao_jogador)'),
          ('TRANSFER_MANAGER +0x92c: a fila da transferência livre (LLIST de ids; cursor +0x936); id; lê (clube_pode_chegar_ao_jogador, fila_livre_do_clube)',
           'TRANSFER_MANAGER +0x92c: the free transfer queue (LLIST of ids; cursor +0x936); id; reads (clube_pode_chegar_ao_jogador, fila_livre_do_clube)'),
          ('oferta de transferência +4: a lista da troca (itens: +0 id, +4 termos mandados); +0x2c: o estado; +0x2d: a marca; valor; lê e grava (clube_pode_chegar_ao_jogador, termos_ao_jogador_da_troca)',
           'transfer offer +4: the exchange list (items: +0 id, +4 terms sent); +0x2c: the state; +0x2d: the flag; value; reads and writes (clube_pode_chegar_ao_jogador, termos_ao_jogador_da_troca)'),
          ('clube +0x53 / +0x57: a nação e a região; ponteiro; lê (clube_pode_chegar_ao_jogador)', 'club +0x53 / +0x57: the nation and the region; pointer; reads (clube_pode_chegar_ao_jogador)'),
          ('[0x9cf458], [0x9cf424], [0x9cf4dc], [0x9cf8d8], [0x9cf488], [0x9d0128], [0x9d01a0]: os ids das nações e dos clubes das regras; id; lê (clube_pode_chegar_ao_jogador)',
           '[0x9cf458], [0x9cf424], [0x9cf4dc], [0x9cf8d8], [0x9cf488], [0x9d0128], [0x9d01a0]: the nation and club ids of the rules; id; reads (clube_pode_chegar_ao_jogador)')]
INTERESSA = ['CMRTE']
