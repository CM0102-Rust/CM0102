# O emprestimo do arquivo de configuracao, a noticia do fim do contrato e a do pedido de adiamento da transferencia
# (transfer_manager.cpp, contract_manager.cpp): o emprestimo (TRANSFER_MANAGER::add_config_loan 0x8bf400, 49 chamadas
# ate 2002-06-01 na cobertura de 17/10, no comeco do jogo: as condicoes, o contrato estendido ate o fim do emprestimo, o
# contrato de emprestimo assinado, o elenco, os numeros de camisa, as reservas do time B e a permissao de trabalho com o
# fim recebido 7 dias adiante), a noticia 0xbb9 do fim de contrato, teste ou emprestimo (CONTRACT_MANAGER::
# post_contract_end_news 0x4da590, 149 chamadas) e a 0xfab do pedido de adiamento (TRANSFER_MANAGER::
# post_transfer_delay_request_news 0x8dda90, 13 chamadas). Chamadas originais pelo endereco: 0x85e410, 0x540a50,
# 0x53c590, 0x4cf6d0, 0x417c10, 0x85e590, 0x4d01e0, 0x5563f0, 0x540980, 0x85add0, 0x85a470, 0x4cc0f0, 0x8c7560,
# 0x8c7070, 0x68d020, 0x68d040, 0x8d1130, 0x77bbe0; as outras ja em Rust (0x4cf5a0, 0x54c5b0, 0x842180, 0x68d1b0,
# 0x785790, 0x7861e0, 0x7863f0, 0x786780). Iguais no cm0102-1080.exe. Comparadores: o emprestimo so a original com o
# plano pelas mesmas condicoes (passando, a pessoa sai emprestada; senao fica como estava); as noticias so a original
# (alocam).
SAVE = False
ROTINAS = [('emprestimo_da_configuracao', 0x8bf400, [0x6a, 0xff, 0x68, 0x28, 0x3b, 0x96, 0x00]),
           ('noticia_fim_do_contrato', 0x4da590, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_pedido_de_adiamento', 0x8dda90, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o empréstimo do arquivo de configuração, a notícia do fim do contrato e a do pedido de adiamento da transferência',
        'a loan from the configuration file, the news of a contract ending and of a transfer delay request',
        [('emprestimo_da_configuracao', 'add_config_loan', '0x8bf400', 'transfer_manager.cpp', 'thiscall',
          'o empréstimo do arquivo de configuração: as condições, o contrato estendido até o fim, o contrato de empréstimo, o elenco, os números de camisa e a permissão de trabalho',
          'a loan from the configuration file: the checks, the contract extended to its end, the loan contract, the squad, the squad numbers and the work permit'),
         ('noticia_fim_do_contrato', 'post_contract_end_news', '0x4da590', 'contract_manager.cpp', 'thiscall',
          'a notícia 0xbb9 do fim do contrato, do teste ou do empréstimo, ao clube, aos interessados ou, para os famosos, aos técnicos da nação ou a todos',
          'the 0xbb9 news of a contract, trial or loan ending, to the club, the interested clubs or, for famous players, the nation\'s managers or everyone'),
         ('noticia_pedido_de_adiamento', 'post_transfer_delay_request_news', '0x8dda90', 'transfer_manager.cpp', 'thiscall',
          'a notícia 0xfab do clube que pede para adiar a transferência uma semana, aos clubes humanos interessados',
          'the 0xfab news of a club asking to delay a transfer by a week, to the interested human clubs')])
CAMPOS = [('pessoa +0x46: data (fim do vínculo); valor; lê e grava o fim do empréstimo (emprestimo_da_configuracao)',
           'person +0x46: date (end of tie); value; reads and writes the loan end (emprestimo_da_configuracao)'),
          ('pessoa +0x6d; valor; grava 0x10 no lugar de 2 (emprestimo_da_configuracao)', 'person +0x6d; value; writes 0x10 instead of 2 (emprestimo_da_configuracao)'),
          ('contrato +0x2d: fim do contrato; valor; grava o fim do empréstimo (emprestimo_da_configuracao)',
           'contract +0x2d: contract end; value; writes the loan end (emprestimo_da_configuracao)'),
          ('contrato local +0x25: começo; valor; grava (emprestimo_da_configuracao)', 'local contract +0x25: start; value; writes (emprestimo_da_configuracao)'),
          ('pessoa +0x1a nação, +0x39 clube atual, +0x61 ficha; ficha +0xb/+0xd reputação; valor; lê (noticia_fim_do_contrato)',
           'person +0x1a nation, +0x39 current club, +0x61 player record; record +0xb/+0xd reputation; value; reads (noticia_fim_do_contrato)'),
          ('registro de 79 bytes +0 (general.dat): índice do contrato; valor; lê (noticia_fim_do_contrato)',
           '79-byte record +0 (general.dat): contract index; value; reads (noticia_fim_do_contrato)'),
          ('oferta +8 comprador, +0xc jogador, +0x20 vaga, +0x2c estado; valor; lê (noticia_pedido_de_adiamento)',
           'offer +8 buyer, +0xc player, +0x20 slot, +0x2c state; value; reads (noticia_pedido_de_adiamento)'),
          ('[0xae1080] último contrato consultado; ponteiro; grava (as notícias)', '[0xae1080] last contract looked up; pointer; writes (the news)')]
INTERESSA = ['CMRTE', 'SGE']
