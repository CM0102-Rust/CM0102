# A oferta de emprestimo pelo jogador da lista e duas noticias de contrato (shortlist_manager.cpp,
# contract_manager.cpp): a oferta de emprestimo que um clube faz por um jogador da lista dele (Shortlist_Manager::
# try_loan_offer_for_player 0x842740, pela avaliacao das listas 0x84a020: interesse, periodo, data de volta, janela e a
# porcentagem do salario), a noticia do contrato acabando (post_contract_expiry_news 0x4d6790, tambem aos clubes
# interessados) e a da saida pela lei Bosman (post_bosman_departure_news 0x4d6d70). Chamadas originais pelo endereco:
# 0x846920 (o interesse), 0x843e10 (o estado da vaga), 0x54cab0, 0x54cfb0, 0x54cf70, 0x669360, 0x544670, 0x68d020/
# 0x68d040, 0x77bbe0, 0x785790, 0x7861e0, a caixa "Error" (aviso_em); as outras ja em Rust (0x849ce0, 0x4cf5a0,
# 0x8d0590, 0x8d0530, 0x549f70, 0x54c770, 0x8bed60, 0x842180). Iguais no cm0102-1080.exe. Comparadores: a oferta pelo
# ensaio (a copia sem a oferta e sem a atualizacao da lista, a original a partir do mesmo estado; recusa: devolucao e
# memoria iguais; oferta: a original tambem oferece); as noticias so a original (alocam).
SAVE = False
ROTINAS = [('tentar_emprestimo_pelo_jogador', 0x842740, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_contrato_acabando', 0x4d6790, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_saida_pela_lei_bosman', 0x4d6d70, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a oferta de empréstimo pelo jogador da lista, a notícia do contrato acabando e a da saída pela lei Bosman',
        'the loan offer for a shortlisted player, the news of an expiring contract and of a Bosman departure',
        [('tentar_emprestimo_pelo_jogador', 'try_loan_offer_for_player', '0x842740', 'shortlist_manager.cpp', 'thiscall',
          'com interesse de 3,5, o período do empréstimo e a data de volta dentro do contrato, da janela e a 3 semanas de hoje, faz a oferta com 0, 50 ou 100% do salário pela diferença de reputação; senão −1',
          'with an interest of 3.5, the loan period and a return date within the contract, the window and 3 weeks from today, makes the offer with 0, 50 or 100% of the wage by the reputation gap; otherwise −1'),
         ('noticia_contrato_acabando', 'post_contract_expiry_news', '0x4d6790', 'contract_manager.cpp', 'thiscall',
          'a notícia 0xbbd do contrato acabando ao técnico do clube e, se pedido, aos clubes interessados de nações ativas',
          'the 0xbbd news of an expiring contract to the club manager and, if asked, to interested clubs of active nations'),
         ('noticia_saida_pela_lei_bosman', 'post_bosman_departure_news', '0x4d6d70', 'contract_manager.cpp', 'thiscall',
          'a notícia 0xbbe da saída pela lei Bosman, com o clube de destino ou −1',
          'the 0xbbe news of a Bosman departure, with the destination club or −1')])
CAMPOS = [('contrato +4: clube (dono); id; lê (as três)', 'contract +4: club (owner); id; reads (all three)'),
          ('contrato +0x2d: fim do contrato; valor; lê (tentar_emprestimo_pelo_jogador)', 'contract +0x2d: contract end; value; reads (tentar_emprestimo_pelo_jogador)'),
          ('vaga da lista +0x24 >> 1 & 0xf: situação; valor; lê (tentar_emprestimo_pelo_jogador)',
           'shortlist slot +0x24 >> 1 & 0xf: squad status; value; reads (tentar_emprestimo_pelo_jogador)'),
          ('clube +0x53: nação; ponteiro; +0x80: reputação; valor; lê (tentar_emprestimo_pelo_jogador, noticia_contrato_acabando)',
           'club +0x53: nation; pointer; +0x80: reputation; value; reads (tentar_emprestimo_pelo_jogador, noticia_contrato_acabando)'),
          ('[0xb63d8a] + 72 × nação-chave: o começo da janela; valor; lê (tentar_emprestimo_pelo_jogador)',
           '[0xb63d8a] + 72 × key nation: window start; value; reads (tentar_emprestimo_pelo_jogador)'),
          ('pessoa +0x61: ficha; ponteiro; lê (noticia_contrato_acabando)', 'person +0x61: player record; pointer; reads (noticia_contrato_acabando)'),
          ('[0xae1080] o último contrato pedido; ponteiro; grava (tentar_emprestimo_pelo_jogador)',
           '[0xae1080] the last contract asked for; pointer; writes (tentar_emprestimo_pelo_jogador)')]
INTERESSA = ['CMRTE', 'CMScout']
