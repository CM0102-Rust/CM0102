# Os amistosos internacionais do dia (Friendly::update_international_friendlies 0x5cbb00, friendly.cpp, 515 chamadas
# ate 2002-06-01 na cobertura de 17/10, pelo dia dos amistosos 0x5c2460 ja em Rust; quadro SEH so da data local, de
# destrutor vazio) e a lista escolhida do olheiro do tecnico humano (SCOUT_MANAGER::get_selected_list 0x7f6e30 e
# set_selected_list 0x7f6f20, scout_manager.cpp, pelas telas de busca). Familia: das rotinas de shortlist_manager,
# player_search e scout_manager que rodam na carreira sem humano, as livres sao grandes (0x83cb40 3 KB, 0x84a020 3 KB,
# 0x842e10 2,5 KB, 0x840d80 4,6 KB) ou ja estao nos itens cpy_1..3; a 0x84abe0 so roda com tecnico humano e a 0x843800
# nao rodou na cobertura. A 0x5cbb00 vem de friendly.cpp (fora das familias desta rodada). Convencoes conferidas no
# ponto de chamada da original: 0x54a110 thiscall(data) (lea ecx); 0x54c770 thiscall(0xae2c90, saida, 0xc) (mov ecx,
# 0xae2c90; 2 push); 0x68d080 thiscall(lista) (mov ecx, [lista]; sem push); 0x945333 cdecl(vetor, conta, tamanho,
# comparacao) (add esp, 0x10); 0x5cbdd0 thiscall(entrada, ano) (o ecx e a entrada; 1 push); 0x417c10
# thiscall(destino, saida, origem) (lea ecx, destino; 2 push); 0x946464 cdecl(chave, vetor, conta, 0x14, 0x5cc610)
# (add esp, 0x14); 0x456770 e 0x456740 thiscall(data, baixo, alto) (lea ecx, data; 2 push); 0x5cbdf0 thiscall(this,
# primeira, quantas) (mov ecx, esi; 2 push). As desviadas sao thiscall (ecx = o Friendly, lido em +0x34ec..+0x34fe; ecx
# = o SCOUT_MANAGER, lido em +0x4c). Iguais no cm0102-1080.exe. Na virada do ano a original soma o deslocamento da
# entrada a um dword cuja metade alta e sobra de pilha (so a metade baixa e o ano; a 0x54a130 le palavras): a copia usa
# zero e o comparador manda o dia para a original sozinha. Comparadores: a 0x5cbb00 lado a lado (o dia marcado e as
# datas estaticas) nos dias sem amistosos a marcar; no primeiro dia do ano (as listas liberam) e quando ha amistosos (a
# 0x5cbdf0 cria jogos e sorteia), so a original, conferindo o dia marcado; as duas listas escolhidas pela devolucao e
# lado a lado pelos 16 bytes; os casos de erro (caixa "Error") so a original.
SAVE = False
ROTINAS = [('amistosos_internacionais_do_dia', 0x5cbb00, [0x6a, 0xff, 0x68, 0xa0, 0xa3, 0x95, 0x00]),
           ('lista_escolhida_do_tecnico', 0x7f6e30, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('escolher_lista_do_tecnico', 0x7f6f20, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os amistosos internacionais do dia e a lista escolhida do olheiro do técnico humano',
        'the day\'s international friendlies and the human manager\'s selected scout list',
        [('amistosos_internacionais_do_dia', 'Friendly::update_international_friendlies', '0x5cbb00', 'friendly.cpp', 'thiscall',
          'no primeiro dia do ano esvazia as listas de nações, ordena as nações pela reputação e dá o ano às datas internacionais pelo ciclo da Copa; depois marca os amistosos das datas iguais a hoje + 12 semanas (0x5cbdf0), uma vez por dia',
          'on the first day of the year empties the nation lists, sorts the nations by reputation and gives the international dates their year by the World Cup cycle; then arranges the friendlies of the dates equal to today + 12 weeks (0x5cbdf0), once a day'),
         ('lista_escolhida_do_tecnico', 'SCOUT_MANAGER::get_selected_list', '0x7f6e30', 'scout_manager.cpp', 'thiscall',
          'a lista escolhida pelo técnico humano (nulo: o da vez)',
          'the list selected by the human manager (null: the current one)'),
         ('escolher_lista_do_tecnico', 'SCOUT_MANAGER::set_selected_list', '0x7f6f20', 'scout_manager.cpp', 'thiscall',
          'grava a lista escolhida pelo técnico humano (nulo: o da vez)',
          'stores the list selected by the human manager (null: the current one)')])
CAMPOS = [('Friendly +0x34ec/+0x34f0: datas internacionais (20 bytes: +0 dia da semana, +1 dia, +2 mês, +3 data, +0xb deslocamento do ano) e conta; lê e grava a data (amistosos_internacionais_do_dia)',
           'Friendly +0x34ec/+0x34f0: international dates (20 bytes: +0 weekday, +1 day, +2 month, +3 date, +0xb year offset) and count; reads and writes the date (amistosos_internacionais_do_dia)'),
          ('Friendly +0x34f4: último dia feito (palavra); valor; lê e grava (amistosos_internacionais_do_dia)',
           'Friendly +0x34f4: last day done (word); value; reads and writes (amistosos_internacionais_do_dia)'),
          ('Friendly +0x34f6 listas de nações ([0xae2350] delas), +0x34fa/+0x34fe nações e conta; lê, esvazia e ordena (amistosos_internacionais_do_dia)',
           'Friendly +0x34f6 nation lists ([0xae2350] of them), +0x34fa/+0x34fe nations and count; reads, empties and sorts (amistosos_internacionais_do_dia)'),
          ('[0xdd6ec8] data-limite dos amistosos; valor; lê (amistosos_internacionais_do_dia)',
           '[0xdd6ec8] friendlies limit date; value; reads (amistosos_internacionais_do_dia)'),
          ('SCOUT_MANAGER +0x4c: lista escolhida por técnico humano (16 bytes); valor; lê e grava (lista_escolhida_do_tecnico, escolher_lista_do_tecnico)',
           'SCOUT_MANAGER +0x4c: selected list per human manager (16 bytes); value; reads and writes (lista_escolhida_do_tecnico, escolher_lista_do_tecnico)')]
INTERESSA = ['GPF2']
