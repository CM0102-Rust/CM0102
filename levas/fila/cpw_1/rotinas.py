# A satisfacao do jogador com o contrato e duas noticias de contrato (staff_contracts.cpp, contract_manager.cpp): a
# satisfacao de 1 a 3 (STAFF_CONTRACT::get_contract_satisfaction 0x866620, pelo passe diario do pessoal 0x5ce2d0 com
# sorteio(50) zero; o resultado escolhe o ramo do dia: pedir contrato novo, ficar mais feliz, pedir para sair), a
# noticia de que o jogador quer contrato novo (post_wants_new_contract_news 0x4d9f80, pelo mesmo passe) e a do pedido de
# transferencia retirado (post_transfer_request_withdrawn_news 0x4d9460). Chamadas originais pelo endereco: 0x4ca470
# (a situacao merecida), 0x85fc50 (o contrato desejado), 0x866830 (o valor do contrato), 0x8c1870, 0x8c5cf0, 0x8c5e40,
# 0x77bbe0, 0x785790, 0x7861e0 (a noticia), a caixa "Error" (aviso_em). Iguais no cm0102-1080.exe. Comparadores: a
# satisfacao lado a lado (devolucao, sorteio, [0xae1080] e o contrato); as noticias so a original (alocam).
SAVE = False
ROTINAS = [('satisfacao_com_o_contrato', 0x866620, [0x6a, 0xff, 0x68, 0x90, 0x2a, 0x96, 0x00]),
           ('noticia_quer_contrato_novo', 0x4d9f80, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_pedido_de_transferencia_retirado', 0x4d9460, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a satisfação do jogador com o contrato, a notícia de que ele quer contrato novo e a do pedido de transferência retirado',
        'a player\'s satisfaction with his contract, the news that he wants a new contract and of a withdrawn transfer request',
        [('satisfacao_com_o_contrato', 'get_contract_satisfaction', '0x866620', 'staff_contracts.cpp', 'thiscall',
          'de 1 a 3: 100 × o valor do contrato atual / o do desejado (com a situação merecida), com o desconto de quem chegou há pouco; abaixo de 50: 1; abaixo de 80: 3; senão 2',
          '1 to 3: 100 × the value of the current contract / that of the desired one (with the deserved squad status), with an allowance for recent signings; below 50: 1; below 80: 3; otherwise 2'),
         ('noticia_quer_contrato_novo', 'post_wants_new_contract_news', '0x4d9f80', 'contract_manager.cpp', 'thiscall',
          'a notícia 0xbc2 de que o jogador quer contrato novo, ao técnico do clube, se ele não estiver na lista de transferências nem em oferta',
          'the 0xbc2 news that the player wants a new contract, to the club manager, unless he is transfer listed or in an offer'),
         ('noticia_pedido_de_transferencia_retirado', 'post_transfer_request_withdrawn_news', '0x4d9460', 'contract_manager.cpp', 'thiscall',
          'a notícia 0xbc0 do pedido de transferência retirado, com o motivo, ao técnico do clube',
          'the 0xbc0 news of a withdrawn transfer request, with the reason, to the club manager')])
CAMPOS = [('contrato +4: clube; id; lê (as três)', 'contract +4: club; id; reads (all three)'),
          ('contrato +0x25 dia, +0x27 ano: início do contrato; valor; lê (satisfacao_com_o_contrato)',
           'contract +0x25 day, +0x27 year: contract start; value; reads (satisfacao_com_o_contrato)'),
          ('contrato +0x4f >> 4: situação no elenco; valor; lê (satisfacao_com_o_contrato, noticia_quer_contrato_novo)',
           'contract +0x4f >> 4: squad status; value; reads (satisfacao_com_o_contrato, noticia_quer_contrato_novo)'),
          ('contrato +0xc salário, +0x2d/+0x2f fim, +0x3f; valor; lê (as notícias)',
           'contract +0xc wage, +0x2d/+0x2f end, +0x3f; value; reads (the news)'),
          ('pessoa +0x59: adaptabilidade; valor; lê (satisfacao_com_o_contrato, noticia_quer_contrato_novo)',
           'person +0x59: adaptability; value; reads (satisfacao_com_o_contrato, noticia_quer_contrato_novo)'),
          ('pessoa +0x5b; ficha +0xd: reputação; valor; lê (noticia_quer_contrato_novo)',
           'person +0x5b; player record +0xd: reputation; value; reads (noticia_quer_contrato_novo)'),
          ('[0xae2c90] hoje; valor; lê (satisfacao_com_o_contrato)', '[0xae2c90] today; value; reads (satisfacao_com_o_contrato)')]
INTERESSA = ['CMRTE']
