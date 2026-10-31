# A recusa dos termos e duas notícias da diretoria (transfer_manager.cpp, manager_manager.cpp): o jogador recusa os
# termos de uma oferta (TRANSFER_MANAGER::handle_player_rejected_terms 0x8c28c0, pelas respostas de contrato: oferta
# reagendada, listas e marcas de lance, reputação do jogador que recusa um clube mais famoso; ou o jogador sai da
# troca e o valor dele passa ao preço), a notícia da confiança da diretoria (MANAGER_MANAGER::
# post_board_confidence_update_news 0x6a4c40, pelos dias do técnico) e a das expectativas da diretoria
# (post_board_expectations_news 0x6a4a20, pelo começo da temporada). Notícias montadas na pilha, de volta pelo
# critério novo (31/10/2026). Chamadas originais pelo endereço: 0x4cf5a0, 0x68d1b0, 0x68d1f0, 0x68d040, 0x944c94,
# 0x600a50, 0x84aeb0, 0x843e10, 0x8c6220, 0x4cca70, 0x8d8e90, 0x4d0c60, 0x8c8090, 0x8c0810, 0x8c2f50 (cpv_1), 0x77bbe0,
# 0x785790, 0x786090, 0x7861e0, a caixa "Error" (aviso_em). Iguais no cm0102-1080.exe. Comparadores: só a original nas
# três (mexem nas ofertas e listas, alocam notícias); as cópias levam os ramos e se provam pela carreira e pelas
# mutações.
SAVE = False
ROTINAS = [('recusa_dos_termos', 0x8c28c0, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00]),
           ('noticia_da_confianca_da_diretoria', 0x6a4c40, [0x6a, 0xff, 0x68, 0x6b, 0xce, 0x95, 0x00]),
           ('noticia_das_expectativas_da_diretoria', 0x6a4a20, [0x6a, 0xff, 0x68, 0x4b, 0xce, 0x95, 0x00])]
DOCS = ('a recusa dos termos pelo jogador, a notícia da confiança da diretoria e a das expectativas da diretoria',
        'the player\'s refusal of the terms, the board confidence news and the board expectations news',
        [('recusa_dos_termos', 'handle_player_rejected_terms', '0x8c28c0', 'transfer_manager.cpp', 'thiscall',
          'o jogador recusa os termos: a oferta volta em 10 dias, saem as marcas de lance, a reputação dele sobe se o comprador é mais famoso; ou ele sai da troca e o valor passa ao preço',
          'the player refuses the terms: the offer returns in 10 days, the bid markers go, his reputation rises if the buyer is more famous; or he leaves the exchange and his value moves to the fee'),
         ('noticia_da_confianca_da_diretoria', 'post_board_confidence_update_news', '0x6a4c40', 'manager_manager.cpp', 'thiscall',
          'a notícia 0x1772 da confiança da diretoria com os dias do técnico no cargo',
          'the 0x1772 board confidence news with the manager\'s days in the job'),
         ('noticia_das_expectativas_da_diretoria', 'post_board_expectations_news', '0x6a4a20', 'manager_manager.cpp', 'thiscall',
          'a notícia 0x1771 das expectativas da diretoria para o técnico do clube',
          'the 0x1771 board expectations news to the club manager')])
CAMPOS = [('oferta +0x10: estado dos termos (4 = recusados); +0x2c: status (0xf); +0x2e: tentativas; valor; grava (recusa_dos_termos)',
           'offer +0x10: terms state (4 = refused); +0x2c: status (0xf); +0x2e: attempts; value; writes (recusa_dos_termos)'),
          ('oferta +0x1b: preço; valor; lê e grava (recusa_dos_termos)',
           'offer +0x1b: fee; value; reads and writes (recusa_dos_termos)'),
          ('oferta +4: lista dos jogadores da troca (liberada se vazia); ponteiro; lê e grava (recusa_dos_termos)',
           'offer +4: list of the exchange players (freed when empty); pointer; reads and writes (recusa_dos_termos)'),
          ('[this+0x86e] registro de 8 bytes por jogador: bit 0 de +0 (em negociação), +2 (ofertas); valor; grava (recusa_dos_termos)',
           '[this+0x86e] 8-byte record per player: bit 0 of +0 (under negotiation), +2 (offers); value; writes (recusa_dos_termos)'),
          ('clube +0x80: reputação; valor; lê (recusa_dos_termos)',
           'club +0x80: reputation; value; reads (recusa_dos_termos)'),
          ('ficha do jogador (pessoa +0x61) +0xb: reputação atual (até 10000); valor; lê e grava (recusa_dos_termos)',
           'player record (person +0x61) +0xb: current reputation (up to 10000); value; reads and writes (recusa_dos_termos)'),
          ('clube +0xcf: técnico; ponteiro; técnico +0x3e: data de chegada (dia, ano +0x40); lê (noticia_da_confianca_da_diretoria)',
           'club +0xcf: manager; pointer; manager +0x3e: arrival date (day, year +0x40); reads (noticia_da_confianca_da_diretoria)'),
          ('[0xdd6ec8] data a partir da qual saem as notícias (dia, ano +2); valor; lê (as duas notícias)',
           '[0xdd6ec8] date from which the news are posted (day, year +2); value; reads (both news)'),
          ('[0xae1080] o último contrato pedido; ponteiro; grava (recusa_dos_termos)',
           '[0xae1080] the last contract asked for; pointer; writes (recusa_dos_termos)')]
INTERESSA = ['CMRTE']
