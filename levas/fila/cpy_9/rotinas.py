# Os árbitros do jogo novo (officials_manager.cpp), as três pela montagem dos árbitros OfficialsManager::set_up_officials
# 0x79bf60 (construtor 0x79b580 sem save, na criação do jogo, que a carreira automática faz): officials_manager_add_
# random_officials 0x79c5c0 (nas nações sem liga com menos de 5 árbitros e nas que precisam de mais para as ligas; a
# nota da 0x53a0e0 no motor.rs já registra a chamada 0x79c6b4 na criação), officials_manager_validate_officials 0x79caa0
# (a primeira coisa da montagem, todas as fichas) e Official::operator= 0x79c530 (as fichas que ficam). Iguais no
# cm0102-1080.exe; sem SEH; os quadros de 0x200 e 0x10 são da caixa de erro e da data local. Comparadores: a 0x79c5c0
# realoca e ordena a tabela (não pode rodar duas vezes): só a original, depois de um ensaio da cópia (as fichas geradas
# numa área própria, a partir do mesmo estado do sorteio, que é reposto), conferindo o estado final do sorteio, a conta
# nova e cada ficha do ensaio na tabela; a 0x79caa0 lado a lado pelas fichas e pelo sorteio; a 0x79c530 lado a lado pela
# ficha de destino.
# Convenções conferidas nos pontos de chamada das rotinas chamadas por endereço:
# - 0x539b90 primeiro_nome_pelo_peso e 0x53a0e0 segundo_nome_pelo_peso: `push edi; call` cada, sem ajuste logo depois; o
#   `add esp, 0x38` em 0x79c766 desfaz os 2 nomes + os 12 sorteios (14 × 4) = cdecl(nacao).
# - 0x90cfc0 sortear: `push n; call` (ajustes juntos: 0x38 na 0x79c5c0; `add esp, 4` a cada um na 0x79caa0) = cdecl(n).
# - 0x945667 realloc: `push tamanho; push tabela; call; add esp, 8` = cdecl(tabela, tamanho) (0x79c5fc..0x79c608).
# - 0x945333 qsort: 4 push e `add esp, 0x10` = cdecl(base, n, 0x2b, 0x79d480) (0x79c78c..0x79c7a0).
# - 0x549f70 data_dma: `push -1; push 0x776; push 1; push 1; lea ecx, [esp+0x20]; call` sem ajuste = thiscall(data, 1,
#   1, 0x776, −1) (0x79cd99).
# - 0x417c10 copia: `push eax; push ecx; lea ecx, [esi+0xc]; call` sem ajuste = thiscall(destino, temporária, origem)
#   (0x79c54f).
# - Erro sem memória da 0x79c5c0 pelo `erro_em` do motor.rs (0x5cc7d0, 0x5a8f60, 0x945934 cdecl, como lá).
# - Os pontos de chamada: 0x79c5c0 em 0x79c2d5/0x79c328 com `mov ecx, esi` e 2 push (o corpo não lê o ecx; ret 8):
#   thiscall com o this sem uso; 0x79caa0 em 0x79bf6c sem push nem ajuste (cdecl sem argumentos); 0x79c530 em 0x79c04e
#   com `mov ecx, ebp` e 1 push (thiscall, ret 4).
SAVE = False
ROTINAS = [('arbitros_sorteados', 0x79c5c0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('validar_arbitros', 0x79caa0, [0x8b, 0x0d, 0x90, 0x23, 0xae, 0x00]),
           ('copiar_ficha_de_arbitro', 0x79c530, [0x83, 0xec, 0x08, 0x56, 0x57])]
DOCS = ('os árbitros do jogo novo: os árbitros sorteados de uma nação, o conserto das fichas dos árbitros e a cópia de uma ficha',
        'the referees of a new game: the random referees of a nation, the repair of the referee records and the copy of a record',
        [('arbitros_sorteados', 'officials_manager_add_random_officials', '0x79c5c0', 'officials_manager.cpp', 'thiscall (this sem uso), ret 8',
          'realoca a tabela dos árbitros, gera `conta` árbitros novos da nação (nomes pelo peso, notas e atributos sorteados, nascidos nos anos 1960) e ordena a tabela por nação e qualidade',
          'reallocates the referee table, generates `count` new referees of the nation (names by weight, random ratings and attributes, born in the 1960s) and sorts the table by nation and quality'),
         ('validar_arbitros', 'officials_manager_validate_officials', '0x79caa0', 'officials_manager.cpp', 'cdecl',
          'conserta cada ficha de árbitro: a nação pelos nomes, as notas e os atributos fora da faixa sorteados de novo e a data de nascimento antiga demais',
          'repairs each referee record: the nation from the names, out-of-range ratings and attributes drawn again and a too old birth date'),
         ('copiar_ficha_de_arbitro', 'Official::operator=', '0x79c530', 'officials_manager.cpp', 'thiscall, ret 4',
          'copia uma ficha de árbitro de 0x2b bytes campo a campo, a data pela cópia de data',
          'copies a 0x2b-byte referee record field by field, the date through the date copy')])
CAMPOS = [('árbitro (0x2b bytes em [0xae23e8], [0xae2390] deles) +0 id, +4 e +8 nomes (registro de nome +0x37 índice da nação), +0xc dia e +0xe ano de nascimento, +0x16 nação, +0x1a, +0x1e, +0x20 e +0x22 notas (palavras; a ordem usa +0x28 × 50 + +0x22), +0x24..+0x2a atributos (bytes 1..20); ponteiros/valores; grava as fichas novas, a conta e a tabela (arbitros_sorteados), lê e grava (validar_arbitros, copiar_ficha_de_arbitro)',
           'referee (0x2b bytes at [0xae23e8], [0xae2390] of them) +0 id, +4 and +8 names (name record +0x37 nation index), +0xc birth day and +0xe year, +0x16 nation, +0x1a, +0x1e, +0x20 and +0x22 ratings (words; the order uses +0x28 × 50 + +0x22), +0x24..+0x2a attributes (bytes 1..20); pointers/values; writes the new records, the count and the table (arbitros_sorteados), reads and writes (validar_arbitros, copiar_ficha_de_arbitro)'),
          ('nação (0x122 bytes em [0xae23a8]) +0x71 continente, +0x8e nível dos árbitros (byte); [0xae2350] nações; ponteiros/valores; lê',
           'nation (0x122 bytes at [0xae23a8]) +0x71 continent, +0x8e referee level (byte); [0xae2350] nations; pointers/values; reads')]
INTERESSA = ['CMRTE', 'GPF2']
