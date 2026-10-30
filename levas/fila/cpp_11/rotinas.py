# Os amistosos internacionais (friendly.cpp): o par de nações que não joga amistoso (Friendly::is_banned_nation_pair
# 0x5cc3e0, chamada pelos amistosos internacionais 0x5cbdf0, 20 vezes na carreira até 2002-06-01, uma por par
# candidato; o vetor dos pares vem da 0x5cc2d0, já em Rust), a ordem das nações pela reputação (o comparador do qsort
# 0x5cc5f0, da lista das nações 0x5ca3e0 e da 0x5cbb00, 515 chamadas) e a dos registros de amistoso pela data (o
# comparador 0x5cc610, da 0x5cbb00 e da 0x5cc510). Sem SEH, sem sorteio, sem escrita. A 0x5cc3e0 tem um resto
# (Austrália, Nova Zelândia, Oceania) que só lê e sempre devolve 0: a cópia não o repete. Iguais no cm0102-1080.exe.
# Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Comparadores: os três pelo valor.
SAVE = False
ROTINAS = [('par_de_nacoes_sem_amistoso', 0x5cc3e0, [0x51, 0x8a, 0x81, 0x06, 0x35, 0x00, 0x00]),
           ('ordem_das_nacoes_pela_reputacao', 0x5cc5f0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x08]),
           ('ordem_dos_amistosos_pela_data', 0x5cc610, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('os amistosos internacionais: o par de nações que não joga amistoso, a ordem das nações pela reputação e a dos amistosos pela data',
        'international friendlies: the nation pair that plays no friendlies, the order of nations by reputation and of friendlies by date',
        [('par_de_nacoes_sem_amistoso', 'Friendly::is_banned_nation_pair', '0x5cc3e0', 'friendly.cpp', 'thiscall, ret 8',
          'devolve 1 quando (a, b), (a, −1) ou (−1, b) está nos pares proibidos (+0x3502, conta +0x3506), senão 0',
          'returns 1 when (a, b), (a, −1) or (−1, b) is among the banned pairs (+0x3502, count +0x3506), else 0'),
         ('ordem_das_nacoes_pela_reputacao', 'compare_nations_by_reputation_desc', '0x5cc5f0', 'friendly.cpp', 'cdecl',
          'comparador do qsort dos ponteiros de nação pela reputação (+0x8e), a maior primeiro',
          'qsort comparator of nation pointers by reputation (+0x8e), highest first'),
         ('ordem_dos_amistosos_pela_data', 'compare_fixture_records_by_date', '0x5cc610', 'friendly.cpp', 'cdecl',
          'comparador do qsort dos registros de amistoso pela data em +3 (0x456770/0x456740), a mais cedo primeiro',
          'qsort comparator of friendly records by the date at +3 (0x456770/0x456740), earliest first')])
CAMPOS = [('amistosos +0x3502: o vetor dos pares proibidos (2 ids de nação por par, −1 = qualquer); ponteiro; lê; +0x3506 a conta (byte com sinal); valor; lê (par_de_nacoes_sem_amistoso)',
           'friendlies +0x3502: the banned pairs array (2 nation ids per pair, −1 = any); pointer; reads; +0x3506 the count (signed byte); value; reads (par_de_nacoes_sem_amistoso)'),
          ('nação +0x8e: a reputação; valor; lê (ordem_das_nacoes_pela_reputacao)',
           'nation +0x8e: the reputation; value; reads (ordem_das_nacoes_pela_reputacao)'),
          ('registro de amistoso (20 bytes) +3: a data (dia, ano); valor; lê (ordem_dos_amistosos_pela_data)',
           'friendly record (20 bytes) +3: the date (day, year); value; reads (ordem_dos_amistosos_pela_data)')]
INTERESSA = ['CMRTE']
