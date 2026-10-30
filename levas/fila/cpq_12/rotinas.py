# O lado do jogo, a ordem dos coeficientes e o pedido de observação (comp_util.cpp, scout_manager.cpp): o lado do
# clube no jogo (comp_util_get_fixture_venue_side 0x4afa80: casa, fora ou neutro, pela ficha de jogo da
# record_utils 0x7e4895 e pelo olheiro da busca 0x7bd567, além das telas), a ordem do qsort dos coeficientes das
# nações (compare_nation_coefficients_desc 0x4afba0, na game_update_nation_coefficients 0x5d6187, uma vez por mês;
# a original libera o vetor ordenado logo depois) e se o clube pediu a observação do jogador
# (SCOUT_MANAGER::has_scout_request 0x7f8d70, pela busca 0x7b5be0 a cada jogo observado; os pedidos só vêm das telas
# do humano). Sem sorteio e sem efeitos: as três são consultas. A 0x7f8d70 tem quadro SEH só por uma data local
# (0x54a110, sem destrutor chamado); jogador nulo: a cópia chama a original (caixa da linha 0x5da). A 0x4afa80
# devolve um char (quem chama só lê o al). Iguais no cm0102-1080.exe. Comparadores: os três pelo valor.
SAVE = False
ROTINAS = [('lado_do_jogo_para_o_clube', 0x4afa80, [0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('ordem_dos_coeficientes_das_nacoes', 0x4afba0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x54, 0x24, 0x08]),
           ('pedido_de_observacao', 0x7f8d70, [0x6a, 0xff, 0x68, 0xfb, 0x0f, 0x96, 0x00])]
DOCS = ('o lado do jogo, a ordem dos coeficientes das nações e o pedido de observação',
        'the fixture side, the nation coefficient order and the scout request',
        [('lado_do_jogo_para_o_clube', 'comp_util_get_fixture_venue_side', '0x4afa80', 'comp_util.cpp', 'cdecl',
          'o lado do clube no jogo: 1 em casa, 2 fora, 3 neutro, pelo estádio do jogo e dos dois clubes (seleções: pela nação do estádio)',
          'the club side in the fixture: 1 home, 2 away, 3 neutral, by the fixture stadium and the two clubs\' stadiums (national sides: by the stadium nation)'),
         ('ordem_dos_coeficientes_das_nacoes', 'compare_nation_coefficients_desc', '0x4afba0', 'comp_util.cpp', 'cdecl',
          'a ordem do qsort dos coeficientes das nações: o maior antes',
          'the qsort order of the nation coefficients: highest first'),
         ('pedido_de_observacao', 'SCOUT_MANAGER::has_scout_request', '0x7f8d70', 'scout_manager.cpp', 'thiscall, ret 0xc',
          'se o clube (ou o do humano da vez) pediu a observação do jogador, com menos de 3 relatórios e, se pedido, com a data já vencida',
          'whether the club (or the current human\'s) asked to scout the player, with fewer than 3 reports and, if asked, with the date already due')])
CAMPOS = [('fixture +0x18: o estádio do jogo (0 sem estádio); +0x1c/+0x20 mandante e visitante; ponteiros; lê (lado_do_jogo_para_o_clube)',
           'fixture +0x18: the fixture stadium (0 without one); +0x1c/+0x20 home and away clubs; pointers; reads (lado_do_jogo_para_o_clube)'),
          ('club +0x69: o estádio; club +0x53: a nação; estádio +0x38 → nação +0x1f; ponteiros; lê (lado_do_jogo_para_o_clube)',
           'club +0x69: the stadium; club +0x53: the nation; stadium +0x38 → nation +0x1f; pointers; reads (lado_do_jogo_para_o_clube)'),
          ('vetor temporário da 0x5d5f20 (12 bytes: +0 nação, +4 coeficiente em double); lê (ordem_dos_coeficientes_das_nacoes)',
           '0x5d5f20 temporary array (12 bytes: +0 nation, +4 coefficient as a double); reads (ordem_dos_coeficientes_das_nacoes)'),
          ('SCOUT_MANAGER +0x5c, +0x64: os pedidos de observação (17 bytes: +0 id do clube, +4 id do jogador, +8 data, +0x10 relatórios) e a conta; lê (pedido_de_observacao)',
           'SCOUT_MANAGER +0x5c, +0x64: the scout requests (17 bytes: +0 club id, +4 player id, +8 date, +0x10 reports) and the count; reads (pedido_de_observacao)'),
          ('[0xb74c26 + 0x300 × [0xb77c7a]] +0x39: o clube do humano da vez; [0xae2c90] hoje; lê (pedido_de_observacao)',
           '[0xb74c26 + 0x300 × [0xb77c7a]] +0x39: the current human\'s club; [0xae2c90] today; reads (pedido_de_observacao)')]
INTERESSA = ['CMScout', 'CMRTE']
