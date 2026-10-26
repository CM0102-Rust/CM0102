# Do Database.cpp, consultas das telas: as 14 regiões em ordem de nome (0x545dc0, com a sua comparação 0x545e00: o
# filtro de região das telas de busca, search_screens 0x80970c/0x814b7e/0x815d5e) e a pessoa do outro lado de um laço
# de opinião (0x549dd0: aba de transferências do perfil da pessoa, staff_screens 0x87dd80). Simulação (SAVE = False),
# rotinas só de tela. Sem SEH, sem sorteio; as locais de 0xc8 da 0x545e00 têm o primeiro byte zerado pela 0x616f20
# antes de tudo. A 0x545dc0 escreve o vetor global 0xae2104 (14 bytes) e o idioma em [0xae232b]: comparação pela
# memória (lado_a_lado_leve). As chamadas (0x945333, 0x545e00, 0x616f20) são pelo endereço do jogo.
SAVE = False
ROTINAS = [('regioes_pelo_nome', 0x545dc0, [0xb0, 0x01, 0xb9, 0x04, 0x21, 0xae, 0x00]),
           ('ordem_das_regioes_pelo_nome', 0x545e00, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x90, 0x01, 0x00, 0x00]),
           ('pessoa_da_opiniao', 0x549dd0, [0x8b, 0x4c, 0x24, 0x04, 0x33, 0xc0])]
DOCS = ('as regiões em ordem de nome e a pessoa de um laço de opinião',
        'the regions sorted by name and the person of an opinion link',
        [('regioes_pelo_nome', 'get_regions_sorted_by_name', '0x545dc0', 'Database.cpp', 'cdecl',
          'o vetor global das 14 regiões (0xae2104) ordenado pelo nome no idioma pedido (qsort 0x945333 com 0x545e00)',
          'the global array of the 14 regions (0xae2104) sorted by name in the given language (qsort 0x945333 with 0x545e00)'),
         ('ordem_das_regioes_pelo_nome', 'compare_region_names', '0x545e00', 'Database.cpp', 'cdecl',
          'comparação do qsort de regiões pelo nome (0x616f20) sem maiúsculas; a nula: 1; b nula: -1',
          'qsort comparison of regions by name (0x616f20), case-insensitive; null a: 1; null b: -1'),
         ('pessoa_da_opiniao', 'staff_get_opinion_link_staff', '0x549dd0', 'Database.cpp', 'cdecl',
          'a pessoa do outro lado do laço de opinião (registro [0xae2d0c] + id × 79: +0x27/+0x37 ou +0x3b/+0x4b), ou 0',
          'the person at the other end of the opinion link (record [0xae2d0c] + id × 79: +0x27/+0x37 or +0x3b/+0x4b), or 0')])
CAMPOS = [('0xae2104 (14 bytes): as regiões em ordem de nome; [0xae232b]: idioma da ordem; escreve (regioes_pelo_nome)',
           '0xae2104 (14 bytes): regions sorted by name; [0xae232b]: sort language; writes (regioes_pelo_nome)'),
          ('registro de opinião da pessoa ([0xae2d0c] + id × 79): +0x27 índice (−1 = sem laço), +0x37 alvo, +0x3b índice, +0x4b alvo; lê (pessoa_da_opiniao)',
           'person opinion record ([0xae2d0c] + id × 79): +0x27 index (−1 = no link), +0x37 target, +0x3b index, +0x4b target; reads (pessoa_da_opiniao)')]
INTERESSA = ['SGE', 'CMRTE']
