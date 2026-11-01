# A situacao da pessoa nas listas de observacao dos clubes, a marca de lista de emprestimos e o teto de preco pelo
# estatuto (shortlist_manager.cpp): Shortlist_Manager::update_staff_status 0x843e10 (4599 chamadas ate 2002-06-01 na
# cobertura de 17/10; quadro SEH so da LLIST local, construtor 0x68d020 e destrutor 0x68d040 chamados como a
# original), add_to_loan_list 0x8442b0 (telas de transferencia) e max_fee_for_player_status 0x84b1f0 (tela de
# contrato). Convencoes conferidas no ponto de chamada da original: 0x68d020, 0x68d040, 0x68d1b0 thiscall(lista) (lea
# ecx, lista); 0x842180 thiscall(this, pessoa, lista, 0, 3, 0) (mov ecx, ebx; 5 push; ret 0x14); a recursao 0x843e10
# thiscall(this, pessoa, clube, 1, 2) (mov ecx, ebx; 4 push); 0x946464 cdecl(chave, vetor, conta, 8, comparacao)
# (add esp, 0x14); 0x600a50 cdecl(clube, 0, 1, 0, 0) (add esp, 0x14); 0x4cf5f0 thiscall([0xae1050], pessoa, clube)
# (mov ecx, [0xae1050]; 2 push); 0x83f3a0 thiscall(this, vaga, clube, pessoa) (mov ecx, ebx; 3 push); 0x54c5b0 e
# 0x54c8e0 thiscall(0xae2c90, saida, dias) (mov ecx, 0xae2c90; 2 push); 0x417c10 thiscall(vaga + 0x1b, saida,
# origem) (lea ecx, [vaga + 0x1b]; 2 push); 0x4cf5a0 thiscall([0xae1050], pessoa) (1 push); 0x5b7d20 cdecl(ficha)
# (add esp, 4); 0x84b230 com mov ecx, esi e 3 push (clube, posicoes, estatuto): chamada como thiscall (a copia stdcall
# do motor nao le o ecx). As desviadas: a 0x843e10 e a 0x84b1f0 sao thiscall (ecx = o Shortlist_Manager, lido em
# +0/+4/+0xdc na primeira e passado adiante na segunda); a 0x8442b0 e stdcall (troca o ecx por [0xae1050] antes de
# ler). Iguais no cm0102-1080.exe. Comparadores: a 0x843e10 lado a lado (os dois vetores de pares e as contas, o byte
# da conta de listas da pessoa, as 40 vagas e o +8 da entrada, o +0x4f do contrato, as datas estaticas) e a devolucao;
# na recursao do evento 4 e no evento 1 com a pessoa no alvo (a 0x83f3a0 sorteia, acrescenta pares com realocacao e da
# noticias), so a original. A 0x8442b0 lado a lado pelo +0x4e do contrato; a 0x84b1f0 pela devolucao.
SAVE = False
ROTINAS = [('situacao_na_lista_de_observacao', 0x843e10, [0x6a, 0xff, 0x68, 0x18, 0x25, 0x96, 0x00]),
           ('por_na_lista_de_emprestimos', 0x8442b0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00]),
           ('teto_de_preco_pelo_estatuto', 0x84b1f0, [0x8b, 0x44, 0x24, 0x08, 0x56])]
DOCS = ('a situação da pessoa nas listas de observação dos clubes, a marca de lista de empréstimos e o teto de preço pelo estatuto',
        'a person\'s status in the clubs\' shortlists, the loan list flag and the fee limit by squad status',
        [('situacao_na_lista_de_observacao', 'Shortlist_Manager::update_staff_status', '0x843e10', 'shortlist_manager.cpp', 'thiscall',
          'aplica um evento de transferência da pessoa à lista do clube: no evento 4 do próprio clube, o evento 1 a cada clube interessado; o par (pessoa, clube) sai dos vetores ordenados e a conta de listas da pessoa cai; nas vagas com a pessoa, 1 refaz a vaga, 2 e 5 bloqueiam por 120 e 14 dias, 3 e 4 soltam com a data de hoje − 120',
          'applies a transfer event for the person to the club\'s shortlist: for event 4 at his own club, event 1 to every interested club; the (person, club) pair leaves the sorted arrays and the person\'s shortlist count drops; in the slots holding the person, 1 refills the slot, 2 and 5 block for 120 and 14 days, 3 and 4 release with today − 120'),
         ('por_na_lista_de_emprestimos', 'shortlist_manager_add_to_loan_list', '0x8442b0', 'shortlist_manager.cpp', 'stdcall',
          'liga o bit 1 (lista de empréstimos) de +0x4e do contrato da pessoa',
          'sets bit 1 (loan listed) of the person\'s contract +0x4e'),
         ('teto_de_preco_pelo_estatuto', 'shortlist_manager_max_fee_for_player_status', '0x84b1f0', 'shortlist_manager.cpp', 'thiscall',
          'o teto de preço do clube para o jogador no estatuto (0x84b230 com as posições fortes); −1 sem pessoa ou sem ficha de jogador',
          'the club\'s fee limit for the player at the squad status (0x84b230 with his strong positions); −1 without a person or player record')])
CAMPOS = [('Shortlist_Manager +0/+0x10 e +4/+0x14: vetores ordenados de pares (pessoa, clube) de 8 bytes e contas; ponteiro e valor; lê e grava (situacao_na_lista_de_observacao)',
           'Shortlist_Manager +0/+0x10 and +4/+0x14: sorted 8-byte (person, club) pair arrays and counts; pointer and value; reads and writes (situacao_na_lista_de_observacao)'),
          ('Shortlist_Manager +0xdc: entrada de 9 bytes por clube (+0 vagas, +8 grava 100); ponteiro; lê e grava (situacao_na_lista_de_observacao)',
           'Shortlist_Manager +0xdc: 9-byte entry per club (+0 slots, +8 writes 100); pointer; reads and writes (situacao_na_lista_de_observacao)'),
          ('vaga da lista (0x25 bytes) +0 tipo, +7 e +0xb valores, +0xf alvo, +0x13 pessoa bloqueada, +0x17 lance, +0x1b data, +0x24 estatuto e marca; lê e grava (situacao_na_lista_de_observacao)',
           'shortlist slot (0x25 bytes) +0 type, +7 and +0xb values, +0xf target, +0x13 blocked person, +0x17 bid, +0x1b date, +0x24 status and flag; reads and writes (situacao_na_lista_de_observacao)'),
          ('[0xae2d0c] registro de 79 bytes por pessoa +0xb: nibble baixo = em quantas listas; valor; grava −1 (situacao_na_lista_de_observacao)',
           '[0xae2d0c] 79-byte record per person +0xb: low nibble = on how many shortlists; value; writes −1 (situacao_na_lista_de_observacao)'),
          ('contrato +0x4f: nibble alto = estatuto no elenco; valor; grava (situacao_na_lista_de_observacao)',
           'contract +0x4f: high nibble = squad status; value; writes (situacao_na_lista_de_observacao)'),
          ('contrato +0x4e: bit 1 lista de empréstimos; valor; grava (por_na_lista_de_emprestimos)',
           'contract +0x4e: bit 1 loan listed; value; writes (por_na_lista_de_emprestimos)'),
          ('pessoa +0x39 clube, +0x61 ficha de jogador; ponteiro; lê (situacao_na_lista_de_observacao, teto_de_preco_pelo_estatuto)',
           'person +0x39 club, +0x61 player record; pointer; reads (situacao_na_lista_de_observacao, teto_de_preco_pelo_estatuto)')]
INTERESSA = ['CMRTE', 'CMScout', 'SGE']
