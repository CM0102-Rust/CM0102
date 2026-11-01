# A verba de transferencias repartida pelas vagas da lista de observacao (Shortlist_Manager::allocate_transfer_budget
# 0x83b6e0, 17425 chamadas ate 2002-06-01 na cobertura de 17/10, pela atualizacao da lista do clube 0x83a730 ja em
# Rust), o estatuto permitido pela idade (shortlist_manager_squad_status_allowed 0x84b4d0, telas de contrato e de
# transferencia) e o nome das listas do olheiro (scout_manager_write_list_name 0x7fc150, construtor do SCOUT_MANAGER e
# telas de busca; 160 chamadas). Convencoes conferidas no ponto de chamada da original: 0x600a50 cdecl(clube, 0, 1, 0,
# 0) (add esp, 0x14); 0x53b900 cdecl(clube) (add esp, 4); 0x5a0590 thiscall([0xae2a38], clube) (mov ecx, [0xae2a38]; 1
# push); 0x598800 thiscall(financas, 0, 0, 0, 1, 0) (mov ecx, eax; 5 push); 0x945333 cdecl(vetor, 0x28, 0x25,
# 0x83b690) (add esp, 0x10); 0x8c61e0 thiscall([0xde1f54], lance) (1 push); 0x83f3a0 thiscall(this, vaga, clube, 0)
# (mov ecx, [esp+0x3c] = o this guardado; 3 push); 0x4cf5a0 thiscall([0xae1050], pessoa) (o resultado vai a
# [0xae1080]); 0x544590 cdecl(pessoa, 0) (add esp, 8); 0x4cf5f0 thiscall([0xae1050], pessoa, clube) (2 push); 0x846920
# thiscall(this, pessoa, clube, 1, 0, estatuto, 0) (mov ecx, [esp+0x38]; 6 push; o float volta em st0); 0x90cfc0
# cdecl(n) (add esp, 4); 0x85fa00 thiscall(contrato) (mov ecx, esi; sem push; movsx ax); 0x8c5cf0
# thiscall([0xde1f54], pessoa, 0, 0) (3 push); 0x54c8e0 thiscall(0xae2c90, saida, dias) (2 push); 0x66f4e0
# cdecl(texto, formato, ...) (add esp, 8 ou 0xc). As desviadas: a 0x83b6e0 thiscall (ecx = o Shortlist_Manager, lido
# em +0xdc); a 0x84b4d0 stdcall (chamada com ecx, que ela troca por [esp+4] antes de ler); a 0x7fc150 cdecl (ret sem
# numero; quem chama limpa). Iguais no cm0102-1080.exe. x87 em precisao dupla (as contas na ordem fild, fidiv, fimul,
# fmul e a _ftol 0x945640 pela ftol). Comparadores: a 0x83b6e0 so a original (a 0x83f3a0 sorteia, acrescenta pares
# com realocacao e da noticias); a 0x84b4d0 pela devolucao; a 0x7fc150 lado a lado pelo texto; os casos de erro (caixa
# "Error") so a original.
SAVE = False
ROTINAS = [('distribuir_verba_de_transferencias', 0x83b6e0, [0x83, 0xec, 0x2c, 0x53, 0x55]),
           ('estatuto_permitido', 0x84b4d0, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nome_da_lista_do_olheiro', 0x7fc150, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a verba de transferências repartida pelas vagas da lista de observação, o estatuto permitido pela idade e o nome das listas do olheiro',
        'the transfer budget shared across the shortlist slots, the squad status allowed by age and the scout list names',
        [('distribuir_verba_de_transferencias', 'Shortlist_Manager::allocate_transfer_budget', '0x83b6e0', 'shortlist_manager.cpp', 'thiscall',
          'reparte a verba de transferências do clube do computador pelas vagas abertas da lista (fatores do tipo e do estatuto), reduz pelos lances guardados e pela sobra, preenche cada vaga sem alvo e dá a sobra à primeira vaga de jogador do próprio clube que o técnico venderia',
          'shares an AI club\'s transfer budget across the open shortlist slots (type and status factors), reduces it by held bids and the remainder, fills each slot without a target and gives the remainder to the first own-club player slot the manager would sell'),
         ('estatuto_permitido', 'shortlist_manager_squad_status_allowed', '0x84b4d0', 'shortlist_manager.cpp', 'stdcall',
          'os estatutos 5 e 6 (jovens) só abaixo de 24 anos; os outros, sempre',
          'statuses 5 and 6 (youth) only below age 24; the others, always'),
         ('nome_da_lista_do_olheiro', 'scout_manager_write_list_name', '0x7fc150', 'scout_manager.cpp', 'cdecl',
          'o nome traduzido da lista do olheiro: Short List, Player Search, Staff Search ou Scout #n',
          'the translated scout list name: Short List, Player Search, Staff Search or Scout #n')])
CAMPOS = [('Shortlist_Manager +0xdc: entrada de 9 bytes por clube (+0 vagas, +4 estado 1); ponteiro; lê (distribuir_verba_de_transferencias)',
           'Shortlist_Manager +0xdc: 9-byte entry per club (+0 slots, +4 state 1); pointer; reads (distribuir_verba_de_transferencias)'),
          ('vaga da lista (0x25 bytes) +0 tipo, +3, +7 verba, +0xb valor, +0xf alvo, +0x13, +0x17 lance, +0x24 estatuto e marca; lê e grava (distribuir_verba_de_transferencias)',
           'shortlist slot (0x25 bytes) +0 type, +3, +7 budget, +0xb value, +0xf target, +0x13, +0x17 bid, +0x24 status and flag; reads and writes (distribuir_verba_de_transferencias)'),
          ('clube +0x64: 1 = clube rico (vagas de não jogador com verba); valor; lê (distribuir_verba_de_transferencias)',
           'club +0x64: 1 = rich club (non-player slots get budget); value; reads (distribuir_verba_de_transferencias)'),
          ('pessoa +0x52 valor, +0x39 clube, +0x18 idade, +0x61 ficha; lê (distribuir_verba_de_transferencias, estatuto_permitido)',
           'person +0x52 value, +0x39 club, +0x18 age, +0x61 record; reads (distribuir_verba_de_transferencias, estatuto_permitido)'),
          ('contrato +0x4f nibble alto estatuto, +0x25/+0x27 data; lê (distribuir_verba_de_transferencias)',
           'contract +0x4f high nibble status, +0x25/+0x27 date; reads (distribuir_verba_de_transferencias)'),
          ('[0xae1080] último contrato consultado; ponteiro; grava (distribuir_verba_de_transferencias)',
           '[0xae1080] last contract looked up; pointer; writes (distribuir_verba_de_transferencias)')]
INTERESSA = ['CMRTE', 'CMScout']
