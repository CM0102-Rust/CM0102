# A mudança de divisão e os embaralhamentos (league.cpp, comp_util.cpp): mudar o clube de divisão na subida e na
# descida (League::move_club_to_division 0x6830b0, chamada direta por 33 pontos: a cadeia 0x689c80, já em Rust, e as
# ligas de cada país; no fim da temporada), embaralhar um vetor de ponteiros (comp_util_shuffle_pointer_array 0x4afb40,
# por 45 pontos da montagem das competições) e ordenar pela reputação e embaralhar os melhores
# (comp_util_sort_and_shuffle_top_clubs 0x4afbd0, 4 chamadas na carreira até 2002-06-01). A 0x6830b0 chama as
# cláusulas (0x4cd120), a vaga de técnico (0x69ba50) e a obra do estádio (0x599e00), originais; as duas que embaralham
# sorteiam (0x90cfc0) e a 0x4afbd0 ordena com o qsort da CRT (0x945333, com a 0x4b07d0). Sem SEH; a caixa só na
# 0x4afbd0 com contas erradas (a cópia chama a original). Iguais no cm0102-1080.exe. Simulação: as mutações pela
# carreira e a confirmação no jogo real comparando. Comparadores: a mudança só pela original (efeitos que não podem
# rodar duas vezes); os embaralhamentos lado a lado pelo estado do sorteio e pelo vetor, a cópia em without_counting.
SAVE = False
ROTINAS = [('mudar_clube_de_divisao', 0x6830b0, [0x8b, 0x0d, 0xf0, 0x7e, 0xdd, 0x00]),
           ('embaralhar_ponteiros', 0x4afb40, [0x56, 0x57, 0x0f, 0xbf, 0x74, 0x24, 0x10]),
           ('ordenar_e_embaralhar_os_melhores', 0x4afbd0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a mudança de divisão do clube e os embaralhamentos das listas de clubes',
        'the club division move and the shuffles of the club lists',
        [('mudar_clube_de_divisao', 'League::move_club_to_division', '0x6830b0', 'league.cpp', 'thiscall, ret 0xc',
          'a divisão atual vira a da última temporada e a nova entra; limpa as cláusulas de divisão (e os números da camisa se a nova os usa), abre a vaga de técnico pedida, faz a obra do estádio mínimo da liga e passa a situação 3 a 2',
          'the current division becomes last season\'s and the new one is set; clears the division clauses (and the squad numbers when the new one uses them), opens the requested manager vacancy, upgrades the stadium to the league minimum and turns status 3 into 2'),
         ('embaralhar_ponteiros', 'comp_util_shuffle_pointer_array', '0x4afb40', 'comp_util.cpp', 'cdecl',
          'embaralha um vetor de n ponteiros com 3 × n trocas de pares sorteados',
          'shuffles an array of n pointers with 3 × n swaps of drawn pairs'),
         ('ordenar_e_embaralhar_os_melhores', 'comp_util_sort_and_shuffle_top_clubs', '0x4afbd0', 'comp_util.cpp', 'cdecl',
          'ordena os clubes pela reputação (nulos no fim) e embaralha os min(3 × escolhas, n) primeiros',
          'sorts the clubs by reputation (nulls last) and shuffles the first min(3 × picks, n)')])
CAMPOS = [('clube +0x57/+0x5b: divisão e divisão da última temporada; ponteiro; lê e grava (mudar_clube_de_divisao)',
           'club +0x57/+0x5b: division and last season\'s division; pointer; reads and writes (mudar_clube_de_divisao)'),
          ('clube +0x64: situação (3 passa a 2); valor; lê e grava (mudar_clube_de_divisao)',
           'club +0x64: status (3 becomes 2); value; reads and writes (mudar_clube_de_divisao)'),
          ('clube +0xcf, +0x69: o técnico e a ficha do estádio; ponteiro; lê (mudar_clube_de_divisao)',
           'club +0xcf, +0x69: the manager and the stadium record; pointer; reads (mudar_clube_de_divisao)'),
          ('competição +0x43, +0xe2, +0xe4: o tipo (2 = liga) e o estádio mínimo (lugares, capacidade); valor; lê (mudar_clube_de_divisao)',
           'competition +0x43, +0xe2, +0xe4: the type (2 = league) and the minimum stadium (seats, capacity); value; reads (mudar_clube_de_divisao)'),
          ('[0xadadfc] as competições carregadas pelo id; ponteiro; lê (mudar_clube_de_divisao)',
           '[0xadadfc] the loaded competitions by id; pointer; reads (mudar_clube_de_divisao)'),
          ('clube +0x80: reputação (a ordem do qsort); valor; lê (ordenar_e_embaralhar_os_melhores, pela 0x4b07d0)',
           'club +0x80: reputation (the qsort order); value; reads (ordenar_e_embaralhar_os_melhores, through 0x4b07d0)')]
INTERESSA = ['CMRTE', 'GPF2']
