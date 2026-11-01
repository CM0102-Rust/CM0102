# As vagas da lista de observacao com lance e as marcas de lista do contrato (shortlist_manager.cpp): a conta das vagas
# com alvo e lance (Shortlist_Manager::count_slots_with_bids 0x844540, pela avaliacao das listas 0x84a020 com 210 mil
# chamadas ate 2002-06-01 na cobertura de 17/10, pela avaliacao do elenco 0x83cb40 e pelo laco do dia 0x5cccd0; a
# 0x84a020 chega a ela: a 0x842740, chamada so por ela depois da conta, rodou 312 vezes), tirar da lista de
# transferencias (0x844280) e da de emprestimos (0x8442d0), pelas telas de transferencia e pela renovacao 0x4dc3a0.
# Convencoes conferidas no ponto de chamada da original: 0x600a50 cdecl(clube, 1, 1, 0, 0) (add esp, 0x14 depois, 5
# push); 0x54c8e0 thiscall(0xae2c90, saida, 0x14) (mov ecx, 0xae2c90; 2 push; sem add esp); 0x8c5cf0
# thiscall([0xde1f54], pessoa, 0, 0) (mov ecx, [0xde1f54]; 3 push); 0x4cf5a0 thiscall([0xae1050], pessoa) (mov ecx,
# [0xae1050]; 1 push). As tres desviadas nao leem o ecx de entrada: a 0x844540 e thiscall (ecx = o Shortlist_Manager,
# lido em +0xdc); a 0x844280 e a 0x8442d0 sao stdcall (o ecx e trocado por [0xae1050] antes de ser lido). Iguais no
# cm0102-1080.exe. Comparadores: as tres lado a lado (a conta pelas 40 vagas do clube e pelas datas estaticas; as
# marcas pelo byte +0x4e do contrato) e a devolucao. O caminho do lance vencido (lista no estado 2 de clube do humano)
# nao roda na carreira sem humano; as mutacoes ficam nas condicoes da conta.
SAVE = False
ROTINAS = [('contar_vagas_com_lance', 0x844540, [0x83, 0xec, 0x14, 0x53, 0x8b, 0x5c, 0x24, 0x1c]),
           ('tirar_da_lista_de_transferencias', 0x844280, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00]),
           ('tirar_da_lista_de_emprestimos', 0x8442d0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00])]
DOCS = ('as vagas da lista de observação com lance e as marcas de lista de transferências e de empréstimos do contrato',
        'the shortlist slots with a bid and the contract\'s transfer list and loan list flags',
        [('contar_vagas_com_lance', 'Shortlist_Manager::count_slots_with_bids', '0x844540', 'shortlist_manager.cpp', 'thiscall',
          'conta as vagas da lista do clube com alvo e lance na posição pedida (0xff: qualquer); com a lista no estado 2 de clube do humano, a vaga cujo técnico chegou há mais de 20 dias e cujo alvo não tem ofertas perde o alvo e o lance',
          'counts the club\'s shortlist slots with a target and a bid at the given position (0xff: any); with the list in state 2 for a human club, a slot whose manager arrived over 20 days ago and whose target has no offers loses its target and bid'),
         ('tirar_da_lista_de_transferencias', 'shortlist_manager_remove_from_transfer_list', '0x844280', 'shortlist_manager.cpp', 'stdcall',
          'tira o bit 0 (lista de transferências) de +0x4e do contrato; 1 com contrato, 0 sem',
          'clears bit 0 (transfer listed) of the contract\'s +0x4e; 1 with a contract, 0 without'),
         ('tirar_da_lista_de_emprestimos', 'shortlist_manager_remove_from_loan_list', '0x8442d0', 'shortlist_manager.cpp', 'stdcall',
          'tira o bit 1 (lista de empréstimos) de +0x4e do contrato; 1 com contrato, 0 sem',
          'clears bit 1 (loan listed) of the contract\'s +0x4e; 1 with a contract, 0 without')])
CAMPOS = [('Shortlist_Manager +0xdc: entrada de 9 bytes por clube (+0 vagas, +4 estado); ponteiro; lê (contar_vagas_com_lance)',
           'Shortlist_Manager +0xdc: 9-byte entry per club (+0 slots, +4 state); pointer; reads (contar_vagas_com_lance)'),
          ('vaga da lista (0x25 bytes) +0 posição, +0xf alvo, +0x17 lance; valor e id; lê, grava −1 no alvo e no lance vencidos (contar_vagas_com_lance)',
           'shortlist slot (0x25 bytes) +0 position, +0xf target, +0x17 bid; value and id; reads, writes −1 to a stale target and bid (contar_vagas_com_lance)'),
          ('clube +0xcf: técnico (+0x3e data de chegada); ponteiro; lê (contar_vagas_com_lance)',
           'club +0xcf: manager (+0x3e arrival date); pointer; reads (contar_vagas_com_lance)'),
          ('contrato +0x4e: bit 0 lista de transferências, bit 1 lista de empréstimos; valor; grava (tirar_da_lista_de_transferencias, tirar_da_lista_de_emprestimos)',
           'contract +0x4e: bit 0 transfer listed, bit 1 loan listed; value; writes (tirar_da_lista_de_transferencias, tirar_da_lista_de_emprestimos)')]
INTERESSA = ['CMRTE', 'CMScout']
