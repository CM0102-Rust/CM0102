# A negociação guardada e duas notícias (transfer_manager.cpp, manager_manager.cpp): os termos de uma oferta guardados
# no vetor das negociações (TRANSFER_MANAGER::add_transfer_negotiation 0x8be9d0, pelas contrapropostas: o vetor cresce
# de 50, a negociação nova recebe os termos campo a campo e o id seguinte, a oferta é agendada), o boato do técnico em
# outro clube (MANAGER_MANAGER::post_job_rumour_news 0x69fd50, com dois sorteios) e a recusa do jogador em se mudar
# (TRANSFER_MANAGER::post_player_rejects_move_news 0x8d8310). Notícias montadas na pilha, de volta pelo critério novo
# (31/10/2026). Chamadas originais pelo endereço: 0x84b580, 0x945667, 0x945501, 0x944cf1, 0x8e5680, 0x8e5710, 0x944c94,
# 0x417c10, 0x68d020, 0x68d2f0, 0x7875e0, 0x8e4610, 0x8d6bf0, 0x8c6220, 0x90cfc0 (sorteio), 0x8d1da0, 0x4cf5a0, 0x8d1130,
# 0x77bbe0, 0x785790, 0x7861e0, a caixa "Error" (aviso_em). Iguais no cm0102-1080.exe. Comparadores: só a original nas
# três (alocam, sorteiam, agendam, mandam notícias); as cópias levam os ramos e se provam pela carreira e pelas mutações.
SAVE = False
ROTINAS = [('guardar_a_negociacao', 0x8be9d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_do_boato_do_tecnico', 0x69fd50, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_da_recusa_em_se_mudar', 0x8d8310, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a negociação guardada da oferta, o boato do técnico em outro clube e a recusa do jogador em se mudar',
        'the stored negotiation of an offer, the manager job rumour and the player refusing to move',
        [('guardar_a_negociacao', 'add_transfer_negotiation', '0x8be9d0', 'transfer_manager.cpp', 'thiscall',
          'guarda os termos da oferta numa negociação nova (o vetor cresce de 50; o id seguinte) e agenda a oferta em 2 dias, ou em 14 com notícia para o comprador humano',
          'stores the offer terms in a new negotiation (the array grows by 50; the next id) and schedules the offer in 2 days, or in 14 with news for a human buyer'),
         ('noticia_do_boato_do_tecnico', 'post_job_rumour_news', '0x69fd50', 'manager_manager.cpp', 'thiscall',
          'com um sorteio em quatro, a notícia 0x1776 do boato que liga um técnico a outro clube',
          'on a one-in-four draw, the 0x1776 news of a rumour linking a manager with another club'),
         ('noticia_da_recusa_em_se_mudar', 'post_player_rejects_move_news', '0x8d8310', 'transfer_manager.cpp', 'thiscall',
          'a notícia 0xfae do jogador que recusa a mudança, com o motivo e os técnicos dos dois clubes',
          'the 0xfae news of the player refusing the move, with the reason and both clubs\' managers')])
CAMPOS = [('transferências +0xc: vetor das negociações (capacidade +0x10, conta +0x14); +0x85a: id seguinte; ponteiro e valor; lê e grava (guardar_a_negociacao)',
           'transfers +0xc: negotiation array (capacity +0x10, count +0x14); +0x85a: next id; pointer and value; reads and writes (guardar_a_negociacao)'),
          ('oferta +0x28: vaga da negociação (−1 sem); +0x2c: status (1); +0x2d: marcada; valor; lê e grava (guardar_a_negociacao)',
           'offer +0x28: negotiation slot (−1 none); +0x2c: status (1); +0x2d: marked; value; reads and writes (guardar_a_negociacao)'),
          ('negociação (0x41 bytes): cópia dos termos +0..+0x40 (data +0x12; 4 bits baixos de +0x30 e +0x31), +0x20 da oferta, +0x24 id, +4 lista nova; grava (guardar_a_negociacao)',
           'negotiation (0x41 bytes): copy of the terms +0..+0x40 (date +0x12; low 4 bits of +0x30 and +0x31), +0x20 of the offer, +0x24 id, +4 new list; writes (guardar_a_negociacao)'),
          ('pessoa +0x39: clube; +0x3d: cargo; lê (noticia_do_boato_do_tecnico)',
           'person +0x39: club; +0x3d: job; reads (noticia_do_boato_do_tecnico)'),
          ('clube +0xcf: técnico; ponteiro; lê (noticia_da_recusa_em_se_mudar)',
           'club +0xcf: manager; pointer; reads (noticia_da_recusa_em_se_mudar)'),
          ('[0xdd6ec8] data a partir da qual saem as notícias; valor; lê (noticia_do_boato_do_tecnico)',
           '[0xdd6ec8] date from which the news are posted; value; reads (noticia_do_boato_do_tecnico)'),
          ('[0xae1080] o último contrato pedido; ponteiro; grava (noticia_da_recusa_em_se_mudar)',
           '[0xae1080] the last contract asked for; pointer; writes (noticia_da_recusa_em_se_mudar)')]
INTERESSA = ['CMRTE', 'GPF2']
