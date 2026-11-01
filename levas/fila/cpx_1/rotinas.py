# A oferta de transferencia recusada pelo clube vendedor, a retirada pelo comprador e a noticia da oferta recusada
# (transfer_manager.cpp): a recusa (TRANSFER_MANAGER::reject_transfer_offer 0x8bf680, 35 chamadas ate 2002-06-01 na
# cobertura de 17/10: estado 2, desassossego do jogador, lances desmarcados, a oferta volta em 25 dias para sair), a
# retirada (withdraw_transfer_offer 0x8bfcb0, 28 chamadas: estado 3, +0x10 = 4, volta em 10 dias; no motivo 2, o
# desassossego) e a noticia 0xfa1 (post_offer_rejected_news 0x8d5170, chamada pelas duas e pela recusa do emprestimo).
# Chamadas originais pelo endereco: 0x8c1290 (o desassossego, com sorteio), 0x8cbf70, 0x8c6db0, 0x4cf4a0, 0x843e10,
# 0x4ccb30, 0x600a50, 0x842080, 0x8d1130, 0x8e7a70, 0x77bbe0; as outras ja em Rust (0x4cf5a0, 0x68d1b0, 0x84aeb0,
# 0x849ce0, 0x4cca70, 0x8c6220, 0x785790, 0x7861e0). Iguais no cm0102-1080.exe. Comparadores: a recusa e a retirada
# pelo ensaio (a copia sem as chamadas que mexem no resto do jogo, a original a partir do mesmo estado; iguais o
# estado, +0x10, a marca +4 dos jogadores da troca e a marca de oferta da pessoa); a noticia so a original (aloca).
SAVE = False
ROTINAS = [('recusar_oferta_de_transferencia', 0x8bf680, [0x51, 0x53, 0x8b, 0x5c, 0x24, 0x0c]),
           ('retirar_oferta_de_transferencia', 0x8bfcb0, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('noticia_oferta_recusada', 0x8d5170, [0x6a, 0xff, 0x68, 0x2b, 0x43, 0x96, 0x00])]
DOCS = ('a oferta de transferência recusada pelo clube vendedor, a retirada pelo comprador e a notícia da oferta recusada',
        'a transfer offer rejected by the selling club, one withdrawn by the buyer and the news of the rejected offer',
        [('recusar_oferta_de_transferencia', 'reject_transfer_offer', '0x8bf680', 'transfer_manager.cpp', 'thiscall',
          'recusa a oferta: o jogador pode se desassossegar, estado 2, lances desmarcados, ofertas de contrato fechadas, a notícia, a volta em 25 dias para sair e a marca de oferta da pessoa',
          'rejects the offer: the player may become unsettled, state 2, bids unmarked, contract offers closed, the news, back in 25 days for removal and the person\'s offer flag'),
         ('retirar_oferta_de_transferencia', 'withdraw_transfer_offer', '0x8bfcb0', 'transfer_manager.cpp', 'thiscall',
          'retira a oferta: estado 3, ofertas de contrato guardadas reenviadas, lances desmarcados, a volta em 10 dias; no motivo 2, o desassossego; a notícia e as ofertas de contrato',
          'withdraws the offer: state 3, held contract offers resubmitted, bids unmarked, back in 10 days; for reason 2, the unsettling; the news and the contract offers'),
         ('noticia_oferta_recusada', 'post_offer_rejected_news', '0x8d5170', 'transfer_manager.cpp', 'thiscall',
          'a notícia 0xfa1 da oferta recusada ou retirada, com o motivo, aos clubes humanos interessados',
          'the 0xfa1 news of a rejected or withdrawn offer, with the reason, to the interested human clubs')])
CAMPOS = [('oferta +0: lista das ofertas rivais; ponteiro; lê (recusa, retirada)', 'offer +0: rival offers list; pointer; reads (reject, withdraw)'),
          ('oferta +4: lista dos jogadores da troca (+4 de cada vaga: marca); ponteiro; lê e grava 0 na marca (recusa, retirada)',
           'offer +4: exchange players list (+4 of each slot: flag); pointer; reads and writes 0 to the flag (reject, withdraw)'),
          ('oferta +8 comprador, +0xc jogador; id; lê (as três)', 'offer +8 buyer, +0xc player; id; reads (all three)'),
          ('oferta +0x10; valor; grava 4 e 0 (retirada), 0 (recusa)', 'offer +0x10; value; writes 4 and 0 (withdraw), 0 (reject)'),
          ('oferta +0x2c: estado; valor; grava 2 (recusa) e 3 (retirada); lê (a notícia)', 'offer +0x2c: state; value; writes 2 (reject) and 3 (withdraw); reads (the news)'),
          ('oferta +0x2d; valor; grava 1 com a marca (recusa, retirada)', 'offer +0x2d; value; writes 1 when flagged (reject, withdraw)'),
          ('oferta +0x11, +0x1f, +0x20 vaga, +0x2e, +0x2f; valor; lê', 'offer +0x11, +0x1f, +0x20 slot, +0x2e, +0x2f; value; reads'),
          ('gerência de transferências +0x86e: registro de 8 bytes por pessoa, bit 0 = oferta em andamento; valor; lê e apaga (recusa, retirada)',
           'transfer manager +0x86e: 8-byte record per person, bit 0 = offer in progress; value; reads and clears (reject, withdraw)'),
          ('contrato +4 clube, +0x35 & 0x3f tipo; valor; lê (a notícia)', 'contract +4 club, +0x35 & 0x3f type; value; reads (the news)'),
          ('[0xae1080] último contrato consultado; ponteiro; grava (as três)', '[0xae1080] last contract looked up; pointer; writes (all three)')]
INTERESSA = ['CMRTE']
