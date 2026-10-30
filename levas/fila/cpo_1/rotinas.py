# A confiança da diretoria (manager_manager.cpp): avaliar o negócio da transferência (MANAGER_MANAGER::assess_transfer_deal
# 0x696570: o valor do jogador para o comprador e o vendedor pela 0x4d0c60, que sorteia, a razão taxa/valor em faixas,
# o gasto líquido em double e o jogador favorito ou rejeitado do clube e do presidente), aplicar a confiança depois da
# transferência (MANAGER_MANAGER::apply_transfer_confidence_changes 0x6978f0, chamada pela 0x8c9390 em cada
# transferência concluída: a confiança +6 e os humores +0x14/+0x16 da linha da diretoria) e resolver o ultimato
# (MANAGER_MANAGER::resolve_board_ultimatum 0x6ac470: tira o clube da lista dos ultimatos e, com o técnico há mais de
# 100 dias, põe a saída na lista). Sem SEH, sem leitura de lixo (as marcas da 0x696570 avaliada antes, +0x1f = 1,
# ficam com os bits dos ponteiros empurrados, como na original: a cópia faz o mesmo). Iguais no cm0102-1080.exe.
# Razões em double contra as constantes do próprio exe (0x968918 0,2; 0x968968 0,5; 0x968950 2; 0x969000 6; 0x968fd0
# 0,125; 0x968f10 8; 0x9689e0 0,25; 0x968938 4; float 0x968f18 0,5). Comparadores: as duas primeiras lado a lado com o
# estado do sorteador (a cópia sem contar); o ultimato só a original com o plano (tira e libera o item da lista).
SAVE = False
ROTINAS = [('avaliar_o_negocio_da_transferencia', 0x696570, [0x83, 0xec, 0x08, 0x89, 0x4c, 0x24, 0x00]),
           ('confianca_da_diretoria_na_transferencia', 0x6978f0, [0x51, 0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x14]),
           ('resolver_o_ultimato_da_diretoria', 0x6ac470, [0x83, 0xec, 0x08, 0x53, 0x56])]
DOCS = ('a confiança da diretoria: o negócio da transferência, a confiança depois da transferência e o ultimato',
        'board confidence: the transfer deal, the confidence after the transfer and the ultimatum',
        [('avaliar_o_negocio_da_transferencia', 'MANAGER_MANAGER::assess_transfer_deal', '0x696570', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'marca o negócio para o comprador e o vendedor pela razão taxa/valor do jogador (faixas por valor acima ou até 250.000, taxa até (reputação × 0,2)² é justa), soma e tira o valor do gasto líquido de cada um e marca o jogador favorito ou rejeitado do clube e do presidente',
          'flags the deal for buyer and seller by the fee/player value ratio (bands for values above or up to 250,000, a fee up to (reputation × 0.2)² is fair), adds and subtracts the value from each one\'s net spend and flags the club\'s and chairman\'s favoured or disliked player'),
         ('confianca_da_diretoria_na_transferencia', 'MANAGER_MANAGER::apply_transfer_confidence_changes', '0x6978f0', 'manager_manager.cpp', 'thiscall, ret 4',
          'muda a confiança (+6, 1..10000) e os humores (+0x14, +0x16) da diretoria do comprador e do vendedor pelas marcas do negócio e tira o jogador das pessoas favoritas do presidente do vendedor',
          'changes the board confidence (+6, 1..10000) and moods (+0x14, +0x16) of the buyer and the seller from the deal flags and removes the player from the seller chairman\'s favoured people'),
         ('resolver_o_ultimato_da_diretoria', 'MANAGER_MANAGER::resolve_board_ultimatum', '0x6ac470', 'manager_manager.cpp', 'thiscall, ret 4',
          'tira o clube da lista dos ultimatos e, se o técnico está há mais de 100 dias no cargo, põe a saída dele na lista (motivo 2, ou 7 sem jogos na tabela) e devolve 1',
          'removes the club from the ultimatum list and, when the manager has been in the job over 100 days, queues his departure (reason 2, or 7 without games in the table) and returns 1')])
CAMPOS = [('MANAGER_MANAGER +0: a tabela da diretoria por clube (0x49 bytes pelo id); ponteiro; lê (as três)',
           'MANAGER_MANAGER +0: the per-club board table (0x49 bytes by id); pointer; reads (all three)'),
          ('diretoria +2: marcas (bit 2 = ultimato cumprido); valor; grava (resolver_o_ultimato_da_diretoria)',
           'board +2: flags (bit 2 = ultimatum carried out); value; writes (resolver_o_ultimato_da_diretoria)'),
          ('diretoria +6: confiança (1..10000); valor; lê e grava (confianca_da_diretoria_na_transferencia)',
           'board +6: confidence (1..10000); value; reads and writes (confianca_da_diretoria_na_transferencia)'),
          ('diretoria +0x14, +0x16: humores (+0x16 não abaixo de −10000); valor; lê e grava (confianca_da_diretoria_na_transferencia)',
           'board +0x14, +0x16: moods (+0x16 not below −10000); value; reads and writes (confianca_da_diretoria_na_transferencia)'),
          ('diretoria +0x38: gasto líquido em transferências; double; lê e grava (avaliar_o_negocio_da_transferencia)',
           'board +0x38: net transfer spend; double; reads and writes (avaliar_o_negocio_da_transferencia)'),
          ('MANAGER_MANAGER +4, +0x10: as listas das saídas pendentes (6 bytes) e dos ultimatos; ponteiro; lê e grava (resolver_o_ultimato_da_diretoria)',
           'MANAGER_MANAGER +4, +0x10: the pending departure (6 bytes) and ultimatum lists; pointer; reads and writes (resolver_o_ultimato_da_diretoria)'),
          ('transferência +8, +0xc, +0x1f: clube comprador, jogador, já avaliada; id e valor; lê',
           'transfer +8, +0xc, +0x1f: buying club, player, already assessed; id and value; reads'),
          ('pessoa +0x39, +0x61, +0x3e/+0x42: clube, ficha do jogador, data de entrada no cargo; ponteiro e valor; lê',
           'person +0x39, +0x61, +0x3e/+0x42: club, player record, job start date; pointer and value; reads'),
          ('clube +0x80, +0xbf, +0xcf, +0x57: reputação, presidente, técnico, divisão; valor e ponteiro; lê',
           'club +0x80, +0xbf, +0xcf, +0x57: reputation, chairman, manager, division; value and pointer; reads'),
          ('finanças +0x165: clube sem confiança da diretoria (1); valor; lê (confianca_da_diretoria_na_transferencia)',
           'finances +0x165: club without board confidence (1); value; reads (confianca_da_diretoria_na_transferencia)')]
INTERESSA = ['CMRTE']
