# A busca do olheiro e os amistosos: se a busca de olheiro cobre a nação (PLAYER_SEARCH::scout_covers_nation
# 0x7b7490, 4052 chamadas na carreira até 2002-06-01, pela busca 0x7b5be0 e pelos olheiros 0x7f8750/0x7f8ff0; as
# chamadas 0x53e990, 0x7aefd0, 0x53d980 e 0x7f9190 já em Rust), tirar o amistoso marcado (Friendly::remove_friendly_match
# 0x5c87b0, 464 chamadas; as da carreira, 0x5c5f40 e 0x5c85d0, passam limpar_vaga = 0) e os pares de nações que não
# jogam amistoso (Friendly::init_banned_nation_pairs 0x5cc2d0, 1 chamada, ao criar os amistosos). A 0x5c87b0 e a
# 0x5cc2d0 têm 0x200 bytes de pilha só para a caixa "Error" (sem SEH); a cópia da 0x5c87b0 chama a original no erro, a
# da 0x5cc2d0 mostra a caixa (aviso_em, linha 0x1800) porque o malloc não pode repetir. Sem sorteio. Iguais no
# cm0102-1080.exe. Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Comparadores: a busca
# lado a lado pelo valor e pelas missões; tirar o amistoso e os pares, o plano antes e só a original (liberam/alocam).
SAVE = False
ROTINAS = [('busca_cobre_a_nacao', 0x7b7490, [0x83, 0xec, 0x0c, 0x53, 0x55]),
           ('tirar_amistoso_marcado', 0x5c87b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('iniciar_pares_proibidos_de_amistoso', 0x5cc2d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a busca do olheiro e os amistosos: se a busca cobre a nação, tirar o amistoso marcado e os pares de nações que não jogam amistoso',
        'the scout search and the friendlies: whether the search covers the nation, removing the scheduled friendly and the nation pairs that play no friendlies',
        [('busca_cobre_a_nacao', 'PLAYER_SEARCH::scout_covers_nation', '0x7b7490', 'player_search.cpp', 'thiscall, ret 4',
          'se a busca do olheiro (tipo 4, olheiro conferido, com clube) cobre a nação: pelas missões marcadas (clube, nação, região, base) ou, sem missão, pela região da nação do clube, a nação do olheiro e os olheiros do clube',
          'whether the scout search (type 4, scout checked, with a club) covers the nation: by the set assignments (club, nation, region, base) or, without one, by the region of the club\'s nation, the scout\'s nation and the club\'s scouts'),
         ('tirar_amistoso_marcado', 'Friendly::remove_friendly_match', '0x5c87b0', 'friendly.cpp', 'thiscall, ret 8',
          'tira da lista +0x34d4 os itens do jogo, zera a vaga marcada (com limpar_vaga) e tira o jogo da lista +0x34c8 (liberando-o)',
          'removes the match\'s items from the +0x34d4 list, clears the scheduled slot (with clear_linked) and removes the match from the +0x34c8 list (freeing it)'),
         ('iniciar_pares_proibidos_de_amistoso', 'Friendly::init_banned_nation_pairs', '0x5cc2d0', 'friendly.cpp', 'thiscall',
          'aloca os 4 pares de nações que não jogam amistoso entre si (Irã e EUA, Iraque e qualquer uma, Vietnã e EUA, as Coreias) em +0x3502, conta em +0x3506',
          'allocates the 4 nation pairs that play no friendlies with each other (Iran and USA, Iraq and anyone, Vietnam and USA, the Koreas) at +0x3502, count at +0x3506')])
CAMPOS = [('busca de olheiro +0xa: o tipo (4 = olheiro); valor; lê (busca_cobre_a_nacao)', 'scout search +0xa: the type (4 = scout); value; reads (busca_cobre_a_nacao)'),
          ('busca de olheiro +0x174..+0x184: os nomes e a data do olheiro; +0x180 o olheiro; id; lê (busca_cobre_a_nacao)',
           'scout search +0x174..+0x184: the scout\'s names and date; +0x180 the scout; id; reads (busca_cobre_a_nacao)'),
          ('busca de olheiro +0x18c nação, +0x190 clube, +0x194 região (byte), +0x195 competição, +0x199 base, +0x19d adversário: as missões; id, valor e ponteiro; lê (busca_cobre_a_nacao)',
           'scout search +0x18c nation, +0x190 club, +0x194 region (byte), +0x195 competition, +0x199 base, +0x19d opponent: the assignments; id, value and pointer; reads (busca_cobre_a_nacao)'),
          ('nação +0: o id; +0x76 a região; +0x7f o grupo (2); valor; lê (busca_cobre_a_nacao)', 'nation +0: the id; +0x76 the region; +0x7f the group (2); value; reads (busca_cobre_a_nacao)'),
          ('clube +0x53: a nação; ponteiro; lê (busca_cobre_a_nacao)', 'club +0x53: the nation; pointer; reads (busca_cobre_a_nacao)'),
          ('staff +0x39: o clube do olheiro; ponteiro; lê (busca_cobre_a_nacao)', 'staff +0x39: the scout\'s club; pointer; reads (busca_cobre_a_nacao)'),
          ('amistosos +0x34c8: a lista dos jogos marcados; +0x34d4 a lista dos itens que apontam para eles (+0 o jogo); ponteiro; lê e tira (tirar_amistoso_marcado)',
           'friendlies +0x34c8: the list of scheduled matches; +0x34d4 the list of items pointing at them (+0 the match); pointer; reads and removes (tirar_amistoso_marcado)'),
          ('jogo amistoso +0 dia, +2 ano, +0x10 adversário, +0x14 clube; valor e ponteiro; lê (tirar_amistoso_marcado)',
           'friendly match +0 day, +2 year, +0x10 opponent, +0x14 club; value and pointer; reads (tirar_amistoso_marcado)'),
          ('vaga do amistoso +0x10: o adversário; ponteiro; grava 0; +0x18 byte grava 0 (tirar_amistoso_marcado)',
           'friendly slot +0x10: the opponent; pointer; writes 0; +0x18 byte writes 0 (tirar_amistoso_marcado)'),
          ('amistosos +0x3502: o vetor dos pares proibidos (8 ids de nação, −1 = qualquer); ponteiro; grava; +0x3506 a conta (4); valor; grava (iniciar_pares_proibidos_de_amistoso)',
           'friendlies +0x3502: the banned pairs array (8 nation ids, −1 = any); pointer; writes; +0x3506 the count (4); value; writes (iniciar_pares_proibidos_de_amistoso)')]
INTERESSA = ['CMRTE', 'CMScout']
