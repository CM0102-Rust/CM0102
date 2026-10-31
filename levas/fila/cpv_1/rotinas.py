# A oferta passada aos clubes e duas notícias de oferta (transfer_manager.cpp): a oferta com todos os jogadores de
# acordo passa à aprovação dos clubes (TRANSFER_MANAGER::advance_offer_to_club_approval 0x8c2f50, pela recusa dos
# termos 0x8c28c0 e pelas respostas de contrato; cancela as rivais, a espera da permissão de trabalho com sorteio, as
# decisões do comprador e do vendedor), a notícia da oferta aceita (post_offer_accepted_news 0x8d76d0, pela aceitação
# do lance 0x8c0080) e a do empréstimo oferecido (post_loan_offer_news 0x8e1780). Notícias montadas na pilha, de volta
# pelo critério novo (31/10/2026). Chamadas originais pelo endereço: 0x4cf5a0 (contrato), 0x68d1b0 (lista), 0x8c6220
# (agendar), 0x84aeb0 (marcas de lance), 0x8c7470, 0x8e2a00, 0x7875e0, 0x8da5e0, 0x600a50, 0x8cfc90, 0x8c36b0, 0x8bfcb0,
# 0x8c3310, 0x8d1130 (destinatários), 0x77bbe0, 0x785790, 0x7861e0 (a notícia), a caixa "Error" (aviso_em). Iguais no
# cm0102-1080.exe. Comparadores: só a original nas três (mexem nas ofertas, sorteiam, alocam notícias); as cópias levam
# os ramos e se provam pela carreira e pelas mutações.
SAVE = False
ROTINAS = [('passar_a_oferta_aos_clubes', 0x8c2f50, [0x83, 0xec, 0x0c, 0x53, 0x55]),
           ('noticia_da_oferta_aceita', 0x8d76d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_do_emprestimo_oferecido', 0x8e1780, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a oferta passada aos clubes, a notícia da oferta aceita e a do empréstimo oferecido',
        'the offer passed to the clubs, the news of an accepted offer and of a loan offer',
        [('passar_a_oferta_aos_clubes', 'advance_offer_to_club_approval', '0x8c2f50', 'transfer_manager.cpp', 'thiscall',
          'com todos os jogadores de acordo, cancela as ofertas rivais (reagendadas em 20 dias), espera a permissão de trabalho (18 a 24 dias) ou passa a oferta à decisão do comprador e do vendedor',
          'once every player agrees, cancels the rival offers (rescheduled in 20 days), waits for the work permit (18 to 24 days) or passes the offer to the buyer\'s and the seller\'s decision'),
         ('noticia_da_oferta_aceita', 'post_offer_accepted_news', '0x8d76d0', 'transfer_manager.cpp', 'thiscall',
          'a notícia 0xfa3 da oferta aceita aos clubes humanos envolvidos',
          'the 0xfa3 news of an accepted offer to the human clubs involved'),
         ('noticia_do_emprestimo_oferecido', 'post_loan_offer_news', '0x8e1780', 'transfer_manager.cpp', 'thiscall',
          'a notícia 0xfb5 da oferta de empréstimo aos clubes humanos envolvidos; sem contrato, a caixa de erro',
          'the 0xfb5 news of a loan offer to the human clubs involved; without a contract, the error box')])
CAMPOS = [('oferta +0: lista das ofertas pelo mesmo jogador (índices); +4: lista dos jogadores da troca; ponteiro; lê (passar_a_oferta_aos_clubes)',
           'offer +0: list of offers for the same player (indices); +4: list of the exchange players; pointer; reads (passar_a_oferta_aos_clubes)'),
          ('oferta +8: clube comprador; +0xc: jogador; id; lê (as três)',
           'offer +8: buying club; +0xc: player; id; reads (all three)'),
          ('oferta +0x10: estado dos termos (2 = de acordo); +0x20: índice no vetor; +0x2f: confirmação direta; valor; lê (passar_a_oferta_aos_clubes)',
           'offer +0x10: terms state (2 = agreed); +0x20: index in the array; +0x2f: direct confirmation; value; reads (passar_a_oferta_aos_clubes)'),
          ('oferta +0x2c: status (0xa espera da permissão, 0xb com os clubes, 0xf cancelada); +0x2e: tentativas (3 nas rivais); valor; lê e grava (passar_a_oferta_aos_clubes)',
           'offer +0x2c: status (0xa waiting for the permit, 0xb with the clubs, 0xf cancelled); +0x2e: attempts (3 on rivals); value; reads and writes (passar_a_oferta_aos_clubes)'),
          ('oferta +0x30: marcas (& 0xf: em espera); valor; lê (passar_a_oferta_aos_clubes)',
           'offer +0x30: flags (& 0xf: on hold); value; reads (passar_a_oferta_aos_clubes)'),
          ('oferta +0x1f, +0x20, +0x24, +4; termos +0xc, +0x2d, +0x2f; valor; lê (as notícias)',
           'offer +0x1f, +0x20, +0x24, +4; terms +0xc, +0x2d, +0x2f; value; reads (the news)'),
          ('contrato +4: clube; id; +0x35 & 0x3f: tipo (5 sem clube); +0xc: salário; lê (as três)',
           'contract +4: club; id; +0x35 & 0x3f: type (5 without club); +0xc: wage; reads (all three)'),
          ('[0xae1080] o último contrato pedido; ponteiro; grava (as três)',
           '[0xae1080] the last contract asked for; pointer; writes (all three)')]
INTERESSA = ['CMRTE']
